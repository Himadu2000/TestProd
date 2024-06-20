mod models;

use models::Product;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct ProductsQuery;

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }

    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }
}

#[Object]
impl ProductsMutation {
    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }

    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }

    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }
}
