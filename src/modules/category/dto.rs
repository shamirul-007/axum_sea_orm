use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateCategoryDto {
    #[serde(default)]
    #[validate(length(min = 3, max = 100, message = "Category name must be at least 3 characters long or max 100"))]
    pub name: String,

    #[serde(default)]
    #[validate(length(min = 3,message = "Category description must be at least 3 characters long or max 3000"))]
    pub image: String,

    #[serde(default)]
    #[validate(length(min = 3, max = 100, message = "Category description must be at least 3 characters long or max 100"))]
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 3, max = 100, message = "Category name must be at least 3 characters long or max 100"))]
    pub name: Option<String>,

    #[validate(length(min = 3,message = "Category description must be at least 3 characters long or max 3000"))]
    pub image: Option<String>,

    #[validate(length(min = 3, max = 100, message = "Category description must be at least 3 characters long or max 100"))]
    pub description: Option<String>
}