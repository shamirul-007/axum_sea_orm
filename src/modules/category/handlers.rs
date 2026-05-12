use crate::entities::category;
use crate::modules::category::dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::modules::category::service::CategoryService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError, ValidatedJson};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn get_category_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<category::Model>>, AppError> {
    let category = CategoryService::new(state.db)
        .get_category_by_id(id)
        .await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(data): ValidatedJson<UpdateCategoryDto>,
) -> Result<Json<ApiResponse<category::Model>>, AppError> {
    let category = CategoryService::new(state.db)
        .update_category(id, data)
        .await?;
    Ok(Json(ApiResponse::success(category)))
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<category::Model>>>, AppError> {
    let categories = CategoryService::new(state.db).get_categories().await?;
    Ok(Json(ApiResponse::success(categories)))
}

pub async fn create_category(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateCategoryDto>,
) -> Result<Json<ApiResponse<category::Model>>, AppError> {
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
