use axum::Router;
use axum::routing::get;
use crate::modules::category::handlers::{create_category, delete_category, get_categories, get_category_by_id, update_category};
use crate::state::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(get_categories)
            .post(create_category))
        .route("/categories/:id", get(get_category_by_id).patch(update_category).delete(delete_category))
}