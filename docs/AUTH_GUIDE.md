# Auth System Implementation Guide (A→Z)

> **New to auth? Start here.** This guide walks you, step by step, through adding a
> real login system to `axum_sea_orm`. It assumes you can read Rust but have *not*
> built authentication before. Every step says **what** you're doing and **why** before
> showing the code, so you're never just copy-pasting blind.
>
> Built for the existing codebase: axum 0.7, sea-orm 0.12, a separate `migration/` crate,
> the module pattern (`dto/ services/ handler/ routes/`), the `AppError` + `ApiResponse<T>`
> response envelope, the `ValidatedJson<T>` extractor, and `AppState { db }`.

---

## First, the big picture

Right now the `user` table is just `id, name, email`. There is **no password, no login,
no security at all** — anyone could pretend to be anyone. Our goal is the kind of login
system you've used a hundred times: you register, you log in, and afterwards the server
remembers who you are on each request.

We'll get there with two kinds of "tokens". A token is just a string the client sends to
prove who it is. Here are the only two ideas you really need to hold in your head:

- **Access token** — a short-lived pass (15 minutes). The client attaches it to every
  request. It's a **JWT** (explained below). The server can check it *without a database
  lookup*, which makes it fast. Because it's short-lived, if it leaks the damage window is small.
- **Refresh token** — a long-lived key (7 days) used *only* to get a fresh access token
  when the old one expires. We store it in the database so we can **revoke** it (that's how
  "log out" and "log out everywhere" work).

### Glossary (read this once, refer back as needed)

| Term | Plain-English meaning |
|------|-----------------------|
| **Hashing** | A one-way scramble of a value. You can check `does this input match the stored hash?` but you can **never** un-scramble it back to the original. This is how we store passwords without ever keeping the real password. |
| **Salt** | Random data mixed into a password before hashing so two users with the same password get different hashes. Argon2 handles this for us automatically. |
| **Argon2** | A *deliberately slow* password-hashing algorithm. Slow is good: it makes brute-force guessing painful for attackers. |
| **SHA-256** | A *fast* one-way hash. Wrong for passwords (too fast = easy to brute force), but perfect for refresh tokens because those are already long random strings. |
| **JWT** (JSON Web Token) | A token that *carries* a little signed JSON payload (here: user id + role + expiry). The server signs it with a secret. Anyone can read it, but nobody can forge it without the secret. "Stateless" = the server doesn't store it; it just verifies the signature. |
| **Claims** | The data inside a JWT (`sub` = subject/user-id, `role`, `exp` = expiry). |
| **Revoke** | Mark a token as no-longer-valid in the DB. Used for logout. |
| **Rotation** | Each time a refresh token is used, we throw it away and hand out a brand-new one. If an old one ever gets reused, we know something's wrong. |
| **Extractor** (axum) | A struct that pulls something out of the incoming request *before* your handler runs. We'll write an `AuthUser` extractor that reads the token and gives the handler the logged-in user — or rejects the request with 401. |
| **DTO** | "Data Transfer Object" — a plain struct describing exactly what comes in (request) or goes out (response) over HTTP. Keeps your API shape separate from your DB shape. |

### The whole flow, in one picture

```
                          AUTH DATA FLOW
  register ─► hash pw (argon2) ─► insert user(role=user)
  login ────► find by email ─► verify pw ─► issue:
                                 ├─ access  = JWT{sub,role,exp=15m}   (stateless, not stored)
                                 └─ refresh = random 256-bit ─► sha256 ─► store row
  request ─► Authorization: Bearer <access> ─► AuthUser extractor ─► validate JWT
  refresh ─► sha256(raw) ─► lookup row ─► check !revoked & !expired ─► ROTATE (revoke old, issue new)
  logout ──► sha256(raw) ─► mark row revoked
```

### Three design decisions (and why), so the code makes sense later

1. **Two token types** (above). Access = fast & stateless; refresh = revocable & stored.
2. **The `role` column** (`user` / `admin`) is a Rust enum stored as a plain **string** in
   the DB. (A "native Postgres enum" would be more proper, but it triggers a version mismatch
   between the app's sea-orm and the migration crate's — a string column sidesteps that
   headache with zero downside for us.)
3. **User id becomes a `Uuid`** instead of `i32`. Two reasons: it matches `product`/`category`
   (which already use UUIDs), and sequential integer ids let strangers guess `/users/1`,
   `/users/2`… A UUID is unguessable.

**One subtle but important point — why two *different* hash algorithms?**
Passwords are short and human-chosen ("low entropy"), so we need the *slow, salted* Argon2 to
resist guessing. Refresh tokens are 256 bits of pure randomness ("high entropy") — impossible
to guess — so a *fast* SHA-256 is safe, and being deterministic lets us look a token's row up
by its hash on `/refresh` and `/logout`. Using the right tool for each is the whole point.

---

## Step 0 — Dependencies

> **What:** add the crates that do the cryptography and JWT work for us.
> **Why:** never hand-roll crypto. These are the standard, audited libraries.

