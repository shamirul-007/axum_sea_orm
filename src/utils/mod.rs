mod logger;
pub mod response;
pub mod validation;
pub mod password;
pub mod jwt;

pub use logger::*;
pub use response::{ ApiResponse, AppError };
use uuid::Uuid;
pub use validation::ValidatedJson;
use validator::ValidationError;

pub fn validate_uuid(id: &Uuid) -> Result<(), ValidationError> {
    if *id == Uuid::nil() {
        return Err(ValidationError::new("invalid_uuid"));
    }

    Ok(())
}
