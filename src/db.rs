use std::env;

use sea_orm::{ Database, DatabaseConnection };

pub async fn connect_to_db() -> DatabaseConnection {
    let connection = env::var("DATABASE_URL").unwrap();
    Database::connect(connection).await.expect("Database connection failed")
}
