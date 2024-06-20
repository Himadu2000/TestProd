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
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Value {
    name: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum ProductOptionControl {
    SELECT,
}

#[derive(Debug, Serialize)]
pub struct ProductOption {
    name: Option<String>,
    control: ProductOptionControl,
    required: bool,
    position: u8,
    values: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub struct VariantOption {
    option_id: Thing,
    value_id: Thing,
}

#[derive(Debug, Serialize)]
pub struct Variant {
    sku: Option<String>,
    price: f32,
    stock_quantity: u16,
    weight: Option<f32>,
    options: Vec<VariantOption>,
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
