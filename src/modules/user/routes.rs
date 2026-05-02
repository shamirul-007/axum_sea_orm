use axum::{ Json, Router, extract::{ Path, State }, routing::get };

use crate::{
    modules::user::handler::UserController,
    modules::user::service::UserService,
    state::AppState,
};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/users", get(get_users)).route("/users/:id", get(find_by_id))
}

async fn get_users(State(state): State<AppState>) -> Json<serde_json::Value> {
    let service = UserService::new(state.db.clone());
    let controller = UserController::new(service);

    Json(controller.get_users().await)
}

async fn find_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Json<serde_json::Value> {
    let service = UserService::new(state.db.clone());
    let controller = UserController::new(service);

    let user = controller.get_user(id).await;

    match user {
        Ok(res) => Json(res),

        Err(msg) =>
            Json(
                serde_json::json!({
            "success": false,
            "message": msg
        })
            ),
    }
}
