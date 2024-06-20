use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Thing};

#[derive(Debug, Serialize)]
pub struct Product<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
