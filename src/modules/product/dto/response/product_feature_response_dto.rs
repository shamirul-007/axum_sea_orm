use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProductFeatureResponseDto {
    pub id: Uuid,
    pub name: String,
}
