use axum::Router;
use axum::routing::get;
use crate::modules::category::handlers::{create_category, get_categories, get_category_by_id};
use crate::state::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(get_categories)
            .post(create_category))
        .route("/categories/:id", get(get_category_by_id))
}