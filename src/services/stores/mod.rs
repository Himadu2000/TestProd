mod models;

use models::Stores;
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct StoresQuery;

#[derive(Default)]
pub struct StoresMutation;

#[Object]
impl StoresQuery {
    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }
}

#[Object]
impl StoresMutation {
    async fn create_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: Product,
    ) -> Result<Vec<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<SurrealDb>().unwrap();

        let products: Vec<ProductRecord> = db.create("product").content(data).await.unwrap();

        Ok(products)
    }

    async fn update_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        data: Product,
    ) -> Result<Option<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<SurrealDb>().unwrap();

        let product: Option<ProductRecord> = db.update(("product", id)).merge(data).await.unwrap();

        Ok(product)
    }

    async fn delete_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        delete_image_index: Option<u8>,
    ) -> Result<Option<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<SurrealDb>().unwrap();

        let product: Option<ProductRecord> = db.delete(("product", id)).await.unwrap();

        Ok(product)
    }
}
