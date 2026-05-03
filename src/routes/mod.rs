mod v1;

use axum::{ Router, routing::get };

use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/", get(health_check)).nest("/api/v1", v1::routes())
}

async fn health_check() -> String {
    "everythings works fine".to_owned()
}