Add to the root `Cargo.toml` under `[dependencies]` (some may already be present — that's fine):

```toml
jsonwebtoken = "9"                       # JWT encode/decode (HS256)
argon2       = "0.5"                      # password hashing (slow + salted)
sha2         = "0.10"                     # fast hash for refresh tokens
rand         = "0.8"                      # secure random number generator
hex          = "0.4"                      # turn random bytes into a hex string
# already present and reused: uuid (v4), chrono, serde, validator, thiserror
```

Add a `[dev-dependencies]` section (used only when running `cargo test`):

```toml
[dev-dependencies]
tower = { version = "0.4", features = ["util"] }   # lets tests call routes directly (oneshot)
http-body-util = "0.1"                              # read response bodies in tests
sea-orm = { version = "0.12", features = ["mock"] } # fake database for service tests
```

> **If you hit a build error** about `argon2::password_hash::rand_core::OsRng` not resolving,
> turn on its features: `argon2 = { version = "0.5", features = ["std", "password-hash"] }`.

## Step 0b — Environment variables

> **What:** put secrets and timing config in `.env` instead of hard-coding them.
> **Why:** the JWT secret is *the* key that signs all tokens — it must never be committed to git.

Append to both `.env` (your real local values) and `.env.example` (the committed template):

```
JWT_SECRET=change-me-to-a-long-random-string-min-32-chars
ACCESS_TOKEN_TTL_SECS=900        # 15 minutes — how long an access token lives
REFRESH_TOKEN_TTL_SECS=604800    # 7 days — how long a refresh token lives
```

("TTL" = *time to live*, i.e. how long before it expires.)

## Step 0c — Install `sea-orm-cli`

> **What:** a command-line helper from the sea-orm team. **Why:** it does two jobs for us —
> runs migrations *and* auto-generates the entity structs (Step 2) by reading the live database,
> so you don't have to type them by hand. (The existing entities in this repo were made this way —
> notice the `@generated by sea-orm-codegen` comment at the top of each.)

Install it once, globally:

```bash
cargo install sea-orm-cli
sea-orm-cli --version    # confirm it's on your PATH (this repo was built with 1.1.20)
```

`sea-orm-cli` reads the `DATABASE_URL` from your `.env` automatically (the same one in
`.env.example`), so all the commands below "just work" from the project root with no extra flags.

---

## Step 1 — Migrations (the `migration/` crate)

> **What:** change the database tables. **Why:** the `user` table needs `password` and `role`
> columns and a UUID id, and we need a brand-new `refresh_tokens` table.

> ⚠️ **Important — this project has real data in `product` and `category` (but the `user` table
> is empty).** That changes our approach two ways:
>
> 1. **Do NOT run `migrate fresh`** — it drops *every* table and would delete your product and
>    category rows. We'll use `migrate up`, which only applies *new* migrations and leaves existing
>    data untouched.
> 2. **Don't edit the old `create_users` migration in place.** A migration that has already run is
>    recorded in the `seaql_migrations` table and won't run again under `migrate up`, so editing it
>    would do nothing. Instead we add a **new** migration.
>
> The one piece of luck: the `user` table is **empty** and (verified) **nothing references it** —
> no foreign keys point at it. So our new migration can simply **drop the old `user` table and
> recreate it** with the new shape. Dropping an empty table loses nothing, and it sidesteps the
> genuinely messy Postgres dance of *altering* an `i32` column into a `uuid`.

### 1a. Scaffold the two migration files with the CLI

> **What:** let `sea-orm-cli` create the empty migration files for you instead of hand-making them.
> **Why:** the CLI stamps the correct timestamp into the filename *and* auto-registers the migration
> in `migration/src/lib.rs` — two fiddly things you'd otherwise do by hand and could get wrong.

Run these two commands from the project root, **in this order** (order decides the timestamps, and
the user table must be (re)created before the token table's foreign key can point at it):

```bash
sea-orm-cli migrate generate recreate_user_table
sea-orm-cli migrate generate create_refresh_tokens
```

Each command creates a file like `migration/src/m<TIMESTAMP>_recreate_user_table.rs` with an empty
`up()`/`down()` template, and appends a `Box::new(...)` line for it to `lib.rs` automatically.

> Your real filenames will have today's timestamp (e.g. `m20260617_212200_...`), not the
> `m20260601_*` names used as examples below. That's expected — just use whatever the CLI generated.
> No CLI? Create the two files by hand instead, using any timestamp that sorts *after* the existing
> migrations, and register them yourself (Step 1d shows the `lib.rs` edit).

### 1b. Fill in the user migration (the `recreate_user_table` file)

> Open the file the CLI just generated and replace its template body with this.
> `up()` drops the old (empty) user table, then creates the new one. `down()` reverses it. Because
> the table is empty, the `drop_table` is safe. Each `.col(...)` line is one column — read it like a
> sentence: "Id is a uuid, not null, and is the primary key."

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // The old user table is empty, so dropping it loses no data.
        manager.drop_table(Table::drop().table(User::Table).if_exists().to_owned()).await?;

        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(User::Name).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::Role).string().not_null().default("user"))
                .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone()
                    .not_null().default(Expr::current_timestamp()))
                .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone()
                    .not_null().default(Expr::current_timestamp()))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum User {
    Table, Id, Name, Email, Password, Role, CreatedAt, UpdatedAt,
}
```

> `up()` runs when you migrate forward; `down()` undoes it. The `#[derive(Iden)]` enum is just
> sea-orm's type-safe way of writing column/table names instead of raw strings. Leave the original
> `m20260502_090744_create_users.rs` file as-is — it still creates the old empty table on a
> from-scratch `fresh`, and this new migration immediately upgrades it.

