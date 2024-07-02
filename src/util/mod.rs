pub mod auth;
pub mod files;
pub mod graphql;

use async_graphql::{CustomValidator, ID};
use graphql::Headers;
use std::env::var;
use swd::{
    async_graphql::{validators::regex, Context, Error, InputValueError},
    surrealdb::{engine::remote::http::Client, Surreal},
};

pub struct IdValidator;

impl IdValidator {
    pub fn new() -> Self {
        Self
    }
}

impl CustomValidator<ID> for IdValidator {
    fn check(&self, value: &ID) -> Result<(), InputValueError<ID>> {
        regex(&value.0, "^[a-z0-9]{20}$").map_err(|_| "Invalid store_id...!".into())
    }
}

fn is_store_id_valid(store_id: &String) -> Result<(), &'static str> {
    if store_id.len() == 0 {
        return Ok(());
    }

    if IdValidator::new().check(&ID::from(store_id)).is_ok() {
        return Ok(());
    }

    Err("Invalid store_id...!")
}

pub fn db_and_store_id<'ctx>(
    ctx: &Context<'ctx>,
) -> Result<(&'ctx Surreal<Client>, String), &'static str> {
    let db = ctx.data::<Surreal<Client>>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    let store_id = headers.store_id.clone();

    is_store_id_valid(&store_id)?;

    Ok((db, store_id))
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}

pub struct DbInfo {
    url: String,
    user: String,
    pass: String,
    ns: String,
    db: String,
}

pub fn get_db() -> DbInfo {
    let db = var("").unwrap_or("default");

    let (url, db) = db.split_once("://").unwrap_or(("", ""));

    DbInfo {
        url,
        user,
        pass,
        ns,
        db,
    }
}
