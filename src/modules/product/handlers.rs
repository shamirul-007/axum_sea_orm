use crate::entities::product;
use crate::modules::product::service::ProductService;
use crate::state::AppState;
use crate::utils::{ApiResponse, AppError};
use axum::Json;
use axum::extract::State;

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<axum::Json<ApiResponse<Vec<product::Model>>>, AppError> {
    let products = ProductService::new(state.db).get_products().await?;

    Ok(Json(ApiResponse::success(products)))
}
