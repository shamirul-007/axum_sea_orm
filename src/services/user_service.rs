use sea_orm::{ DatabaseConnection, EntityTrait };

use crate::entity::user;

#[derive(Clone)]
pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_users(&self) -> Result<Vec<user::Model>, sea_orm::DbErr> {
        user::Entity::find().all(&self.db).await
    }

    pub async fn get_user(
        &self,
        id: i32
    ) -> Result<std::option::Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }
}
