mod v1;

use axum::{ Router };

use crate::{ routes::v1::routes_v1, state::AppState };

pub fn create_routes() -> Router<AppState> {
    Router::new().nest("/api/v1", routes_v1())
}
