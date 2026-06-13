use crate::entities::category;
use crate::modules::category::dto::request::create_category_dto::CreateCategoryDto;
use crate::modules::category::dto::request::update_category_dto::UpdateCategoryDto;
use crate::modules::category::dto::response::category_response_dto::CategoryResponseDto;
use crate::utils::AppError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, QueryFilter,
    QueryOrder, TransactionTrait,
};
use uuid::Uuid;

pub struct CategoryService {
    pub db: DatabaseConnection,
}

impl CategoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_category_by_id(&self, id: Uuid) -> Result<CategoryResponseDto, AppError> {
        let trx = self.db.begin().await?;
        let category = category::Entity::find_by_id(id).one(&trx).await?;

        match category {
            Some(c) => {
                trx.commit().await?;
                Ok(CategoryResponseDto::new(c))
            }
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
    ) -> Result<CategoryResponseDto, AppError> {
        let trx = self.db.begin().await?;
        let category = category::Entity::find_by_id(id).one(&trx).await?;

        let category = match category {
            Some(c) => c,
            None => {
                return Err(AppError::NotFound(format!(
                    "Category not found with id: {}",
                    id
                )));
            }
        };

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
            updated_category.image = Set(v);
        }

        let category = updated_category.update(&trx).await?;
        trx.commit().await?;
        Ok(CategoryResponseDto::new(category))
    }

    pub async fn get_categories(&self) -> Result<Vec<CategoryResponseDto>, AppError> {
        let trx = self.db.begin().await?;
        let categories = category::Entity::find()
            .order_by_desc(category::Column::CreatedAt)
            .all(&trx)
            .await?;

        let categories: Vec<CategoryResponseDto> = categories
            .into_iter()
            .map(CategoryResponseDto::new)
            .collect();

        trx.commit().await?;

        Ok(categories)
    }

    pub async fn get_category_by_slug(
        &self,
        slug: String,
    ) -> Result<Option<CategoryResponseDto>, AppError> {
        let trx = self.db.begin().await?;

        let category = category::Entity::find()
            .filter(category::Column::Slug.eq(&slug))
            .one(&trx)
            .await?;

        match category {
            Some(c) => {
                trx.commit().await?;
                Ok(Some(CategoryResponseDto::new(c)))
            }
            None => Err(AppError::NotFound(format!(
                "Category not found with slug: {}",
                slug
            ))),
        }
    }

    pub async fn delete_category_by_id(&self, id: Uuid) -> Result<(), AppError> {
        let trx = self.db.begin().await?;
        category::Entity::delete_by_id(id).exec(&trx).await?;
        trx.commit().await?;
        Ok(())
    }

    pub async fn create_category(
        &self,
        data: CreateCategoryDto,
    ) -> Result<CategoryResponseDto, AppError> {
        let trx = self.db.begin().await?;
        let slug = data.name.to_lowercase().replace(" ", "-");

        if let Ok(Some(_)) = self.get_category_by_slug(slug.clone()).await {
            return Err(AppError::BadRequest("category already exists".into()));
        }

        let now = chrono::Utc::now().naive_utc();

        let category = category::ActiveModel {
            name: Set(data.name),
            id: Set(Uuid::new_v4()),
            image: Set(data.image),
            description: data.description.map(|v| Set(Some(v))).unwrap_or(NotSet),
            slug: Set(slug),
            created_at: Set(now),
            ..Default::default()
        };

        let category = CategoryResponseDto::new(category.insert(&trx).await?);
        trx.commit().await?;
        Ok(category)
    }
}
