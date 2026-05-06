use axum::extract::State;
use axum::Json;
use validator::Validate;
use crate::modules::category::service::CategoryService;
use crate::state::AppState;
use crate::utils::{ ApiResponse, AppError };
use crate::modules::category::{ model as category };
use crate::modules::category::dto::CreateCategoryDto;

pub async fn get_categories(State(state): State<AppState>) -> Result<
    Json<ApiResponse<Vec<category::Model>>>,
    AppError
> {
    let categories = CategoryService::new(state.db).get_categories().await?;
    Ok(Json(ApiResponse::success(categories)))
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>
) -> Result<Json<ApiResponse<category::Model>>, AppError> {
    payload.validate()?;
    let category = CategoryService::new(state.db).create_category(payload).await?;

    Ok(Json(ApiResponse::success(category)))
}
