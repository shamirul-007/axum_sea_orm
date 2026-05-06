use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::modules::category::{model as category};
use crate::modules::category::dto::CreateCategoryDto;
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

    pub async fn create_category(&self, data: CreateCategoryDto) -> Result<category::Model, AppError> {
        let slug = data.name.to_lowercase().replace(" ", "-");
        
        let category = category::ActiveModel {
          name: Set(data.name),
            id: Set(Uuid::new_v4()),
            image: Set(data.image),
            description: data.description.map(|v| Set(Some(v))).unwrap_or(NotSet),
            slug: Set(slug),
            ..Default::default()
        };
        
        Ok(category.insert(&self.db).await?)
    }
}