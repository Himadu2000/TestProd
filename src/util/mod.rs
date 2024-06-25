pub mod auth;
pub mod files;
pub mod graphql;
pub mod store;

use graphql::Headers;
use swd::{async_graphql::Context, async_graphql::Error, SurrealDb};

pub async fn db_and_headers<'ctx>(ctx: &Context<'ctx>, _scope: String) -> Result<(), &'static str> {
    let db = ctx.data::<SurrealDb>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    (db, headers)
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
