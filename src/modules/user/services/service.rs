use sea_orm::{ ActiveModelTrait, DatabaseConnection, EntityTrait, Set };
use uuid::Uuid;

use crate::{ entities::user, utils::AppError };
use crate::modules::user::dto::request::create_user_dto::CreateUserDto;
use crate::modules::user::dto::request::update_user_dto::UpdateUserDto;

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

    pub async fn get_user(&self, id: Uuid) -> Result<user::Model, AppError> {
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

    pub async fn update_user(
        &self,
        id: Uuid,
        data: UpdateUserDto
    ) -> Result<user::Model, AppError> {
        let user = self.get_user(id).await?;

        let mut user_active: user::ActiveModel = user.into();

        if let Some(value) = data.email {
            user_active.email = sea_orm::Set(value);
        }

        if let Some(value) = data.name {
            user_active.name = sea_orm::Set(value);
        }

        Ok(user_active.update(&self.db).await?)
    }
}
