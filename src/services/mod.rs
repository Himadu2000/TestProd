// mod orders;
mod products;
mod stores;
mod tokens;

use crate::util::auth::AuthMutation;
use products::{ProductsMutation, ProductsQuery};
use stores::{StoresMutation, StoresQuery};
use swd::{async_graphql::MergedObject, Object};

#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }

    async fn license(&self) -> &str {
        "This project is licensed under AGPL-3.0-or-later and the source code is available here https://github.com/Cezerin2/Cezerin2."
    }
}

#[derive(MergedObject, Default)]
pub struct Query(QueryRoot, StoresQuery, ProductsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(QueryRoot, AuthMutation, StoresMutation, ProductsMutation);
