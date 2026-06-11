use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProductFeatureResponseDto {
    pub id: Uuid,
    pub name: String,
}

impl ProductFeatureResponseDto {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
