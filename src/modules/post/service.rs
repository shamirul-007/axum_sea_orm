use sea_orm::{ DatabaseConnection, EntityTrait };

use crate::modules::post::model;

#[derive(Clone)]
pub struct PostService {
    pub db: DatabaseConnection,
}

impl PostService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_posts(&self) -> Result<Vec<model::Model>, sea_orm::DbErr> {
        model::Entity::find().all(&self.db).await
    }

    pub async fn get_post(
        &self,
        id: i32
    ) -> Result<std::option::Option<model::Model>, sea_orm::DbErr> {
        model::Entity::find_by_id(id).one(&self.db).await
    }
}
