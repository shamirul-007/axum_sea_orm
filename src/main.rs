mod db;
mod state;
mod routes;
mod controllers;
mod services;
mod entity;

use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use crate::routes::UserRoute;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::connect_to_db().await;

    let state = state::AppState {
        db,
    };

    let app = Router::new().merge(UserRoute::create_user_routes()).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap()
}
