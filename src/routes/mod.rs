mod v1;

use axum::Router;

use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1", v1::routes())
}
