use axum::{ Json, extract::{ Path, State } };

use crate::{ modules::user::service::UserService, state::AppState };

pub async fn get_users(State(state): State<AppState>) -> Json<serde_json::Value> {
    let users = UserService::new(state.db).get_users().await.unwrap();

    Json(serde_json::json!(users))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Json<serde_json::Value> {
    let user = UserService::new(state.db).get_user(id).await;

    match user {
        Ok(Some(user)) => Json(serde_json::json!({ "success": true, "data": user })),
        Ok(None) => Json(serde_json::json!({ "success": false, "message": "User not found" })),
        Err(_) => Json(serde_json::json!({ "success": false, "message": "Internal server error" })),
    }
}
