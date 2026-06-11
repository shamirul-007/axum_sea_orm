use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProductImageResponseDto {
    pub id: Uuid,
    pub image_url: String,
}

impl ProductImageResponseDto {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, image_url: name }
    }
}