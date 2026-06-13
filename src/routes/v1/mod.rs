use axum::Router;

use crate::modules::category::routes::routes::category_routes;
use crate::modules::product::routes::routes::product_routes;
use crate::modules::user::routes::user_routes;
use crate::{
    modules::{post::routes::post_routes, user},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/posts", post_routes())
        .nest("/categories", category_routes())
        .nest("/products", product_routes())
}
