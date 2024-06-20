use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Thing};

#[derive(Debug, Serialize)]
pub struct Image {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Dimensions {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Value {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Option {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct VariantOption {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Variant {
    first: String,
}

#[derive(Debug, Serialize)]
pub struct Product {
    first: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
