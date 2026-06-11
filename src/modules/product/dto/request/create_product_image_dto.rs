use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductImageDto {
    #[validate(url(message = "Invalid image URL"))]
    pub image_url: String,
}