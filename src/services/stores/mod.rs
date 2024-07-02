mod models;

use crate::util::{auth::is_authorized, db_and_store_id};
use models::{Store, StoreRecord};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct StoresQuery;

#[derive(Default)]
pub struct StoresMutation;

#[Object]
impl StoresQuery {
    async fn stores<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&str, &str> {
        let (db, _) = db_and_store_id(ctx)?;
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

        let (db, _) = db_and_store_id(ctx)?;

        let stores: Vec<StoreRecord> = db.create("store").content(data).await.unwrap();

        Ok(stores)
    }
}
