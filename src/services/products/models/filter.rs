mod product;

use product::{Attribute, Dimensions, Image, ProductOption, Variant};
use swd::{
    ComplexObject, Datetime, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing,
};

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "ProductInput")]
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
    #[graphql(skip)]
    pub related_products: Vec<Thing>,
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
    #[graphql(skip)]
    pub category_ids: Vec<Thing>,
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

#[derive(Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct ProductRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub product: Product,
}

#[ComplexObject]
impl ProductRecord {
    async fn id(&self) -> String {
        format!("{}:{}", &self.id.tb, &self.id.id)
    }
}

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductFilterStockStatus {
    AVAILABLE,
}

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductFilterSort {
    PriceMinToMax,
    PriceMaxToMin,
    StockHighest,
    StockLowest,
}

#[derive(InputObject)]
#[graphql(complex)]
pub struct Filter {
    #[allow(dead_code)]
    #[graphql(skip)]
    category_id: Option<Thing>,
    active: Option<bool>,
    discontinued: Option<bool>,
    search: Option<String>,
    on_sale: Option<bool>,
    stock_status: ProductFilterStockStatus,
    price_from: Option<f32>,
    price_to: Option<f32>,
    sku: Option<String>,
    #[allow(dead_code)]
    #[graphql(skip)]
    ids: Option<Vec<Thing>>,
    sort: Option<ProductFilterSort>,
}

#[ComplexObject]
impl Filter {
    async fn id(&self) -> String {
        String::new()
    }
}