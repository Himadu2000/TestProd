use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Datetime, Thing};
use swd::async_graphql::SimpleObject;

#[derive(Debug, Serialize, SimpleObject)]
pub struct Image {
    pub alt: Option<String>,
    pub position: u8,
    pub file: Bytes,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct Dimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct Value {
    pub name: String,
}

#[derive(Debug, Serialize, SimpleObject)]
pub enum ProductOptionControl {
    SELECT,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct ProductOption {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub position: u8,
    pub values: Vec<Value>,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct VariantOption {
    pub option_id: Thing,
    pub value_id: Thing,
}

#[derive(Debug, Serialize, SimpleObject)]
pub struct Variant {
    pub sku: Option<String>,
    pub price: f32,
    pub stock_quantity: u16,
    pub weight: Option<f32>,
    pub options: Vec<VariantOption>,
}

#[derive(Debug, Default, Serialize, SimpleObject)]
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
