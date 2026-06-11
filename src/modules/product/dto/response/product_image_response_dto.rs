use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProductImageResponseDto {
    pub id: Uuid,
    pub image_url: String,
}