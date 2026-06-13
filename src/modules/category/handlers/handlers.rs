use crate::entities::category;
use crate::modules::category::services::service::CategoryService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError, ValidatedJson};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;
use crate::modules::category::dto::request::create_category_dto::CreateCategoryDto;
use crate::modules::category::dto::request::update_category_dto::UpdateCategoryDto;
use crate::modules::category::dto::response::category_response_dto::CategoryResponseDto;

pub async fn get_category_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CategoryResponseDto>>, AppError> {
    let category = CategoryService::new(state.db)
        .get_category_by_id(id)
        .await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(data): ValidatedJson<UpdateCategoryDto>,
) -> Result<Json<ApiResponse<CategoryResponseDto>>, AppError> {
    let category = CategoryService::new(state.db)
        .update_category(id, data)
        .await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CategoryResponseDto>>>, AppError> {
    let categories = CategoryService::new(state.db).get_categories().await?;
    Ok(Json(ApiResponse::success(categories)))
}

pub async fn create_category(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateCategoryDto>,
) -> Result<Json<ApiResponse<CategoryResponseDto>>, AppError> {
    let category = CategoryService::new(state.db)
        .create_category(payload)
        .await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    CategoryService::new(state.db)
        .delete_category_by_id(id)
        .await?;
    Ok(Json(ApiResponse::success(
        "Category deleted successfully".to_string(),
    )))
}
