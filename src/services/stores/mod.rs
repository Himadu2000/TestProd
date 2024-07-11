mod models;

use crate::util::{auth::is_authorized, error};
use models::{Store, StoreRecord};
use swd::{
    async_graphql::Context,
    surrealdb::{engine::remote::http::Client, Surreal},
    Object,
};

#[derive(Default)]
pub struct StoresQuery;

#[derive(Default)]
pub struct StoresMutation;

#[Object]
impl StoresQuery {
    async fn stores<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&str, &str> {
        let _db = ctx.data::<Surreal<Client>>().map_err(error)?;

        Ok("Server Is Running OK...!")
    }
}

#[Object]
impl StoresMutation {
    async fn create_store<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: Store,
    ) -> Result<Vec<StoreRecord>, &str> {
        is_authorized(ctx, String::new()).await?;

        let db = ctx.data::<Surreal<Client>>().map_err(error)?;

        let stores: Vec<StoreRecord> = db.create("store").content(data).await.unwrap();

        Ok(stores)
    }
}
