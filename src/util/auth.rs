mod models;

use models::Product;
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct AuthQuery;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthQuery {
    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        let db = ctx.data::<Surreal<Db>>();
        "Server Is Running OK...!"
    }
}

#[Object]
impl AuthMutation {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
