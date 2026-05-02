mod db;
mod state;
mod routes;
mod controllers;
mod services;
mod entity;
mod utils;

use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use crate::{ routes::{ create_routes }, utils::init_logger };

#[tokio::main]
async fn main() {
    init_logger();

    dotenv().ok();
    tracing::info!("env files loaded");

    let db = db::connect_to_db().await;
    tracing::info!("Databased connected");

    let state = state::AppState {
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