### 1c. Fill in the refresh-tokens migration (the `create_refresh_tokens` file)

> **What:** the table that stores refresh tokens. **Why these columns:**
> `token_hash` (never the raw token!), `expires_at` (so we can reject old ones), `revoked`
> (so logout works), and a `user_id` foreign key (so deleting a user deletes their tokens).

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(RefreshToken::Table)
                .if_not_exists()
                .col(ColumnDef::new(RefreshToken::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(RefreshToken::UserId).uuid().not_null())
                .col(ColumnDef::new(RefreshToken::TokenHash).string().not_null().unique_key())
                .col(ColumnDef::new(RefreshToken::ExpiresAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(RefreshToken::Revoked).boolean().not_null().default(false))
                .col(ColumnDef::new(RefreshToken::CreatedAt).timestamp_with_time_zone()
                    .not_null().default(Expr::current_timestamp()))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-refresh_token-user")
                        .from(RefreshToken::Table, RefreshToken::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)   // delete user → delete their tokens
                )
                .to_owned()
        ).await?;

        // An index makes "find all tokens for this user" fast.
        manager.create_index(
            Index::create().name("idx-refresh_token-user_id")
                .table(RefreshToken::Table).col(RefreshToken::UserId).to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RefreshToken::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum RefreshToken {
    Table, Id, UserId, TokenHash, ExpiresAt, Revoked, CreatedAt,
}
#[derive(Iden)]
enum User { Table, Id }   // re-declared here just so the foreign key can reference it
```

### 1d. Confirm the registration in `migration/src/lib.rs`

Because you scaffolded with `sea-orm-cli migrate generate` (Step 1a), the CLI **already** added both
of these for you — a `mod ...;` line and a `Box::new(...)` entry per migration. Just open `lib.rs`
and confirm they're present and in the right order (recreate-user **before** create-refresh-tokens):

```rust
// module declarations near the top
mod m20260601_000000_recreate_user_table;       // your timestamp will differ
mod m20260601_000001_create_refresh_tokens;

// ...inside the migrations() vec, after the existing entries:
Box::new(m20260601_000000_recreate_user_table::Migration),
Box::new(m20260601_000001_create_refresh_tokens::Migration),
```

> If you created the files by hand instead of using the CLI, add these lines yourself.

### 1e. Run the migration — `up`, NOT `fresh` (your product/category data must survive)

```bash
sea-orm-cli migrate up        # applies ONLY the two new migrations; existing data is untouched
sea-orm-cli migrate status    # confirm both new migrations show as "Applied"
```

> ⚠️ **Do not run `sea-orm-cli migrate fresh` here.** `fresh` drops *every* table and rebuilds from
> scratch — it would erase your real product and category rows. `up` only runs migrations that
> haven't run yet, so it leaves existing tables and their data alone.
>
> `migrate down` undoes the most recent migration (calls its `down()`), handy if you need to retry.
>
> Prefer not to install the CLI? The same command works through the migration crate:
> `cargo run --manifest-path migration/Cargo.toml -- up`.

---

## Step 2 — Entities (`src/entities/`)

> **What:** the Rust structs that mirror the DB tables. sea-orm reads/writes rows through these.
> **Why now:** the migration changed the schema, so the structs must change to match, or nothing compiles.

### The fast way: generate entities from the database

Because you already ran the migration in Step 1e, the database *is* the source of truth — so let
`sea-orm-cli` read it and write the structs for you instead of typing them by hand:

```bash
sea-orm-cli generate entity \
  -o src/entities \              # output folder
  --with-serde both \            # add Serialize + Deserialize derives (this repo uses serde)
  --model-extra-derives 'utoipa::ToSchema'   # OPTIONAL — drop this flag if you don't use utoipa
```

This (re)creates `src/entities/user.rs`, `refresh_token.rs`, `mod.rs`, and the relation wiring —
all matching the schema you just migrated. It's exactly how the existing entities in this repo were
made (hence the `@generated by sea-orm-codegen` headers).

> ⚠️ **The command above regenerates EVERY entity** — so it also rewrites your existing
> `product.rs`, `category.rs`, etc. (and wipes any hand-edits in them). If you only want the new
> tables, scope it with **`--tables`**:
>
> ```bash
> # Generate ONLY the new tables — leaves product.rs, category.rs, etc. untouched:
> sea-orm-cli generate entity -o src/entities --with-serde both \
>   --tables user,refresh_token --with-prelude none
> ```
>
> **One gotcha even with `--tables`:** the CLI still rewrites the *index* files to list only the
> tables it generated this run. So `mod.rs` (and `prelude.rs`, unless you pass `--with-prelude none`
> as above) would lose the `pub mod product;` / `pub mod category;` lines. Fix it by either
> `git restore src/entities/mod.rs` and then adding the two new `pub mod` lines, or just re-adding
> the missing lines by hand (Step 2d).

> ⚠️ **The CLI will NOT create `sea_orm_active_enums.rs` for us.** That file only gets generated
> when the *database* has a native enum type (a Postgres `CREATE TYPE ... AS ENUM`). Our `role`
> column is a plain `.string()` (i.e. `varchar`) — see Step 1b — so the CLI has no way to know
> `"user"`/`"admin"` are the only legal values. What you actually get is:
>
> ```rust
> pub role: String,   // <- generated output, NOT the type we want
> ```
>
> So **Step 2a below is always written by hand**, and after generating you must patch `user.rs`:
> change `pub role: String` to `pub role: Role` and add `use super::sea_orm_active_enums::Role;`.
>
> *(If you'd rather have the CLI do it for you, the alternative is a migration that creates a real
> Postgres enum type and alters the column to it. That works, but every future role value then needs
> an `ALTER TYPE` migration. We stick with `varchar` — simpler — and pay for it with this one manual
> file.)*

> ⚠️ **Generation can't know about four of our hand-tweaks.** After generating (whole-folder or
> `--tables`), re-apply these to the generated files by hand (they're shown in full in 2a–2c below):
> 1. `#[serde(skip_serializing)]` on `user::Model::password` — generation won't add it, and without
>    it the hash can serialize into responses. (`--model-extra-attributes` only adds attributes to
>    the *whole* struct, so it can't target just this field — this stays manual.)
> 2. The whole of `sea_orm_active_enums.rs` (see the warning above), including the
>    `impl Role { fn as_str(...) }` helper that our own code uses.
> 3. Swap the generated `pub role: String` in `user.rs` for `pub role: Role`.
> 4. Double-check the `Uuid` primary keys came out as `#[sea_orm(primary_key, auto_increment = false)]`.
>
> **Tip:** run `git diff src/entities/` after generating so you can see exactly what changed and
> spot anything you need to put back (or `git restore` any entity you didn't mean to touch).

### The manual way (or to apply the tweaks above)

If you'd rather write them by hand — or you're patching the generated output — here's exactly what
each file should contain.

### 2a. New `src/entities/sea_orm_active_enums.rs` — the `Role` type

> This is how we get a type-safe `Role` enum in Rust that's stored as the string `"user"` /
> `"admin"` in the DB. **Write this file yourself** — `sea-orm-cli` won't produce it (see Step 2's
> warning above).
>
> `db_type = "String(None)"` must match the column the migration actually created: Step 1b uses
> `.string()`, which is an unbounded `varchar`. If you write `String(Some(20))` here the entity and
> the real schema disagree — harmless for normal queries, but it silently drifts and would bite you
> the moment anything derives a schema from the entity.

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum Role {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "admin")]
    Admin,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self { Role::User => "user", Role::Admin => "admin" }
    }
}
```

### 2b. Rewrite `src/entities/user.rs`

> Note `#[serde(skip_serializing)]` on `password` — that tells serde "never include this field
> when turning a user into JSON." It's a safety net so the password hash can't accidentally leak
> in a response. (We'll add a *real* fix too, in Step 10.)

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::sea_orm_active_enums::Role;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]   // we generate UUIDs ourselves, not the DB
    pub id: Uuid,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[serde(skip_serializing)]   // defense-in-depth: never leak the hash
    pub password: String,
    pub role: Role,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::refresh_token::Entity")]
    RefreshToken,
}

