use swd::{async_graphql::MergedObject, Object};

#[derive(Default)]
pub struct ProductsQuery;

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}

#[Object]
impl ProductsMutation {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
