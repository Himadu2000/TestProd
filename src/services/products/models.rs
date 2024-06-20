use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Thing};

#[derive(Debug, Serialize)]
pub struct Image {
    alt: Option<String>,
    position: u8,
    file: Bytes,
}

#[derive(Debug, Serialize)]
pub struct Dimensions {
    length: f32,
    width: f32,
    height: f32,
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
pub struct ProductOption {
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
