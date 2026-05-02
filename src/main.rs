mod db;
mod modules;
mod state;
mod utils;

use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use crate::{state::AppState, utils::init_logger};

fn create_routes() -> Router<AppState> {
    Router::new().nest(
        "/api/v1",
        modules::user::routes::UserRoute::create_user_routes(),
    )
}

#[tokio::main]
async fn main() {
    init_logger();

    dotenv().ok();
    tracing::info!("env files loaded");

    let db = db::connect_to_db().await;
    tracing::info!("Database connected");

    let state = AppState {
        db,
    };
    tracing::info!("global state created");

    let app = Router::new()
        .merge(create_routes())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    tracing::info!("routes created");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
