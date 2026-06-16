use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, message = "Name is required"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}
