mod models;

use models::Product;
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct ProductsQuery;

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn status<'ctx>(&self, ctx: &Context<'ctx>) -> &str {
        "Server Is Running OK...!"
    }
}

#[Object]
impl ProductsMutation {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
