use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add timestamp columns
        manager
            .alter_table(
                Table::alter()
                    .table(ProductImage::Table)
                    .add_column(
                        ColumnDef::new(ProductImage::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column(
                        ColumnDef::new(ProductImage::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column(ColumnDef::new(ProductImage::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        // Recreate FK with CASCADE rules
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-product-image")
                    .from(ProductImage::Table, ProductImage::ProductId)
                    .to(Product::Table, Product::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProductImage::Table)
                    .drop_column(ProductImage::CreatedAt)
                    .drop_column(ProductImage::UpdatedAt)
                    .drop_column(ProductImage::DeletedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ProductImage {
    Table,
    ProductId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
