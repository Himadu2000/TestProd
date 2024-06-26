mod models;

use crate::util::{auth::is_authorized, error, graphql::Headers};
use models::{Store, StoreRecord};
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object, SurrealDb};

#[derive(Default)]
pub struct StoresQuery;

#[derive(Default)]
pub struct StoresMutation;

#[Object]
impl StoresQuery {
    async fn stores<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        // let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
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

        let db = ctx.data::<SurrealDb>().map_err(error)?;

        let stores: Vec<StoreRecord> = db.create("store").content(data).await.unwrap();

        Ok(stores)
    }
}
