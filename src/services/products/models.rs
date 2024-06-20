use serde::{Deserialize, Serialize};
use surrealdb::sql::{Bytes, Datetime};
use swd::async_graphql::{ComplexObject, Enum, InputObject, SimpleObject};

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex)]
pub struct Image {
    pub alt: Option<String>,
    pub position: u8,
    #[graphql(skip)]
    pub file: Bytes,
}

#[ComplexObject]
impl Image {
    async fn file(&self) -> String {
        String::new()
    }
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct Dimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct Value {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductOptionControl {
    #[default]
    SELECT,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct ProductOption {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub position: u8,
    pub values: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct VariantOption {
    pub option_id: String,
    pub value_id: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
pub struct Variant {
    pub sku: Option<String>,
    pub price: f32,
    pub stock_quantity: u16,
    pub weight: Option<f32>,
    pub options: Vec<VariantOption>,
}

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex)]
pub struct Product {
    #[graphql(default)]
    pub images: Vec<Image>,
    pub dimensions: Option<Dimensions>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
    #[graphql(default)]
    pub tags: Vec<String>,
    #[graphql(default)]
    pub attributes: Vec<Attribute>,
    #[graphql(default)]
    pub enabled: bool,
    #[graphql(default)]
    pub discontinued: bool,
    pub slug: Option<String>,
    pub sku: Option<String>,
    pub code: Option<String>,
    pub tax_class: Option<String>,
    pub related_products: Vec<String>,
    #[graphql(default)]
    pub prices: Vec<f32>,
    pub cost_price: Option<f32>,
    #[graphql(default)]
    pub regular_price: f32,
    pub sale_price: Option<f32>,
    #[graphql(default)]
    pub quantity_inc: u16,
    #[graphql(default)]
    pub quantity_min: u16,
    pub weight: Option<f32>,
    #[graphql(default)]
    pub stock_quantity: u16,
    #[graphql(default)]
    pub position: u8,
    #[graphql(skip)]
    pub date_stock_expected: Datetime,
    #[graphql(skip)]
    pub date_sale_from: Datetime,
    #[graphql(skip)]
    pub date_sale_to: Datetime,
    #[graphql(default)]
    pub stock_tracking: bool,
    #[graphql(default)]
    pub stock_preorder: bool,
    #[graphql(default)]
    pub stock_backorder: bool,
    pub category_ids: Vec<String>,
    #[graphql(default)]
    pub options: Vec<ProductOption>,
    #[graphql(default)]
    pub variants: Vec<Variant>,
}

#[ComplexObject]
impl Product {
    async fn date_stock_expected(&self) -> String {
        String::new()
    }

    async fn date_sale_from(&self) -> String {
        String::new()
    }

    async fn date_sale_to(&self) -> String {
        String::new()
    }
}

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ProductRecord {
    #[allow(dead_code)]
    pub id: String,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub product: Product,
}
