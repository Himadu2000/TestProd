use swd::{
    ComplexObject, Datetime, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing,
};

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderOptionControl {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "VariantInput")]
pub struct Item {
    #[graphql(skip)]
    pub id: Thing,
    #[graphql(skip)]
    pub product_id: Thing,
    #[graphql(skip)]
    pub variant_id: Thing,
    pub quantity: u16,
    pub discount_total: f32,
    pub name: String,
    pub price: f32,
    pub price_total: f32,
    pub sku: String,
    pub tax_class: String,
    pub tax_total: f32,
    pub variant_name: String,
    pub weight: f32,
}

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "VariantInput")]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "VariantInput")]
pub struct Address {
    pub full_name: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: String,
    pub phone: String,
    pub company: String,
    pub tax_number: String,
    pub coordinates: Coordinates,
    pub details: String,
}

#[ComplexObject]
impl Variant {
    async fn options(&self) -> String {
        String::new()
    }
}

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "OrderInput")]
pub struct Order {
    #[graphql(skip)]
    pub date_placed: Datetime,
    #[graphql(skip)]
    pub date_closed: Option<Datetime>,
    #[graphql(skip)]
    pub date_paid: Option<Datetime>,
    #[graphql(skip)]
    pub date_cancelled: Option<Datetime>,
    #[graphql(default)]
    pub number: u16,
    #[graphql(default)]
    pub shipping_status: String,
    #[graphql(default)]
    pub items: Vec<Item>,
    #[graphql(default)]
    pub transactions: Vec<String>,
    #[graphql(default)]
    pub discounts: Vec<String>,
    pub billing_address: Address,
    pub shipping_address: Address,
    #[graphql(default)]
    pub tax_rate: f32,
    #[graphql(default)]
    pub shipping_tax: f32,
    #[graphql(default)]
    pub shipping_discount: f32,
    #[graphql(default)]
    pub shipping_price: f32,
    #[graphql(default)]
    pub item_tax_included: bool,
    #[graphql(default)]
    pub shipping_tax_included: bool,
    #[graphql(default)]
    pub closed: bool,
    #[graphql(default)]
    pub cancelled: bool,
    #[graphql(default)]
    pub delivered: bool,
    #[graphql(default)]
    pub paid: bool,
    #[graphql(default)]
    pub hold: bool,
    #[graphql(default = true)]
    pub draft: bool,
    #[graphql(default)]
    pub first_name: String,
    #[graphql(default)]
    pub last_name: String,
    pub password: Option<String>,
    #[graphql(default)]
    pub email: String,
    #[graphql(default)]
    pub mobile: u16,
    #[graphql(default)]
    pub referrer_url: String,
    #[graphql(default)]
    pub landing_url: String,
    #[graphql(default)]
    pub channel: String,
    #[graphql(default)]
    pub note: String,
    #[graphql(default)]
    pub comments: String,
    #[graphql(skip)]
    pub coupon: Option<Thing>,
    #[graphql(default)]
    pub tracking_number: u16,
    #[graphql(skip)]
    pub customer_id: Option<Thing>,
    #[graphql(skip)]
    pub status_id: Thing,
    #[graphql(skip)]
    pub payment_method_id: Thing,
    #[graphql(skip)]
    pub shipping_method_id: Thing,
    #[graphql(default)]
    pub tags: Vec<String>,
}

#[ComplexObject]
impl Order {
    async fn date_stock_expected(&self) -> String {
        String::new()
    }
}

#[derive(Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct OrderRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub order: Order,
}

#[ComplexObject]
impl OrderRecord {
    async fn id(&self) -> String {
        format!("{}:{}", &self.id.tb, &self.id.id)
    }
}
