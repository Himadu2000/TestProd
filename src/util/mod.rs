pub mod auth;
pub mod files;
pub mod graphql;

use graphql::Headers;
use swd::{
    async_graphql::{Context, Error},
    SurrealDb,
};

pub fn db_and_store_id<'ctx>(
    ctx: &Context<'ctx>,
) -> Result<(&'ctx SurrealDb, String), &'static str> {
    let db = ctx.data::<SurrealDb>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    Ok((db, headers.store_id.clone()))
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
