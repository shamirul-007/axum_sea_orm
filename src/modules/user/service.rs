use sea_orm::{ ActiveModelTrait, DatabaseConnection, EntityTrait, Set };

use crate::{ modules::user::model as user, utils::AppError };
use super::dto::CreateUserDto;

#[derive(Clone)]
pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Result<Vec<user::Model>, AppError> {
        let users = user::Entity::find().all(&self.db).await?;
        Ok(users)
    }

    pub async fn get_user(&self, id: i32) -> Result<user::Model, AppError> {
        let user = user::Entity
            ::find_by_id(id)
            .one(&self.db).await?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;
        Ok(user)
    }

    pub async fn create_user(&self, data: CreateUserDto) -> Result<user::Model, AppError> {
        let user = user::ActiveModel {
            name: Set(data.name),
            email: Set(data.email),
            ..Default::default()
        };

        let user = user.insert(&self.db).await?;
        Ok(user)
    }
}
