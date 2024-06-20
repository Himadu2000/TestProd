use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Datetime, Thing};

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
    images: Vec<Image>,
    dimensions: Dimensions,
    name: Option<String>,
    description: Option<String>,
    meta_description: Option<String>,
    meta_title: Option<String>,
    tags: Vec<String>,
    attributes: Vec<Attribute>,
    enabled: bool,
    discontinued: bool,
    slug: Option<String>,
    sku: Option<String>,
    code: Option<String>,
    tax_class: Option<String>,
    related_product_ids: Vec<Thing>,
    prices: Vec<f32>,
    cost_price: f32,
    regular_price: f32,
    sale_price: Option<f32>,
    quantity_inc: u16,
    quantity_min: u16,
    weight: Option<f32>,
    stock_quantity: u16,
    position: u8,
    date_stock_expected: Datetime,
    date_sale_from: Datetime,
    date_sale_to: Datetime,
    stock_tracking: bool,
    stock_preorder: bool,
    stock_backorder: bool,
    category_ids: Vec<Thing>,
    options: Vec<ProductOption>,
    variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
