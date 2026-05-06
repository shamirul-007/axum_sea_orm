use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::utils::ApiResponse;

pub struct ValidatedJson<T>(pub T);

fn parse_serde_error(error: &str) -> (String, Value) {
    // Extract field name and error type from serde error message
    let error_lower = error.to_lowercase();

    if error_lower.contains("missing field") {
        // Extract field name: "missing field `image`"
        if let Some(start) = error.find('`') {
            if let Some(end) = error.rfind('`') {
                if start < end {
                    let field = &error[start + 1..end];
                    return (
                        format!("The '{}' field is required", field),
                        json!({ field: "This field is required" }),
                    );
                }
            }
        }
        (
            "Missing required field".to_string(),
            json!({ "general": "Missing required field" }),
        )
    } else if error_lower.contains("invalid type") || error_lower.contains("invalid value") {
        // "invalid type: null, expected a string at line 1 column 16"
        if let Some(start) = error.find('`') {
            if let Some(end) = error.rfind('`') {
                if start < end {
                    let field = &error[start + 1..end];
                    return (
                        format!("The '{}' field has an invalid value", field),
                        json!({ field: "Invalid value provided" }),
                    );
                }
            }
        }
        (
            "Invalid field value".to_string(),
            json!({ "general": "Invalid field value" }),
        )
    } else if error_lower.contains("unknown field") {
        // "unknown field"
        if let Some(start) = error.find('`') {
            if let Some(end) = error.rfind('`') {
                if start < end {
                    let field = &error[start + 1..end];
                    return (
                        format!("Unknown field '{}'", field),
                        json!({ field: "This field is not recognized" }),
                    );
                }
            }
        }
        (
            "Unknown field provided".to_string(),
            json!({ "general": "Unknown field provided" }),
        )
    } else {
        (
            "Invalid request payload".to_string(),
            json!({ "general": "Invalid request body" }),
        )
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(data)) => Ok(Self(data)),
            Err(err) => {
                let error_str = err.to_string();
                let (message, errors) = parse_serde_error(&error_str);

                let response = ApiResponse::<()> {
                    success: false,
                    message: Some(message),
                    data: None,
                    errors: Some(errors),
                };
                Err((StatusCode::BAD_REQUEST, Json(response)).into_response())
            }
        }
    }
}
