use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePostDto {
    #[serde(default)]
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[serde(default)]
    #[validate(length(min = 1, message = "Text is required"))]
    pub text: String,
}
