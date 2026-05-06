use axum::{ extract::{ Path, State }, Json };

use crate::{
    modules::user::{ self, dto::{ CreateUserDto, UpdateUserDto }, service::UserService },
    state::AppState,
    utils::{ ApiResponse, AppError, ValidatedJson },
};

pub async fn get_users(State(state): State<AppState>) -> Result<
    Json<ApiResponse<Vec<user::model::Model>>>,
    AppError
> {
    let users = UserService::new(state.db).get_users().await?;

    Ok(Json(ApiResponse::success(users)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<axum::Json<ApiResponse<user::model::Model>>, AppError> {
    let user = UserService::new(state.db).get_user(id).await?;

    Ok(Json(ApiResponse::success(user)))
}

pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserDto>
) -> Result<Json<ApiResponse<crate::modules::user::model::Model>>, AppError> {
    let user = UserService::new(state.db).create_user(payload).await?;

    Ok(Json(ApiResponse::success(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateUserDto>
) -> Result<axum::Json<ApiResponse<user::model::Model>>, AppError> {
    let user = UserService::new(state.db).update_user(id, payload).await?;
    Ok(Json(ApiResponse::success(user)))
}
