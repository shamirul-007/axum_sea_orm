use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::modules::category::{model as category};
use crate::modules::category::dto::CreateCategoryDto;
use crate::modules::category::model::Model;
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

    pub async fn get_category_by_slug(&self, slug: String) -> Result<Option<Model>, AppError> {
        let category = category::Entity::find().filter(category::Column::Slug.eq(slug)).one(&self.db).await?;

        Ok(category)
    }

    pub async fn create_category(&self, data: CreateCategoryDto) -> Result<category::Model, AppError> {
        let slug = data.name.to_lowercase().replace(" ", "-");

        if let Ok(Some(_)) = self.get_category_by_slug(slug.clone()).await {
            return Err(AppError::BadRequest("category already exists".into()));
        }
        
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