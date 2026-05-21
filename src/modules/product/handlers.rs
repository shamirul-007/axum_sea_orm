use crate::entities::product;
use crate::modules::product::dto::{CreateProductDto, ProductResponseDto};
use crate::modules::product::service::ProductService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError, ValidatedJson};
use axum::Json;
use axum::extract::State;

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<product::Model>>>, AppError> {
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
