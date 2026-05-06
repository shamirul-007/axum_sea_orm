use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::response::AppError;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| match rejection {
                JsonRejection::JsonDataError(err) => AppError::BadRequest(err.to_string()),
                JsonRejection::JsonSyntaxError(err) => AppError::BadRequest(err.to_string()),
                _ => AppError::BadRequest("Invalid JSON".to_string()),
            })?;

        value.validate().map_err(AppError::ValidationError)?;
        Ok(ValidatedJson(value))
    }
}
