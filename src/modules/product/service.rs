use crate::entities::product_feature;
use crate::entities::product_image;
use crate::entities::{category, product};
use crate::modules::product::dto::{
    CategoryResponseDto, CreateProductDto, ProductFeatureResponseDto, ProductImageResponseDto,
    ProductResponseDto,
};
use crate::utils::AppError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    TransactionTrait,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ProductService {
    db: DatabaseConnection,
}

impl ProductService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_products(&self) -> Result<Vec<ProductResponseDto>, AppError> {
        let products = product::Entity::find()
            .filter(product::Column::DeletedAt.is_null())
            .all(&self.db)
            .await?;

        if products.is_empty() {
            return Ok(Vec::<ProductResponseDto>::new());
        };

        let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();
        let category_ids: Vec<Uuid> = products.iter().map(|p| p.category_id).collect();

        let categories = category::Entity::find()
            .filter(category::Column::Id.is_in(category_ids))
            .all(&self.db)
            .await?;

        let category_map: HashMap<Uuid, category::Model> =
            categories.into_iter().map(|c| (c.id, c)).collect();

        let images = product_image::Entity::find()
            .filter(product_image::Column::ProductId.is_in(product_ids.clone()))
            .all(&self.db)
            .await?;

        let mut image_map: HashMap<Uuid, Vec<ProductImageResponseDto>> = HashMap::new();

        for img in images {
            image_map
                .entry(img.product_id)
                .or_default()
                .push(ProductImageResponseDto {
                    id: img.id,
                    image_url: img.image_url,
                });
        }

        let features = product_feature::Entity::find()
            .filter(product_feature::Column::ProductId.is_in(product_ids.clone()))
            .all(&self.db)
            .await?;

        let mut feature_map: HashMap<Uuid, Vec<ProductFeatureResponseDto>> = HashMap::new();

        for feat in features {
            feature_map
                .entry(feat.product_id)
                .or_default()
                .push(ProductFeatureResponseDto {
                    id: feat.id,
                    name: feat.name,
                })
        }

        let res = products
            .into_iter()
            .map(|p| {
                let category = category_map.get(&p.category_id).unwrap().clone();

                ProductResponseDto {
                    id: p.id,
                    name: p.name,
                    slug: p.slug,
                    category: CategoryResponseDto {
                        id: category.id,
                        name: category.name,
                        slug: category.slug,
                        description: category.description,
                        image: category.image,
                    },
                    description: p.description,
                    long_description: p.long_description,
                    price: p.price,
                    compared_at_price: p.compared_at_price,
                    review_count: p.review_count,
                    rating: p.rating,
                    sku: p.sku,
                    tagline: p.tagline,
                    stock: p.stock,
                    is_new: p.is_new.unwrap_or(false),
                    is_featured: p.is_featured.unwrap_or(false),
                    is_best_seller: p.is_best_seller.unwrap_or(false),
                    product_images: image_map.remove(&p.id).unwrap_or_default(),
                    product_features: feature_map.remove(&p.id).unwrap_or_default(),
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                    deleted_at: p.deleted_at,
                }
            })
            .collect();

        Ok(res)
    }

    pub async fn get_product_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<ProductResponseDto>, AppError> {
        let product = match product::Entity::find_by_id(id).one(&self.db).await? {
            Some(x) => x,
            None => return Ok(None),
        };

        let category = match category::Entity::find_by_id(product.category_id)
            .one(&self.db)
            .await?
        {
            Some(c) => c,
            None => return Ok(None),
        };

        let images = product_image::Entity::find()
            .filter(product_image::Column::ProductId.eq(product.id))
            .all(&self.db)
            .await?;

        let product_image = images
            .into_iter()
            .map(|p| ProductImageResponseDto {
                image_url: p.image_url,
                id: p.id,
            })
            .collect();

        let feature = product_feature::Entity::find()
            .filter(product_feature::Column::ProductId.eq(product.id))
            .all(&self.db)
            .await?;

        let product_features = feature
            .into_iter()
            .map(|p| ProductFeatureResponseDto {
                id: p.id,
                name: p.name,
            })
            .collect();

        let product_res = ProductResponseDto {
            id: product.id,
            slug: product.slug,
            category: CategoryResponseDto {
                id: category.id,
                name: category.name,
                slug: category.slug,
                image: category.image,
                description: category.description,
            },
            description: product.description,
            long_description: product.long_description,
            price: product.price,
            compared_at_price: product.compared_at_price,
            review_count: product.review_count,
            rating: product.rating,
            sku: product.sku,
            tagline: product.tagline,
            stock: product.stock,
            is_new: product.is_new.unwrap_or(false),
            is_featured: product.is_featured.unwrap_or(false),
            is_best_seller: product.is_best_seller.unwrap_or(false),
            product_images: product_image,
            name: product.name,
            product_features,
            created_at: product.created_at,
            updated_at: product.updated_at,
            deleted_at: product.deleted_at,
        };

        Ok(Some(product_res))
    }

    pub async fn add_product(
        &self,
        product_dto: CreateProductDto,
    ) -> Result<Option<ProductResponseDto>, AppError> {
        let trx = self.db.begin().await?;

        let slug = product_dto.name.to_lowercase().replace(" ", "-");

        let product_model = product::ActiveModel {
            id: Set(Uuid::new_v4()),
            slug: Set(slug),
            name: Set(product_dto.name),
            tagline: Set(product_dto.tagline),
            description: Set(product_dto.description),
            long_description: Set(product_dto.long_description),
            price: Set(product_dto.price),
            compared_at_price: Set(product_dto.compared_at_price),
            category_id: Set(product_dto.category_id),
            rating: Set(product_dto.rating),
            review_count: Set(product_dto.review_count),
            stock: Set(product_dto.stock),
            sku: Set(product_dto.sku),
            is_new: Set(Some(product_dto.is_new)),
            is_featured: Set(Some(product_dto.is_featured)),
            is_best_seller: Set(Some(product_dto.is_best_seller)),
            ..Default::default()
        };

        let product = product_model.insert(&trx).await?;

        let product_images: Vec<product_image::ActiveModel> = product_dto
            .product_images
            .into_iter()
            .map(|image| product_image::ActiveModel {
                id: Set(Uuid::new_v4()),
                product_id: Set(product.id),
                image_url: Set(image.image_url),
                ..Default::default()
            })
            .collect();

        if !product_images.is_empty() {
            product_image::Entity::insert_many(product_images)
                .exec(&trx)
                .await?;
        }

        let product_features: Vec<product_feature::ActiveModel> = product_dto
            .product_features
            .into_iter()
            .map(|feature| product_feature::ActiveModel {
                id: Set(Uuid::new_v4()),
                product_id: Set(product.id),
                name: Set(feature.name),
                ..Default::default()
            })
            .collect();

        if !product_features.is_empty() {
            product_feature::Entity::insert_many(product_features)
                .exec(&trx)
                .await?;
        }

        trx.commit().await?;

        let product_res = self.get_product_by_id(product.id).await?;
        Ok(product_res)
    }
}
