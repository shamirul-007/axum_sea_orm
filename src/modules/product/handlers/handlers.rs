use crate::modules::product::dto::request::create_product_dto::CreateProductDto;
use crate::modules::product::dto::response::product_response_dto::ProductResponseDto;
use crate::modules::product::services::service::ProductService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError, ValidatedJson};
use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ProductResponseDto>>>, AppError> {
    let products = ProductService::new(state.db).get_products().await?;

    Ok(Json(ApiResponse::success(products)))
}

pub async fn add_product(
    State(state): State<AppState>,
    ValidatedJson(data): ValidatedJson<CreateProductDto>,
) -> Result<Json<ApiResponse<Option<ProductResponseDto>>>, AppError> {
    let product = ProductService::new(state.db).add_product(data).await?;
    Ok(Json(ApiResponse::success(product)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Option<ProductResponseDto>>>, AppError> {
    let product = ProductService::new(state.db).get_product_by_id(id).await?;
    Ok(Json(ApiResponse::success(product)))
}
