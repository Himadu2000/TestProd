mod models;

use crate::util::{auth::is_authorized, db_and_store_id, error, IdValidator};
use models::{Filter, Image, Product, ProductDbRecord, ProductRecord};
use std::io::Read;
use swd::{
    async_graphql::{
        types::{connection::*, ID},
        Context, Error, Upload,
    },
    surrealdb::sql::Bytes,
    Object, SurrealDb, Thing,
};

const ERROR: &str = "Product not found...!";

#[derive(Default)]
pub struct ProductsQuery;

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn get_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(custom = "IdValidator::new()"))] id: ID,
    ) -> Result<ProductRecord, &str> {
        let (db, store_id) = db_and_store_id(ctx).await?;

        let product: Option<ProductRecord> = db
            .query(format!(
                "SELECT * FROM product:{} WHERE store_id = store:{store_id};",
                id.0
            ))
            .await
            .unwrap()
            .take(0)
            .unwrap();

        product.ok_or(ERROR)
    }

    async fn get_products<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        _filter: Option<Filter>,
    ) -> Result<Connection<ID, ProductRecord>, Error> {
        query(
            after,
            before,
            first,
            last,
            |after, before, _first, _last| async move {
                let (db, store_id) = db_and_store_id(ctx).await?;

                let products: Vec<ProductRecord> = db
                    .query(format!(
                        "SELECT * FROM product WHERE store_id = store:{store_id};"
                    ))
                    .await
                    .unwrap()
                    .take(0)
                    .unwrap();

                // let mut start = after.map(|after| after + 1).unwrap_or(0);
                // let mut end = before.unwrap_or(10000);
                // if let Some(first) = first {
                //     end = (start + first).min(end);
                // }
                // if let Some(last) = last {
                //     start = if last > end - start { end } else { end - last };
                // }

                let start = after;
                let end = before;

                // if let Some(first) = first {
                //     end = (start + first).min(end);
                // }
                // if let Some(last) = last {
                //     start = if last > end - start { end } else { end - last };
                // }

                let has_previous_page = start.map_or(false, |value| {
                    products
                        .first()
                        .map_or(false, |product| ID::from(product.id.clone()) == value)
                });
                let has_next_page = end.map_or(false, |value| {
                    products
                        .last()
                        .map_or(false, |product| ID::from(product.id.clone()) == value)
                });

                let mut connection = Connection::new(has_previous_page, has_next_page);
                connection.edges.extend(
                    (products)
                        .into_iter()
                        .map(|n| Edge::with_additional_fields(ID::from(&n.id.id), n, EmptyFields)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[Object]
impl ProductsMutation {
    async fn create_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: Product,
    ) -> Result<Vec<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let (db, store_id) = db_and_store_id(ctx).await?;

        let store_id = Thing {
            tb: "store".to_owned(),
            id: store_id.into(),
        };

        let data = ProductDbRecord {
            store_id,
            product: data,
        };

        let products: Vec<ProductRecord> = db.create("product").content(data).await.unwrap();

        Ok(products)
    }

    async fn update_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(custom = "IdValidator::new()"))] id: ID,
        data: Product,
        images: Option<Vec<Upload>>,
        delete_image_index: Option<u8>,
    ) -> Result<Option<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let (db, store_id) = db_and_store_id(ctx).await?;
        let id = id.0;

        let product: Option<ProductDbRecord> = db.select(("product", &id)).await.unwrap();

        let mut data = data;

        if let Some(images) = images {
            for image in images {
                let mut upload = image.value(ctx).unwrap();

                let mut file = Vec::new();

                upload.content.read_to_end(&mut file).unwrap();

                let ext = upload.filename.split('.').last().unwrap_or("png");

                let file = Image {
                    file: Bytes::from(file),
                    file_as_vec: vec![],
                    mime: upload.content_type.unwrap_or(format!("image/{ext}")),
                    alt: String::new(),
                };

                if let Some(value) = &product {
                    data.images = value
                        .product
                        .images
                        .iter()
                        .map(|value| Image {
                            file: Bytes::from(value.file_as_vec.clone()),
                            ..value.clone()
                        })
                        .collect();
                }

                data.images.push(file);
            }
        }

        if let Some(index) = delete_image_index {
            let file: Option<ProductRecord> = db
                .query(format!(
                    "UPDATE product:{id} SET images = array::remove(images, $index) WHERE store_id = store:{store_id};"
                )).bind(("index",index))
                .await
                .unwrap()
                .take(0)
                .unwrap();

            product.ok_or(ERROR)
        }

        match product {
            Some(value) => {
                if value.store_id.id.to_string() == store_id {
                    let product: Option<ProductRecord> =
                        db.update(("product", id)).merge(data).await.unwrap();

                    return Ok(product);
                }
                Err(ERROR)
            }
            None => Err(ERROR),
        }
    }

    async fn delete_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(custom = "IdValidator::new()"))] id: ID,
    ) -> Result<Option<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<SurrealDb>().map_err(error)?;

        let product: Option<ProductRecord> = db.delete(("product", id.0)).await.unwrap();

        Ok(product)
    }
}