impl Related<super::refresh_token::Entity> for Entity {
    fn to_def() -> RelationDef { Relation::RefreshToken.def() }
}

impl ActiveModelBehavior for ActiveModel {}
```

### 2c. New `src/entities/refresh_token.rs`

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "refresh_token")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(unique)]
    pub token_hash: String,
    pub expires_at: DateTimeWithTimeZone,
    pub revoked: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity",
        from = "Column::UserId", to = "super::user::Column::Id")]
    User,
}
impl Related<super::user::Entity> for Entity {
    fn to_def() -> RelationDef { Relation::User.def() }
}
impl ActiveModelBehavior for ActiveModel {}
```

### 2d. Register both in `src/entities/mod.rs`

Add these two lines alongside the others:

```rust
pub mod refresh_token;
pub mod sea_orm_active_enums;
```

---

## Step 3 — Config + AppState (`src/state.rs`)

> **What:** load the env vars into a `Config` struct and attach it to `AppState`.
> **Why:** `AppState` is the shared bag of stuff every handler can reach. The token utils need the
> JWT secret and TTLs, so they have to live somewhere handlers can get to — here.

The file today is just `AppState { db }`. Replace it with:

```rust
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct Config {
    pub jwt_secret: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            access_ttl_secs: std::env::var("ACCESS_TOKEN_TTL_SECS")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(900),
            refresh_ttl_secs: std::env::var("REFRESH_TOKEN_TTL_SECS")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(604_800),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<Config>,   // Arc = cheap to clone; shared, read-only
}
```

