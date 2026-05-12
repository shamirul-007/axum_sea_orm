use crate::utils::validate_uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductDto {
    #[serde(default)]
    #[validate(length(min = 1, max = 100, message = "product_id is required"))]
    pub name: String,

    #[serde(default)]
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Uuid,

    #[serde(default)]
    #[validate(length(min = 1, max = 200, message = "description is required"))]
    pub description: String,

    #[serde(default)]
    #[validate(length(min = 1, max = 500, message = "long description is required"))]
    pub long_description: String,

    #[serde(default)]
    #[validate(range(min = 1, message = "price is required"))]
    pub price: i32,

    #[serde(default)]
    #[validate(range(min = 1, message = "compared price is required"))]
    pub compared_at_price: i32,

    #[serde(default)]
    #[validate(range(min = 1, message = "review count is required"))]
    pub review_count: i32,

    #[serde(default)]
    #[validate(range(min = 1, max = 5, message = "rating is required"))]
    pub rating: i8,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "sku is required"))]
    pub sku: String,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "tagline is required"))]
    pub tagline: String,

    #[serde(default)]
    #[validate(range(min = 1, message = "stock is required"))]
    pub stock: i8,

    #[serde(default)]
    pub is_featured: bool,

    #[serde(default)]
    pub is_new: bool,

    #[serde(default)]
    pub is_best_seller: bool,

    #[validate(length(min = 1, message = "At least one feature is required"))]
    #[validate(nested)]
    pub product_features: Vec<CreateProductFeatureDto>,

    #[validate(length(min = 1, message = "At least one image is required"))]
    #[validate(nested)]
    pub product_images: Vec<CreateProductImageDto>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductFeatureDto {
    #[validate(length(min = 1, message = "Feature name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductImageDto {
    #[validate(url(message = "Invalid image URL"))]
    pub image_url: String,
}
