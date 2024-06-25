pub mod auth;
pub mod files;
pub mod graphql;
pub mod store;

use graphql::Headers;
use swd::{
    async_graphql::{Context, Error},
    SurrealDb, Thing,
};

pub fn db_and_store_id<'ctx>(
    ctx: &Context<'ctx>,
) -> Result<(&'ctx SurrealDb, Thing), &'static str> {
    let db = ctx.data::<SurrealDb>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    let store_id = headers.store_id.clone();
    let (tb, id) = store_id.split_once(':').unwrap_or_default();
    let store_id = Thing {
        tb: tb.to_owned(),
        id: id.into(),
    };

    Ok((db, store_id))
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