> Why `Arc`? `AppState` gets cloned on every request. Wrapping `Config` in an `Arc` means each
> clone just bumps a reference count instead of copying the whole config — and `Config` doesn't
> need to be `Clone` itself.

Then in `src/main.rs`, after `dotenvy::dotenv().ok();`, build it:

```rust
let state = AppState { db, config: Arc::new(Config::from_env()) };
```

---

## Step 4 — New error types (`src/utils/response.rs`)

> **What:** add `Unauthorized` (401) and `Forbidden` (403) to the existing `AppError` enum.
> **Why:** 401 = "I don't know who you are" (bad/missing token). 403 = "I know who you are, but
> you're not allowed" (e.g. a normal user hitting an admin route). They're different on purpose.

Add these two variants to the `AppError` enum (next to `NotFound`, `BadRequest`, etc.):

```rust
#[error("Unauthorized: {0}")] Unauthorized(String),   // 401
#[error("Forbidden: {0}")]    Forbidden(String),      // 403
```

And add their match arms inside `into_response`, next to the existing ones:

```rust
AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, ApiResponse::<()>::error(msg)),
AppError::Forbidden(msg)    => (StatusCode::FORBIDDEN,    ApiResponse::<()>::error(msg)),
```

---

## Step 5 — Password hashing util (`src/utils/password.rs`)

> **What:** two functions — one to hash a new password, one to check a login attempt.
> **Why two, not "compare strings":** we never store the real password, only its hash. So login
> means "hash what they typed and see if it matches" — which Argon2's `verify_password` does for us
> (it even re-reads the salt out of the stored hash). You never write `if a == b` on passwords.

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::utils::response::AppError;

/// Turn a plaintext password into a storable hash (with a random salt baked in).
pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);   // fresh random salt every time
    Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("hash error: {e}")))
}

/// Returns true if `plain` matches the stored `hash`.
pub fn verify_password(plain: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("bad hash: {e}")))?;
    Ok(Argon2::default().verify_password(plain.as_bytes(), &parsed).is_ok())
}
```

Don't forget to declare the module in `src/utils/mod.rs`: `pub mod password;` (same for the
modules in Steps 6, 7, 8).

---

## Step 6 — JWT util (`src/utils/jwt.rs`)

> **What:** create and verify access tokens. **Why it's "stateless":** the token *contains* the
> user id, role, and expiry, all signed with our secret. To check a token we just re-verify the
> signature and expiry — **no database call**. That's what makes access tokens fast.

```rust
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::response::AppError;

/// The data baked inside every access token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // "subject" = the user id (as a string)
    pub role: String,  // "user" | "admin"
    pub iat: usize,    // issued-at (unix seconds)
    pub exp: usize,    // expiry (unix seconds) — checked automatically on decode
}

pub fn create_access_token(
    user_id: Uuid, role: &str, secret: &str, ttl_secs: i64,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        iat: now as usize,
        exp: (now + ttl_secs) as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| AppError::Internal(format!("jwt encode: {e}")))
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),   // exp is validated for us
    )
    .map(|d| d.claims)
    // On ANY failure (tampered, expired, wrong secret) we return the same vague 401.
    .map_err(|_| AppError::Unauthorized("invalid or expired token".into()))
}
```

---

## Step 7 — Refresh token util (`src/utils/refresh.rs`)

> **What:** make a random refresh token and hash it. **Why we return a pair:** the client gets the
> raw token (that's what they'll send back later); the database stores only the **hash**. If our DB
> ever leaks, the stored hashes are useless to an attacker — they can't be turned back into tokens.

```rust
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Returns (raw_token_for_the_client, sha256_hash_for_the_db).
pub fn generate_refresh_token() -> (String, String) {
    let mut bytes = [0u8; 32];                 // 32 bytes = 256 bits of randomness
    rand::thread_rng().fill_bytes(&mut bytes); // cryptographically secure RNG
    let raw = hex::encode(bytes);
    let hash = hash_refresh_token(&raw);
    (raw, hash)
}

/// Deterministic: the same raw token always produces the same hash,
/// which is exactly what lets us look a token up by its hash later.
pub fn hash_refresh_token(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    hex::encode(hasher.finalize())
}
```

> **Key rule:** both functions must use the *same* encoding (`hex` here). On `/refresh` and
> `/logout` we hash the raw token the client sends and look up the row by that hash — if the
> encodings differed, the lookup would never match.

---

## Step 8 — The auth extractor (`src/utils/auth_extractor.rs`)

> **What:** `AuthUser` and `AdminUser`. **Why extractors:** in axum, if a handler takes
> `user: AuthUser` as a parameter, axum runs this code *before* the handler body. So the token is
> validated automatically and the handler only ever runs for logged-in users — you can't forget to
> check. `AdminUser` does the same plus "must be an admin," returning 403 otherwise.

```rust
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, header}};
use uuid::Uuid;
use crate::{state::AppState, utils::{jwt::verify_access_token, response::AppError}};

