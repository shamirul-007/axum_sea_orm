use sea_orm::{DatabaseConnection, EntityTrait};
use crate::modules::category::{model as category};
use crate::utils::AppError;

pub struct CategoryService {
    pub db: DatabaseConnection
}

impl CategoryService {
    
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }
    pub async fn get_categories(&self) -> Result<Vec<category::Model>,AppError>
    {
       let categories = category::Entity::find().all(&self.db).await?;
        Ok(categories)
    }
}