use axum::extract::State;
use axum::Json;
use crate::modules::category::service::CategoryService;
use crate::state::AppState;
use crate::utils::{ ApiResponse, AppError, ValidatedJson };
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
    ValidatedJson(payload): ValidatedJson<CreateCategoryDto>
) -> Result<Json<ApiResponse<category::Model>>, AppError> {
    let category = CategoryService::new(state.db).create_category(payload).await?;

    Ok(Json(ApiResponse::success(category)))
}
