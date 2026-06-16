use axum::{
    extract::{Path, State},
    Json,
};

use crate::modules::user::dto::request::create_user_dto::CreateUserDto;
use crate::modules::user::dto::request::update_user_dto::UpdateUserDto;
use crate::{
    state::AppState,
    utils::{ApiResponse, AppError, ValidatedJson},
};

use crate::entities::user::Model as UserModel;
use crate::modules::user::services::service::UserService;

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserModel>>>, AppError> {
    let users = UserService::new(state.db).get_users().await?;

    Ok(Json(ApiResponse::success(users)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<axum::Json<ApiResponse<UserModel>>, AppError> {
    let user = UserService::new(state.db).get_user(id).await?;

    Ok(Json(ApiResponse::success(user)))
}

pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserDto>,
) -> Result<Json<ApiResponse<UserModel>>, AppError> {
    let user = UserService::new(state.db).create_user(payload).await?;

    Ok(Json(ApiResponse::success(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateUserDto>,
) -> Result<axum::Json<ApiResponse<UserModel>>, AppError> {
    let user = UserService::new(state.db).update_user(id, payload).await?;
    Ok(Json(ApiResponse::success(user)))
}
