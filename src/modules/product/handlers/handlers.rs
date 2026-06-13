use crate::modules::product::dto::request::create_product_dto::CreateProductDto;
use crate::modules::product::dto::request::update_product_dto::UpdateProductDto;
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

    match product {
        Some(product) => Ok(Json(ApiResponse::success(Some(product)))),
        None => Ok(Json(ApiResponse::error(
            "Product not found with provided id".to_owned(),
        ))),
    }
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(data): ValidatedJson<UpdateProductDto>,
) -> Result<Json<ApiResponse<Option<ProductResponseDto>>>, AppError> {
    let product = ProductService::new(state.db)
        .update_product(id, data)
        .await?;

    match product {
        Some(product) => Ok(Json(ApiResponse::success(Some(product)))),
        None => Ok(Json(ApiResponse::error(
            "Product not found with provided id".to_owned(),
        ))),
    }
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let product = ProductService::new(state.db).delete_product(id).await?;
    Ok(Json(ApiResponse::success(product)))
}
