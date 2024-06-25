mod models;

use models::Product;
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
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
