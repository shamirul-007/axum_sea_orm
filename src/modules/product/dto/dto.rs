use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::prelude::Decimal;
use crate::utils::validate_uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
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
    #[validate(custom(function = "validate_decimal"))]
    pub price: Decimal,

    #[serde(default)]
    #[validate(custom(function = "validate_decimal"))]
    pub compared_at_price: Decimal,

    #[serde(default)]
    #[validate(range(min = 1, message = "review count is required"))]
    pub review_count: i32,

    #[serde(default)]
    #[validate(range(min = 1, max = 5, message = "rating is required"))]
    pub rating: i32,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "sku is required"))]
    pub sku: String,

    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "tagline is required"))]
    pub tagline: String,

    #[serde(default)]
    #[validate(range(min = 1, message = "stock is required"))]
    pub stock: i32,

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

fn validate_decimal(price: &Decimal) -> Result<(), ValidationError> {

    if *price <= Decimal::ZERO {
        return Err(ValidationError::new("invalid_price"));
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct ProductResponseDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub category: CategoryResponseDto,
    pub description: String,
    pub long_description: String,
    pub price: Decimal,
    pub compared_at_price: Decimal,
    pub review_count: i32,
    pub rating: i32,
    pub sku: String,
    pub tagline: String,
    pub stock: i32,
    pub is_new: bool,
    pub is_featured: bool,
    pub is_best_seller: bool,
    pub product_images: Vec<ProductImageResponseDto>,
    pub product_features: Vec<ProductFeatureResponseDto>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ProductImageResponseDto {
    pub id: Uuid,
    pub image_url: String,
}

#[derive(Debug, Serialize)]
pub struct ProductFeatureResponseDto {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponseDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image: String,
}
