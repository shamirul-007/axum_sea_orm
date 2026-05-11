pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260502_090744_create_users;
mod m20260504_153304_create_category_table;
mod m20260511_153546_create_product_table;
mod m20260511_160219_update_category_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260502_090744_create_users::Migration),
            Box::new(m20260504_153304_create_category_table::Migration),
            Box::new(m20260511_153546_create_product_table::Migration),
            Box::new(m20260511_160219_update_category_table::Migration),
        ]
    }
}
