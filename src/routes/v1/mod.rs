use axum::Router;

use crate::{
    modules::{category, post::routes::post_routes, user},
    state::AppState,
};
use crate::modules::product::routes::routes::product_routes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::user_routes())
        .nest("/posts", post_routes())
        .nest("/categories", category::routes::category_routes())
        .nest("/products", product_routes())
}
