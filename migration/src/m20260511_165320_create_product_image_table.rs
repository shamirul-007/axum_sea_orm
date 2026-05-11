use crate::extension::mysql::IndexHintType::Force;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductImage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductImage::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductImage::ProductId).uuid().not_null())
                    .col(ColumnDef::new(ProductImage::ImageUrl).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-image")
                            .from(ProductImage::Table, ProductImage::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductImage::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProductImage {
    Table,
    Id,
    ProductId,
    ImageUrl,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
