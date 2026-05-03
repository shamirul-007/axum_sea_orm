use axum::{routing::get, Router};

use crate::state::AppState;
use super::handler;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route("/users/:id", get(handler::find_by_id))
}
