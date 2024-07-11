pub mod auth;
pub mod files;
pub mod graphql;

use async_graphql::{CustomValidator, ID};
use graphql::Headers;
use std::env::var;
use swd::{
    async_graphql::{validators::regex, Context, Error, InputValueError},
    surrealdb::{engine::remote::http::Client, Surreal},
    Deserialize,
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

pub fn is_store_id_valid(store_id: &String) -> Result<(), &'static str> {
    if store_id.len() == 0 {
        return Err("store_id header not set...!");
    }

    if IdValidator::new().check(&ID::from(store_id)).is_ok() {
        return Ok(());
    }

    Err("Invalid store_id...!")
}

pub async fn db_and_store_id<'ctx>(
    ctx: &Context<'ctx>,
) -> Result<(&'ctx Surreal<Client>, String), &'static str> {
    let db = ctx.data::<Surreal<Client>>().map_err(error)?;
    let headers = ctx.data::<Headers>().map_err(error)?;

    let store_id = headers.store_id.clone();

    is_store_id_valid(&store_id)?;

    #[derive(Deserialize)]
    struct Name {
        #[allow(dead_code)]
        name: String,
    }

    let id: Option<Name> = db
        .query(format!("SELECT name FROM store:{store_id};",))
        .await
        .unwrap()
        .take(0)
        .unwrap();

    if let None = id {
        return Err("Invalid store_id...!");
    }

    Ok((db, store_id))
}

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}

pub struct DbInfo {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub ns: String,
    pub db: String,
}

pub fn get_db() -> DbInfo {
    let db = var("DATABASE_URL").expect("DATABASE_URL not found...!");

    let error = "Invalid connection string...!";

    let (_, db) = db.split_once("://").expect(error);
    let (user, db) = db.split_once(':').expect(error);
    let (pass, db) = db.split_once('@').expect(error);
    let (url, db) = db.split_once('/').expect(error);
    let (ns, db) = db.split_once('/').expect(error);

    DbInfo {
        url: url.to_owned(),
        user: user.to_owned(),
        pass: pass.to_owned(),
        ns: ns.to_owned(),
        db: db.to_owned(),
    }
}
