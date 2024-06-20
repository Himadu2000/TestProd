mod models;

use models::Product;
use surrealdb::{engine::local::Db, Surreal};
use swd::{
    async_graphql::{types::connection::*, Context, Error},
    Object,
};

#[derive(Default)]
pub struct ProductsQuery;

#[derive(Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn get_product<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Result<Product, &str> {
        let db = ctx.data::<Surreal<Db>>();
        Err("Server Is Running OK...!")
    }

    async fn get_products<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, i32, EmptyFields, EmptyFields>, Error> {
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(10000);
                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }
                let mut connection = Connection::new(start > 0, end < 10000);
                connection.edges.extend(
                    (start..end)
                        .into_iter()
                        .map(|n| Edge::with_additional_fields(n, n as i32, EmptyFields)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[Object]
impl ProductsMutation {
    async fn create_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        data: Product,
    ) -> Result<String, &str> {
        let db = ctx.data::<Surreal<Db>>();
        Err("Server Is Running OK...!")
    }

    async fn update_product<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
        data: Product,
    ) -> Result<Product, &str> {
        let db = ctx.data::<Surreal<Db>>();
        Err("Server Is Running OK...!")
    }

    async fn delete_product<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Result<bool, &str> {
        let db = ctx.data::<Surreal<Db>>();
        Err("Server Is Running OK...!")
    }
}
