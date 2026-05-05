use sea_orm_migration::{ prelude::*, schema::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Category::Table)
                .if_not_exists()
                .col(ColumnDef::new(Category::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Category::Slug).not_null().unique_key().string())
                .col(ColumnDef::new(Category::Name).string().not_null())
                .col(ColumnDef::new(Category::description).text().null())
                .col(ColumnDef::new(Category::Image).string().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Category::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Slug,
    Name,
    Image,
    description,
}