pub struct AuthUser { pub id: Uuid, pub role: String }

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        // 1. Read the "Authorization" header.
        let header = parts.headers.get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("missing Authorization header".into()))?;
        // 2. It must look like "Bearer <token>".
        let token = header.strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("expected Bearer token".into()))?;
        // 3. Verify the JWT signature + expiry.
        let claims = verify_access_token(token, &state.config.jwt_secret)?;
        // 4. The subject claim is the user id.
        let id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("bad subject".into()))?;
        Ok(AuthUser { id, role: claims.role })
    }
}

pub struct AdminUser(pub AuthUser);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let user = AuthUser::from_request_parts(parts, state).await?;  // reuse the logic above
        if user.role != "admin" {
            return Err(AppError::Forbidden("admin only".into()));
        }
        Ok(AdminUser(user))
    }
}
```

> **To protect any handler**, just add `user: AuthUser` (or `AdminUser`) to its parameter list.
> If the token is bad/missing, the request is rejected *before* your handler code runs.

---

## Step 9 — The auth module (`src/modules/auth/`)

> **What:** the actual `/register`, `/login`, `/refresh`, `/logout`, `/me` endpoints.
> **Why this shape:** mirror the existing `user` module exactly — `dto/request`, `dto/response`,
> `services`, `handler`, `routes`, `mod.rs` — so it feels consistent with the rest of the codebase.

### 9a. Request DTOs — what the client sends in

> These use the project's existing `Validate` derive. `validator` checks the rules (valid email,
> min length…) automatically inside `ValidatedJson`, and bad input becomes a 400 before your code runs.

```rust
// dto/request/register_dto.rs
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 3, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 chars"))]
    pub password: String,
}

// dto/request/login_dto.rs
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

// dto/request/refresh_dto.rs  — also used by /logout
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RefreshDto {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}
```

### 9b. Response DTOs — what we send back

> **The golden rule:** responses are built from DTOs, *never* the raw `user::Model`. `UserResponse`
> simply has no `password` field, so there's no way for the hash to escape.

```rust
// dto/response/user_response.rs  — NEVER contains the password
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
}
impl From<crate::entities::user::Model> for UserResponse {
    fn from(u: crate::entities::user::Model) -> Self {
        Self { id: u.id, name: u.name, email: u.email, role: u.role.as_str().to_string() }
    }
}

// dto/response/auth_response.rs  — what /login and /refresh return
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,     // always "Bearer"
    pub expires_in: i64,        // access token TTL, in seconds
    pub user: UserResponse,
}
```

### 9c. The auth service (`services/service.rs`) — the brains

> This is where the real logic lives (handlers just call into it). The service is constructed with
> the `db` and `Arc<Config>`. Below is the **recipe** for each method in plain steps — turn each
> bullet list into Rust as you go. Read the comments carefully; the security subtleties are all here.

```
register(RegisterDto) -> UserResponse
    - check if a user with this email already exists; if so -> AppError::BadRequest
    - hash the password with hash_password()
    - insert a user: id = Uuid::new_v4(), role = Role::User, created_at/updated_at = now
    - return UserResponse::from(the new user)   // no password goes back

login(LoginDto) -> AuthResponse
    - find the user by email; if none -> Unauthorized("invalid credentials")
    - verify_password(); if it returns false -> Unauthorized("invalid credentials")
        ⚠️ Use the SAME message for "no such email" and "wrong password".
           Different messages would let an attacker discover which emails are registered.
    - access_token = create_access_token(user.id, user.role.as_str(), secret, access_ttl)
    - (raw, hash) = generate_refresh_token()
    - insert a refresh_token row { id, user_id, token_hash = hash,
                                   expires_at = now + refresh_ttl, revoked = false }
    - return AuthResponse {
          access_token, refresh_token: raw,   // the client gets the RAW token, never the hash
          token_type: "Bearer", expires_in: access_ttl, user
      }

refresh(RefreshDto) -> AuthResponse        // this is token ROTATION
    - hash = hash_refresh_token(the raw token the client sent)
    - row = find a refresh_token by token_hash; if none -> Unauthorized
    - if row.revoked OR row.expires_at < now -> Unauthorized
    - load the user by row.user_id
    - mark the OLD row revoked = true (so it can never be used again)
    - issue a NEW access token AND a NEW refresh row (this is the "rotation")
    - return a fresh AuthResponse

logout(RefreshDto) -> ()
    - hash the token; find the row; set revoked = true
    - if the row doesn't exist, that's fine — just succeed (idempotent)
```

Concrete sea-orm hints so you're not stuck:

- **Find by email:** `user::Entity::find().filter(user::Column::Email.eq(&dto.email)).one(&self.db).await?`
- **Insert:** build an `ActiveModel` with `Set(...)` on each field, then `.insert(&self.db).await?`.
- **"Now":** `chrono::Utc::now()` — compare directly against the `expires_at` field.
- **Update a row:** turn the found `Model` into an `ActiveModel`, set the changed field, `.update(&self.db).await?`.
- You don't need a DB transaction for login: the access token isn't stored, so the only DB write is
  the single refresh-row insert — there's no half-finished state to worry about.

### 9d. Handlers (`handler/handler.rs`) — thin glue

> Handlers do almost nothing: take the request, call the service, wrap the result in `ApiResponse`.
> All the thinking lives in the service. Note `me` takes `user: AuthUser` — that's what protects it.

```rust
pub async fn register(State(s): State<AppState>, ValidatedJson(dto): ValidatedJson<RegisterDto>)
    -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = AuthService::new(s).register(dto).await?;
    Ok(Json(ApiResponse::success(user)))
}

