use axum::{routing::get, Router};

use crate::{
    modules::post::handler::{create_post, get_post, get_posts},
    state::AppState,
};

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route("/:id", get(get_post))
}
