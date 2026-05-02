use axum::{ Router, routing::{ get } };

use crate::{ handlers::get_users, state::AppState };

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/", get(get_users))
}
