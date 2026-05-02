use axum::{ Router, routing::get };

use crate::{ modules::post::handler::{ get_post, get_posts }, state::AppState };

pub fn post_routes() -> Router<AppState> {
    Router::new().route("/posts", get(get_posts)).route("/posts/:id", get(get_post))
}
