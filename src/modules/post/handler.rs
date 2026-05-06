use axum::{ extract::{ Path, State }, Json };
use validator::Validate;

use crate::{
    modules::post::{ dto::CreatePostDto, service::PostService },
    state::AppState,
    utils::{ ApiResponse, AppError },
};

pub async fn get_posts(State(state): State<AppState>) -> Result<
    Json<ApiResponse<Vec<crate::modules::post::model::Model>>>,
    AppError
> {
    let posts = PostService::new(state.db).get_posts().await?;

    Ok(Json(ApiResponse::success(posts)))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<Json<ApiResponse<crate::modules::post::model::Model>>, AppError> {
    let post = PostService::new(state.db).get_post(id).await?;

    Ok(Json(ApiResponse::success(post)))
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostDto>
) -> Result<Json<ApiResponse<crate::modules::post::model::Model>>, AppError> {
    payload.validate()?;

    let post = PostService::new(state.db).create_post(payload).await?;

    Ok(Json(ApiResponse::success(post)))
}
