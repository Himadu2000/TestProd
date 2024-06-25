mod models;

use crate::util::graphql::Headers;
use models::Stores;
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object, SurrealDb};

#[derive(Default)]
pub struct StoresQuery;

#[derive(Default)]
pub struct StoresMutation;

#[Object]
impl StoresQuery {
    async fn stores<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }
}

#[Object]
impl StoresMutation {
    async fn create_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: Stores,
    ) -> Result<Vec<ProductRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<SurrealDb>().map_err(error)?;
        let headers = ctx.data::<Headers>().map_err(error)?;

        let data = ProductDbRecord {
            store_id,
            product: data,
        };

        let products: Vec<ProductRecord> = db.create("product").content(data).await.unwrap();

        Ok(products)
    }
}
