use sea_orm_migration::{ prelude::*, schema::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Product::Table)
                .if_not_exists()
                .col(ColumnDef::new(Product::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Product::Name).string().not_null())
                .col(ColumnDef::new(Product::Slug).string().not_null())
                .col(ColumnDef::new(Product::CategoryId).uuid().not_null())
                .col(ColumnDef::new(Product::Description).string().not_null())
                .col(ColumnDef::new(Product::LongDescription).text().not_null())
                .col(ColumnDef::new(Product::Price).decimal().not_null())
                .col(ColumnDef::new(Product::ComparedAtPrice).decimal().not_null())
                .col(ColumnDef::new(Product::ReviewCount).integer().not_null())
                .col(ColumnDef::new(Product::Rating).integer().not_null())
                .col(ColumnDef::new(Product::Sku).string().not_null())
                .col(ColumnDef::new(Product::Tagline).string().not_null())
                .col(ColumnDef::new(Product::Stock).integer().not_null())
                .col(ColumnDef::new(Product::IsNew).boolean().default(false))
                .col(ColumnDef::new(Product::IsFeatured).boolean().default(false))
                .col(ColumnDef::new(Product::IsBestSeller).boolean().default(false))
                .col(
                    ColumnDef::new(Product::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                )
                .col(
                    ColumnDef::new(Product::UpdatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp())
                )
                .col(
                    ColumnDef::new(Product::DeletedAt)
                        .timestamp()
                        .default(Expr::current_timestamp())
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("category-product-fk")
                        .from(Product::Table, Product::CategoryId)
                        .to(Category::Table, Category::Id)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Product::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    CategoryId,
    Name,
    Slug,
    Tagline,
    Description,
    LongDescription,
    Price,
    ComparedAtPrice,
    Rating,
    ReviewCount,
    Stock,
    Sku,
    IsNew,
    IsFeatured,
    IsBestSeller,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}