pub async fn login(State(s): State<AppState>, ValidatedJson(dto): ValidatedJson<LoginDto>)
    -> Result<Json<ApiResponse<AuthResponse>>, AppError> { /* ...call service... */ }

pub async fn refresh(State(s): State<AppState>, ValidatedJson(dto): ValidatedJson<RefreshDto>)
    -> Result<Json<ApiResponse<AuthResponse>>, AppError> { /* ... */ }

pub async fn logout(State(s): State<AppState>, ValidatedJson(dto): ValidatedJson<RefreshDto>)
    -> Result<Json<ApiResponse<()>>, AppError> { /* ... */ }

pub async fn me(user: AuthUser, State(s): State<AppState>)   // <- AuthUser = login required
    -> Result<Json<ApiResponse<UserResponse>>, AppError> { /* load user by user.id */ }
```

### 9e. Routes (`routes/routes.rs`)

```rust
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me))     // protected because the handler takes AuthUser
}
```

Then wire it into `src/routes/v1/mod.rs` with `.nest("/auth", auth_routes())`, which gives you the
final URLs `/api/v1/auth/register`, `/api/v1/auth/login`, etc.

---

## Step 10 — Patch the existing `user` module (don't skip this!)

> **Why this step exists:** the `user` table now has a `password` column. The current user handlers
> return the raw `user::Model`. That means **the password hash would be serialized into the JSON
> response** — a real leak. There is no error or crash; it just silently happens. So:

1. **Stop leaking the hash.** In `src/modules/user/handler/handler.rs`, map every `user::Model`
   into a `UserResponse` (reuse the one from Step 9b) and change the return types to
   `ApiResponse<UserResponse>` / `Vec<UserResponse>`. The `#[serde(skip_serializing)]` on the entity
   is a backup, but converting to a response DTO is the real fix.
2. **Fix `create_user` / `CreateUserDto`.** It won't compile/insert now that `password` is NOT NULL.
   Pick one:
   - (a) Remove the public "create user" endpoint and treat account creation as *registration only*, or
   - (b) Keep an **admin-only** create: add `password` + `role` to `CreateUserDto`, hash the password
     in the service, and guard the route with the `AdminUser` extractor.
3. **Decide which user routes need protection.** Recommended: `GET /users`, `GET /users/:id`,
   `PUT /users/:id`, `DELETE /users/:id` require `AdminUser` (or `AuthUser` plus an ownership check —
   "you may only edit yourself").

---

## Step 11 — Tests (there are currently ZERO)

> **Why bother:** auth is exactly the kind of code where a silent mistake is a security hole. Tests
> are how you *prove* the tricky bits (rotation, the no-leak rule, identical login errors) actually work.
> Write them as you build each piece, not at the very end.

Three layers, from cheapest to most realistic:

**Unit tests** (pure functions, `#[cfg(test)]` in the same file — fast):
- `utils/password`: hash ≠ the plaintext; `verify` is true for the right password, false for the wrong one; a malformed hash returns an error.
- `utils/jwt`: encode→decode round-trips the `sub`/`role`; an expired token (use `ttl = -1`) is rejected; a tampered token is rejected; the wrong secret is rejected.
- `utils/refresh`: `hash_refresh_token` is deterministic; two `generate_refresh_token` calls produce different tokens; the raw token never equals the stored hash.

**Service tests** (sea-orm `MockDatabase` — a fake DB, no Postgres needed):
- `register`: stores a *hashed* password (assert stored ≠ plaintext); role defaults to `user`; duplicate email → BadRequest.
- `login`: wrong password → Unauthorized; unknown email → Unauthorized **with the same message**; success returns access + refresh and writes one refresh row.
- `refresh`: valid → new tokens and the old row is now revoked (rotation); a revoked row → Unauthorized; an expired row → Unauthorized; an unknown hash → Unauthorized.
- `logout`: marks the row revoked; succeeds even for an unknown token (idempotent).

**Extractor tests:**
- `AuthUser`: missing header → 401; non-Bearer header → 401; bad/expired token → 401; valid token → passes.
- `AdminUser`: a valid *non-admin* token → 403; an admin token → passes.

**Integration test** (`tower::ServiceExt::oneshot` calls your real router) — the one big end-to-end flow:
- `register → login → GET /me with the Bearer token → refresh → use the new access token → logout →
  confirm the OLD refresh token now fails`.
- Assert the `/login` response body contains **no** `password` field.

**Regression test (the most important one):**
- Hit `GET /users/:id` (and `/login`) and assert the JSON has no `password` key. This specifically
  guards against the silent leak that Step 10.1 fixes — if someone later reverts that fix, this test
  goes red instead of the hash quietly shipping to clients.

