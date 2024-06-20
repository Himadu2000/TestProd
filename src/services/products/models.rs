use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Thing};

#[derive(Debug, Serialize)]
pub struct Product {
    first: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
