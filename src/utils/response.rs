use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
            errors: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            data: None,
            errors: None,
        }
    }

    pub fn validation_error(errors: ValidationErrors) -> Self {
        Self {
            success: false,
            message: Some("Validation failed".to_string()),
            data: None,
            errors: Some(serde_json::to_value(errors).unwrap_or_default()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Validation failed")]
    ValidationError(#[from] ValidationErrors),
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, ApiResponse::<()>::error(msg)),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiResponse::<()>::error(msg),
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, ApiResponse::<()>::error(msg)),
            AppError::ValidationError(errs) => {
                (StatusCode::BAD_REQUEST, ApiResponse::<()>::validation_error(errs))
            }
            AppError::DbError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiResponse::<()>::error(err.to_string()),
            ),
        };

        (status, Json(body)).into_response()
    }
}