```
COVERAGE TARGET (a rough checklist, not a rule)
  utils/password ......... 3 paths  [hash, verify-ok, verify-fail]
  utils/jwt .............. 4 paths  [roundtrip, expired, tampered, wrong-secret]
  utils/refresh .......... 3 paths
  auth service ........... register(2) login(3) refresh(4) logout(2) = 11
  extractors ............. AuthUser(4) AdminUser(2) = 6
  integration flow ....... 1 end-to-end + 1 leak regression
```

Run everything with `cargo test`. (Service tests need the `mock` feature you added in Step 0.)

---

## The failure modes this design handles (and how)

> Skim this table when you're done — it's a checklist that every "what if an attacker..." case is covered.

| Where | The realistic attack / mistake | What stops it |
|-------|--------------------------------|---------------|
| login | wrong password / unknown email | Unauthorized with an **identical message** (so attackers can't learn which emails exist) |
| refresh | someone replays an old token after it rotated | the old row is `revoked = true` → Unauthorized |
| refresh | a stolen but expired token | the `expires_at < now` check → Unauthorized |
| AuthUser | missing / garbage / expired JWT | 401 from the extractor; the handler never even runs |
| AdminUser | a normal user hits an admin route | 403 |
| register | two people register the same email at once | the DB `unique` constraint on `email` rejects the second; map that `DbErr` to BadRequest |
| any response | the password hash sneaks into JSON | the `UserResponse` DTO + `#[serde(skip_serializing)]` |

⚠️ **The sneakiest one:** if you skip Step 10.1, the hash leaks with **no error at all**. That's
why the Step 11 regression test exists — it's your tripwire.

---

## Suggested build order

Do it in this order so each piece compiles before you need the next:

1. Cargo deps + `.env` + install `sea-orm-cli` (Steps 0, 0b, 0c)
2. Migrations: `sea-orm-cli migrate generate ...` to scaffold, fill them in, then `sea-orm-cli migrate up` (⚠️ not `fresh` — see Step 1)
3. Entities: generate with `sea-orm-cli generate entity`, then hand-write `sea_orm_active_enums.rs` and re-apply the 4 tweaks (Step 2)
4. Config + AppState + the `main.rs` wiring (Step 3)
5. The new `AppError` variants (Step 4)
6. Utils: password, jwt, refresh, auth_extractor (Steps 5–8)
7. Auth module: DTOs → service → handlers → routes → nest it (Step 9)
8. Patch the user module for the leak + create_user (Step 10)
9. Tests (Step 11) — ideally alongside each piece above, not all at the end

## Every file you'll touch (the map)

```
migration/src/m20260502_090744_create_users.rs           (leave as-is)
migration/src/m20260601_000000_recreate_user_table.rs    (new — drops & recreates empty user table)
migration/src/m20260601_000001_create_refresh_tokens.rs  (new)
migration/src/lib.rs                                      (register both new migrations)
src/entities/{user.rs (rewrite), refresh_token.rs (new),
              sea_orm_active_enums.rs (new — hand-written, CLI won't emit it), mod.rs (register)}
src/state.rs                                              (Config + AppState)
src/main.rs                                               (build the Config)
src/utils/{response.rs (+2 variants), password.rs, jwt.rs, refresh.rs, auth_extractor.rs, mod.rs}
src/modules/auth/**                                       (the whole new module)
src/modules/user/handler/handler.rs                       (use UserResponse, no leak)
src/modules/user/dto/request/create_user_dto.rs           (+password/+role if you keep admin-create)
src/routes/v1/mod.rs                                      (nest /auth)
.env / .env.example                                       (JWT_SECRET + the two TTLs)
Cargo.toml                                                (deps + dev-deps)
```

## What we are deliberately NOT doing (yet)

These are real features — just out of scope for this first pass. Don't let them block you:

- **Email verification / password reset** — a separate feature with its own tokens.
- **Rate limiting on `/login`** (to slow brute-force guessing) — add the `tower-governor` crate later.
- **OAuth / "Sign in with Google"** — entirely separate, out of scope.
- **Refresh-token reuse detection** (if a rotated token is replayed, kill *all* of that user's
  sessions) — nice hardening to add after the basics work.
- **A native Postgres enum for `role`** — we use a string column on purpose (see the big-picture section).

## Final manual check (do this once it all compiles)

```bash
sea-orm-cli migrate up          # apply the new migrations (NOT `fresh` — that wipes product/category data)
cargo run                       # server starts on :4000

# register
curl -s :4000/api/v1/auth/register -H 'content-type: application/json' \
  -d '{"name":"Alice","email":"a@b.com","password":"password123"}'

# login -> copy the access_token and refresh_token out of the response
curl -s :4000/api/v1/auth/login -H 'content-type: application/json' \
  -d '{"email":"a@b.com","password":"password123"}'

# call a protected route with the access token
curl -s :4000/api/v1/auth/me -H "authorization: Bearer <ACCESS>"

# refresh -> gives new tokens; the OLD refresh token must now be rejected
curl -s :4000/api/v1/auth/refresh -H 'content-type: application/json' \
  -d '{"refresh_token":"<REFRESH>"}'

cargo test
```

You're done when: no response anywhere contains a `password` field, the old refresh token is
rejected after a refresh, and a non-admin token is blocked from the admin routes.
