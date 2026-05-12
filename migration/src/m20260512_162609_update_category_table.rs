use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .table(Category::Table)
            .add_column(
                ColumnDef::new(Category::UpdatedAt)
                    .timestamp()
                    .not_null()
                    .default(Expr::current_timestamp())
            )
            .add_column(ColumnDef::new(Category::DeletedAt).timestamp().null())
            .to_owned();

        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Category::Table)
                .drop_column(Category::UpdatedAt)
                .drop_column(Category::DeletedAt)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    UpdatedAt,
    DeletedAt,
}
