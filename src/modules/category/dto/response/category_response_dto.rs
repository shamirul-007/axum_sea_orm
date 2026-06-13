use crate::entities::category;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponseDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

impl CategoryResponseDto {
    pub fn new(category: category::Model) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            image: category.image,
            created_at: category.created_at,
            updated_at: category.updated_at,
            deleted_at: category.deleted_at,
        }
    }
}
