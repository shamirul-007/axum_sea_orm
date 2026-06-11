use crate::state::AppState;
use axum::Router;
use axum::routing::get;
use crate::modules::product::handlers::handlers::{add_product, get_products};

pub fn product_routes() -> Router<AppState> {
    Router::new().route("/", get(get_products).post(add_product))
}
