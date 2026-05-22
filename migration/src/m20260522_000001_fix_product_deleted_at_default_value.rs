use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Product::Table)
                .modify_column(
                    ColumnDef::new(Product::DeletedAt)
                        .timestamp()
                        .null()
                        .default(Expr::cust("NULL"))
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Product::Table)
                .modify_column(
                    ColumnDef::new(Product::DeletedAt)
                        .timestamp()
                        .null()
                        .default(Expr::current_timestamp())
                )
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    DeletedAt,
}
