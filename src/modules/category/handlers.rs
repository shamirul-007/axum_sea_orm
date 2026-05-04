use axum::extract::State;
use axum::Json;
use crate::modules::category::service::CategoryService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError};
use crate::modules::category::{model as category};

pub async fn get_categories(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<category::Model>>>, AppError> {
    let categories = CategoryService::new(state.db).get_categories().await?;
    Ok(Json(ApiResponse::success(categories)))
}