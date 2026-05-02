use axum::Router;

use crate::{ modules::{ post::routes::post_routes, user }, state::AppState };

pub fn routes() -> Router<AppState> {
    Router::new().merge(user::routes::user_routes()).merge(post_routes())
}
