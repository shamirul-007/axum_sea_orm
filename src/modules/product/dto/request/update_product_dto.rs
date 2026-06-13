use crate::modules::product::dto::request::create_product_feature_dto::CreateProductFeatureDto;
use crate::modules::product::dto::request::create_product_image_dto::CreateProductImageDto;
use crate::modules::product::utils::validate_decimal;
use crate::utils::validate_uuid;
use sea_orm::prelude::Decimal;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProductDto {
    #[serde(default)]
    #[validate(length(min = 1, max = 100, message = "product name is required"))]
    pub name: Option<String>,

    #[serde(default)]
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<Uuid>,

    #[serde(default)]
    #[validate(length(min = 1, max = 200, message = "description is required"))]
    pub description: Option<String>,

    #[serde(default)]
    #[validate(length(min = 1, max = 500, message = "long description is required"))]
    pub long_description: Option<String>,

    #[serde(default)]
    #[validate(custom(function = "validate_decimal"))]
    pub price: Option<Decimal>,

    #[serde(default)]
    #[validate(custom(function = "validate_decimal"))]
    pub compared_at_price: Option<Decimal>,

    #[serde(default)]
    #[validate(range(min = 1, message = "review count is required"))]
    pub review_count: Option<i32>,

    #[serde(default)]
    #[validate(range(min = 1, max = 5, message = "rating is required"))]
    pub rating: Option<i32>,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "sku is required"))]
    pub sku: Option<String>,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "tagline is required"))]
    pub tagline: Option<String>,

    #[serde(default)]
    #[validate(range(min = 1, message = "stock is required"))]
    pub stock: Option<i32>,

    #[serde(default)]
    pub is_featured: Option<bool>,

    #[serde(default)]
    pub is_new: Option<bool>,

    #[serde(default)]
    pub is_best_seller: Option<bool>,

    #[serde(default)]
    pub product_features: Option<Vec<CreateProductFeatureDto>>,

    #[serde(default)]
    pub product_images: Option<Vec<CreateProductImageDto>>,
}
