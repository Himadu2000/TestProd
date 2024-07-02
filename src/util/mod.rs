pub mod auth;
pub mod files;
pub mod graphql;

use graphql::Headers;
use swd::{
    async_graphql::{validators::regex, Context, Error},
    SurrealDb,
};

fn is_store_id_valid(store_id: &String) -> Result<(), &'static str> {
    if store_id.len() == 0 {
        return Ok(());
    }

    if regex(store_id, "^[a-z0-9]{20}$").is_ok() {
        return Ok(());
    }

    Err("Invalid store_id...!")
}

pub fn db_and_store_id<'ctx>(
    ctx: &Context<'ctx>,
) -> Result<(&'ctx SurrealDb, String), &'static str> {
    let db = ctx.data::<SurrealDb>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    let store_id = headers.store_id.clone();

    is_store_id_valid(&store_id)?;

    Ok((db, store_id))
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
