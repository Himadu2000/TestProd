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
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
pub struct Value {
    name: String,
}

#[derive(Debug, Serialize)]
pub enum ProductOptionControl {
    SELECT,
}

#[derive(Debug, Serialize)]
pub struct ProductOption {
    name: String,
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

#[derive(Debug, Default, Serialize)]
pub struct Product {
    pub images: Vec<Image>,
    pub dimensions: Option<Dimensions>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
    pub tags: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub enabled: bool,
    pub discontinued: bool,
    pub slug: Option<String>,
    pub sku: Option<String>,
    pub code: Option<String>,
    pub tax_class: Option<String>,
    pub related_products: Vec<Thing>,
    pub prices: Vec<f32>,
    pub cost_price: Option<f32>,
    pub regular_price: f32,
    pub sale_price: Option<f32>,
    pub quantity_inc: u16,
    pub quantity_min: u16,
    pub weight: Option<f32>,
    pub stock_quantity: u16,
    pub position: u8,
    pub date_stock_expected: Datetime,
    pub date_sale_from: Datetime,
    pub date_sale_to: Datetime,
    pub stock_tracking: bool,
    pub stock_preorder: bool,
    pub stock_backorder: bool,
    pub category_ids: Vec<Thing>,
    pub options: Vec<ProductOption>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
