use crate::entities::product;
use crate::utils::AppError;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct ProductService {
    db: DatabaseConnection,
}

impl ProductService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_products(&self) -> Result<Vec<product::Model>, AppError> {
        let products = product::Entity::find().all(&self.db).await?;
        Ok(products)
    }
}
