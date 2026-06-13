use crate::modules::product::handlers::handlers::{
    add_product,
    get_product,
    get_products,
    update_product,
};
use crate::state::AppState;
use axum::Router;
use axum::routing::{ get, patch };

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_products).post(add_product))
        .route("/:id", get(get_product).patch(update_product))
}
