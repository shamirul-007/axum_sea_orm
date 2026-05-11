use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("category", "created_at").await? {
            return Ok(());
        }

        let stmt = Table::alter()
            .table(Category::Table)
            .add_column(
                ColumnDef::new(Category::CreatedAt)
                    .timestamp()
                    .not_null()
                    .default(Expr::current_timestamp())
            )
            .to_owned();

        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Category::Table)
                .drop_column(Category::CreatedAt)
                .drop_column(Category::UpdatedAt)
                .drop_column(Category::DeletedAt)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
