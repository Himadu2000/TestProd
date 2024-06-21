mod models;

use models::Product;
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
