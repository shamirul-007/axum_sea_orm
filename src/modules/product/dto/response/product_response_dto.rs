use crate::modules::product::dto::response::category_response_dto::CategoryResponseDto;
use crate::modules::product::dto::response::product_feature_response_dto::ProductFeatureResponseDto;
use crate::modules::product::dto::response::product_image_response_dto::ProductImageResponseDto;
use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::Serialize;
use uuid::Uuid;

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
