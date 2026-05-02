use axum::{ Json, Router, extract::State, routing::get };

use crate::{ controllers::UserController, services::UserService, state::AppState };

#[derive(Clone)]
pub struct UserRoute;

impl UserRoute {
    async fn get_users(State(state): State<AppState>) -> Json<serde_json::Value> {
        let service = UserService::new(state.db.clone());
        let controller = UserController::new(service);

        Json(controller.get_users().await)
    }

    pub fn create_user_routes() -> Router<AppState> {
        Router::new().route("/", get(Self::get_users))
    }
}
