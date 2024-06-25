// mod orders;
mod products;
mod stores;
mod tokens;

use crate::util::auth::AuthMutation;
use products::{ProductsMutation, ProductsQuery};
use swd::{async_graphql::MergedObject, Object};

#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}

#[derive(MergedObject, Default)]
pub struct Query(QueryRoot, ProductsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(QueryRoot, AuthMutation, ProductsMutation);
