use crate::modules::user::handler::handler::{create_user, find_by_id, get_users, update_user};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(find_by_id).put(update_user))
}
