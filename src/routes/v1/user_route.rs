use axum::{ Json, Router, extract::{ Path, State }, routing::get };

use crate::{ controllers::UserController, services::UserService, state::AppState };

#[derive(Clone)]
pub struct UserRoute;

impl UserRoute {
    async fn get_users(State(state): State<AppState>) -> Json<serde_json::Value> {
        let service = UserService::new(state.db.clone());
        let controller = UserController::new(service);

        Json(controller.get_users().await)
    }

    async fn find_by_id(
        State(state): State<AppState>,
        Path(id): Path<i32>
    ) -> Json<serde_json::Value> {
        let service = UserService::new(state.db.clone());
        let controller = UserController::new(service);

        match controller.get_user(id).await {
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

    pub fn create_user_routes() -> Router<AppState> {
        Router::new()
            .route("/users", get(Self::get_users))
            .route("/users/:id", get(Self::find_by_id))
    }
}
