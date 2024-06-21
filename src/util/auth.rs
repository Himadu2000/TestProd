mod models;

use models::Product;
use surrealdb::{engine::local::Db, Surreal};
use swd::{async_graphql::Context, Object};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, #[graphql(validator(email))] email: String) -> Result<String> {
        "Server Is Running OK...!"
    }
}
