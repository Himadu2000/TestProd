use swd::{async_graphql::MergedObject, Object};

#[derive(Default)]
struct QueryRoot;

#[Object]
impl ProductsQuery {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}

#[Object]
impl ProductsQuery {
    async fn status(&self) -> &str {
        "Server Is Running OK...!"
    }
}
