use crate::modules::category::dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::modules::category::handlers::update_category;
use crate::modules::category::model as category;
use crate::modules::category::model::Model;
use crate::utils::AppError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter, QueryOrder,
};
use uuid::Uuid;

pub struct CategoryService {
    pub db: DatabaseConnection,
}

impl CategoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_category_by_id(&self, id: Uuid) -> Result<category::Model, AppError> {
        let category = category::Entity::find_by_id(id)
            .filter(category::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        match category {
            Some(c) => Ok(c),
            None => Err(AppError::NotFound(format!(
                "Category not found with id: {}",
                id
            ))),
        }
    }

    pub async fn update_category(
        &self,
        id: Uuid,
        data: UpdateCategoryDto,
    ) -> Result<category::Model, AppError> {
        let category = self.get_category_by_id(id).await?;

        let mut updated_category: category::ActiveModel = category.into();

        if let Some(v) = data.name {
            let slug = v.to_lowercase().replace(" ", "-");
            updated_category.slug = Set(slug);
            updated_category.name = Set(v);
        }

        if let Some(v) = data.description {
            updated_category.description = Set(Some(v));
        }

        if let Some(v) = data.image {
            updated_category.image = Set(v)
        }

        Ok(updated_category.update(&self.db).await?)
    }

    pub async fn get_categories(&self) -> Result<Vec<category::Model>, AppError> {
        let categories = category::Entity::find()
            .filter(category::Column::DeletedAt.is_null())
            .order_by_desc(category::Column::CreatedAt)
            .all(&self.db)
            .await?;
        Ok(categories)
    }

    pub async fn get_category_by_slug(&self, slug: String) -> Result<Option<Model>, AppError> {
        let category = category::Entity::find()
            .filter(category::Column::Slug.eq(slug))
            .one(&self.db)
            .await?;

        Ok(category)
    }

    pub async fn delete_category_by_id(&self, id: Uuid) -> Result<(), AppError> {
        let category = self.get_category_by_id(id).await?;

        let mut deleted_category: category::ActiveModel = category.into();

        let now = chrono::Utc::now().naive_utc();

        deleted_category.deleted_at = Set(Some(now));
        deleted_category.updated_at = Set(now);
        deleted_category.update(&self.db).await?;

        Ok(())
    }

    pub async fn create_category(
        &self,
        data: CreateCategoryDto,
    ) -> Result<category::Model, AppError> {
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
