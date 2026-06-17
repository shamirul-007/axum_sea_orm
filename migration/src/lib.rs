pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260502_090744_create_users;
mod m20260504_153304_create_category_table;
mod m20260511_153546_create_product_table;
mod m20260511_160219_update_category_table;
mod m20260511_165320_create_product_image_table;
mod m20260511_170510_update_product_image_table;
mod m20260511_171859_create_product_features_table;
mod m20260512_162609_update_category_table;
mod m20260512_180000_fix_category_columns;
mod m20260521_170715_fix_product_table_deleted_at_default_value;
mod m20260522_000001_fix_product_deleted_at_default_value;
mod m20260617_154243_recreate_user_table;
mod m20260617_154403_create_refresh_tokens;

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
            Box::new(m20260511_165320_create_product_image_table::Migration),
            Box::new(m20260511_170510_update_product_image_table::Migration),
            Box::new(m20260511_171859_create_product_features_table::Migration),
            Box::new(m20260512_162609_update_category_table::Migration),
            Box::new(m20260512_180000_fix_category_columns::Migration),
            Box::new(m20260521_170715_fix_product_table_deleted_at_default_value::Migration),
            Box::new(m20260522_000001_fix_product_deleted_at_default_value::Migration),
            Box::new(m20260617_154243_recreate_user_table::Migration),
            Box::new(m20260617_154403_create_refresh_tokens::Migration),
        ]
    }
}
