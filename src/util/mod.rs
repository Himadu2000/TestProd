pub mod auth;
pub mod files;
pub mod graphql;
pub mod store;

use super::graphql::Headers;
use swd::async_graphql::Error;
use swd::{async_graphql::Context, Object};

pub async fn db_and_headers<'ctx>(ctx: &Context<'ctx>, _scope: String) -> Result<(), &'static str> {
    let token = ctx
        .insert_http_header("Authorization", "")
        .ok_or("Authorization header not set...!")?;

    let token = token
        .to_str()
        .map_err(|_| "Incorrect Authorization header...!")?;

    let headers = ctx.data::<Headers>().unwrap();

    if headers.authorization == "Bearer token03124701209" {
        return Ok(());
    }

    Err("Not authorized...!")
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
