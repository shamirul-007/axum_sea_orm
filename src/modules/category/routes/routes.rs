use crate::modules::category::handlers::handlers::{
    create_category, delete_category, get_categories, get_category_by_id, update_category,
};
use crate::state::AppState;
use axum::Router;
use axum::routing::get;

pub fn category_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/:id",
            get(get_category_by_id)
                .patch(update_category)
                .delete(delete_category),
        )
}
