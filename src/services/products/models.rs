use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Datetime, Thing};
use swd::async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct Image {
    pub alt: Option<String>,
    pub position: u8,
    #[graphql(skip)]
    pub file: Bytes,
}

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct Dimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct Value {
    pub name: String,
}

#[derive(Debug, Default, Serialize)]
pub enum ProductOptionControl {
    #[default]
    SELECT,
}

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct ProductOption {
    pub name: String,
    #[graphql(skip)]
    pub control: ProductOptionControl,
    pub required: bool,
    pub position: u8,
    pub values: Vec<Value>,
}

#[derive(Debug, Serialize)]
pub struct VariantOption {
    pub option_id: Thing,
    pub value_id: Thing,
}

#[derive(Debug, Serialize, SimpleObject, InputObject)]
pub struct Variant {
    pub sku: Option<String>,
    pub price: f32,
    pub stock_quantity: u16,
    pub weight: Option<f32>,
    #[graphql(skip)]
    pub options: Vec<VariantOption>,
}

#[derive(Debug, Default, Serialize, SimpleObject, InputObject)]
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
    #[graphql(skip)]
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
    #[graphql(skip)]
    pub date_stock_expected: Datetime,
    #[graphql(skip)]
    pub date_sale_from: Datetime,
    #[graphql(skip)]
    pub date_sale_to: Datetime,
    pub stock_tracking: bool,
    pub stock_preorder: bool,
    pub stock_backorder: bool,
    #[graphql(skip)]
    pub category_ids: Vec<Thing>,
    pub options: Vec<ProductOption>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}
