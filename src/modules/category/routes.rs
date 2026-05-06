use axum::Router;
use axum::routing::get;
use crate::modules::category::handlers::{create_category, get_categories};
use crate::state::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new().route("/categories", get(get_categories).post(create_category))
}