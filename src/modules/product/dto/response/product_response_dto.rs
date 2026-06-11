use crate::entities::{category, product};
use crate::modules::product::dto::response::category_response_dto::CategoryResponseDto;
use crate::modules::product::dto::response::product_feature_response_dto::ProductFeatureResponseDto;
use crate::modules::product::dto::response::product_image_response_dto::ProductImageResponseDto;
use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProductResponseDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub category: Option<CategoryResponseDto>,
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

impl ProductResponseDto {
    pub fn new(
        p: product::Model,
        category: category::Model,
        images: Vec<ProductImageResponseDto>,
        features: Vec<ProductFeatureResponseDto>,
    ) -> Self {
        Self {
            id: p.id,
            name: p.name,
            slug: p.slug,
            category: Some(CategoryResponseDto {
                id: category.id,
                name: category.name,
                slug: category.slug,
                description: category.description,
                image: category.image,
            }),
            description: p.description,
            long_description: p.long_description,
            price: p.price,
            compared_at_price: p.compared_at_price,
            review_count: p.review_count,
            rating: p.rating,
            sku: p.sku,
            tagline: p.tagline,
            stock: p.stock,
            is_new: p.is_new.unwrap_or(false),
            is_featured: p.is_featured.unwrap_or(false),
            is_best_seller: p.is_best_seller.unwrap_or(false),
            product_images: images,
            product_features: features,
            created_at: p.created_at,
            updated_at: p.updated_at,
            deleted_at: p.deleted_at,
        }
    }
}
