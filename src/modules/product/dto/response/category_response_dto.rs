use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CategoryResponseDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image: String,
}
