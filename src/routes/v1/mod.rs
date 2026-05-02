use axum::Router;

use crate::{ modules::user, state::AppState };

pub fn routes() -> Router<AppState> {
    Router::new().merge(user::routes::user_routes())
}
