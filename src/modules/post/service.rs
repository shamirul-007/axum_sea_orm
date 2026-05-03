use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    modules::post::model,
    utils::AppError,
};
use super::dto::CreatePostDto;

#[derive(Clone)]
pub struct PostService {
    pub db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_posts(&self) -> Result<Vec<model::Model>, AppError> {
        let posts = model::Entity::find().all(&self.db).await?;
        Ok(posts)
    }

    pub async fn get_post(&self, id: i32) -> Result<model::Model, AppError> {
        let post = model::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;
        Ok(post)
    }

    pub async fn create_post(&self, data: CreatePostDto) -> Result<model::Model, AppError> {
        let post = model::ActiveModel {
            title: Set(data.title),
            text: Set(data.text),
            ..Default::default()
        };

        let post = post.insert(&self.db).await?;
        Ok(post)
    }
}
