use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductFeatureDto {
    #[validate(length(min = 1, message = "Feature name is required"))]
    pub name: String,
}