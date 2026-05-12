use axum::Router;

use crate::modules::product::routes::product_routes;
use crate::{
    modules::{category, post::routes::post_routes, user},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::user_routes())
        .nest("/posts", post_routes())
        .nest("/categories", category::routes::category_routes())
        .nest("/products", product_routes())
}
