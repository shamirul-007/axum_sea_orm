use crate::entities::product_feature;
use crate::entities::product_image;
use crate::entities::{category, product};

use crate::modules::product::dto::request::create_product_dto::CreateProductDto;
use crate::modules::product::dto::request::update_product_dto::UpdateProductDto;
use crate::modules::product::dto::response::product_feature_response_dto::ProductFeatureResponseDto;
use crate::modules::product::dto::response::product_image_response_dto::ProductImageResponseDto;
use crate::modules::product::dto::response::product_response_dto::ProductResponseDto;
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
        }

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
                .push(ProductImageResponseDto::new(img.id, img.image_url));
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
                .push(ProductFeatureResponseDto::new(feat.product_id, feat.name));
        }

        let res = products
            .into_iter()
            .map(|p| {
                let category = category_map.get(&p.category_id).unwrap().clone();

                let product_images = image_map.remove(&p.id).unwrap_or_default();
                let product_features = feature_map.remove(&p.id).unwrap_or_default();

                ProductResponseDto::new(p, category, product_images, product_features)
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
            None => {
                return Ok(None);
            }
        };

        let category = match category::Entity::find_by_id(product.category_id)
            .one(&self.db)
            .await?
        {
            Some(c) => c,
            None => {
                return Ok(None);
            }
        };

        let images = product_image::Entity::find()
            .filter(product_image::Column::ProductId.eq(product.id))
            .all(&self.db)
            .await?;

        let product_image = images
            .into_iter()
            .map(|p| ProductImageResponseDto::new(p.product_id, p.image_url))
            .collect();

        let feature = product_feature::Entity::find()
            .filter(product_feature::Column::ProductId.eq(product.id))
            .all(&self.db)
            .await?;

        let product_features = feature
            .into_iter()
            .map(|p| ProductFeatureResponseDto::new(p.id, p.name))
            .collect();

        Ok(Some(ProductResponseDto::new(
            product,
            category,
            product_image,
            product_features,
        )))
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

    pub async fn update_product(
        &self,
        id: Uuid,
        product_dto: UpdateProductDto,
    ) -> Result<Option<ProductResponseDto>, AppError> {
        let trx = self.db.begin().await?;

        let product = match product::Entity::find_by_id(id).one(&trx).await? {
            Some(product) => product,
            None => {
                return Ok(None);
            }
        };

        if let Some(category_id) = product_dto.category_id {
            if category::Entity::find_by_id(category_id)
                .one(&trx)
                .await?
                .is_none()
            {
                return Err(AppError::NotFound(format!(
                    "Category not found with provided id: {}",
                    category_id
                )));
            }
        }

        let mut updated_product: product::ActiveModel = product.into();

        if let Some(name) = product_dto.name {
            let slug = name.to_lowercase().replace(' ', "-");
            updated_product.name = Set(name);
            updated_product.slug = Set(slug);
        }

        if let Some(category_id) = product_dto.category_id {
            updated_product.category_id = Set(category_id);
        }

        if let Some(description) = product_dto.description {
            updated_product.description = Set(description);
        }

        if let Some(long_description) = product_dto.long_description {
            updated_product.long_description = Set(long_description);
        }

        if let Some(price) = product_dto.price {
            updated_product.price = Set(price);
        }

        if let Some(compared_at_price) = product_dto.compared_at_price {
            updated_product.compared_at_price = Set(compared_at_price);
        }

        if let Some(review_count) = product_dto.review_count {
            updated_product.review_count = Set(review_count);
        }

        if let Some(rating) = product_dto.rating {
            updated_product.rating = Set(rating);
        }

        if let Some(sku) = product_dto.sku {
            updated_product.sku = Set(sku);
        }

        if let Some(tagline) = product_dto.tagline {
            updated_product.tagline = Set(tagline);
        }

        if let Some(stock) = product_dto.stock {
            updated_product.stock = Set(stock);
        }

        if let Some(is_new) = product_dto.is_new {
            updated_product.is_new = Set(Some(is_new));
        }

        if let Some(is_featured) = product_dto.is_featured {
            updated_product.is_featured = Set(Some(is_featured));
        }

        if let Some(is_best_seller) = product_dto.is_best_seller {
            updated_product.is_best_seller = Set(Some(is_best_seller));
        }

        let product = updated_product.update(&trx).await?;

        if let Some(product_images) = product_dto.product_images {
            product_image::Entity::delete_many()
                .filter(product_image::Column::ProductId.eq(product.id))
                .exec(&trx)
                .await?;

            if !product_images.is_empty() {
                let product_images: Vec<product_image::ActiveModel> = product_images
                    .into_iter()
                    .map(|image| product_image::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        product_id: Set(product.id),
                        image_url: Set(image.image_url),
                        ..Default::default()
                    })
                    .collect();

                product_image::Entity::insert_many(product_images)
                    .exec(&trx)
                    .await?;
            }
        }

        if let Some(product_features) = product_dto.product_features {
            product_feature::Entity::delete_many()
                .filter(product_feature::Column::ProductId.eq(product.id))
                .exec(&trx)
                .await?;

            if !product_features.is_empty() {
                let product_features: Vec<product_feature::ActiveModel> = product_features
                    .into_iter()
                    .map(|feature| product_feature::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        product_id: Set(product.id),
                        name: Set(feature.name),
                        ..Default::default()
                    })
                    .collect();

                product_feature::Entity::insert_many(product_features)
                    .exec(&trx)
                    .await?;
            }
        }

        trx.commit().await?;

        Ok(self.get_product_by_id(product.id).await?)
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<String, AppError> {
        let trx = self.db.begin().await?;

        product_image::Entity::delete_many()
            .filter(product_image::Column::ProductId.eq(id))
            .exec(&trx)
            .await?;
        product_feature::Entity::delete_many()
            .filter(product_feature::Column::ProductId.eq(id))
            .exec(&trx)
            .await?;

        let product = product::Entity::delete_by_id(id).exec(&trx).await?;

        if product.rows_affected == 0 {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        trx.commit().await?;
        Ok("Product deleted successfully".to_owned())
    }
}
