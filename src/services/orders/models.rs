use swd::{ComplexObject, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing};

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderOptionControl {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "VariantInput")]
pub struct Item {
    #[graphql(skip)]
    id: Thing,
    #[graphql(skip)]
    product_id: Thing,
    #[graphql(skip)]
    variant_id: Thing,
    quantity: u16,
    discount_total: f32,
    name: String,
    price: f32,
    price_total: f32,
    sku: String,
    tax_class: String,
    tax_total: f32,
    variant_name: String,
    weight: f32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "VariantInput")]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "VariantInput")]
pub struct Address {
    full_name: String,
    address1: String,
    address2: String,
    city: String,
    country: String,
    postal_code: String,
    state: String,
    phone: String,
    company: String,
    tax_number: String,
    coordinates: Coordinates,
    details: String,
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
    date_placed: Datetime,
    #[graphql(skip)]
    date_closed: Option<Datetime>,
    #[graphql(skip)]
    date_paid: Option<Datetime>,
    #[graphql(skip)]
    date_cancelled: Option<Datetime>,
    #[graphql(default)]
    number: u16,
    #[graphql(default)]
    shipping_status: String,
    #[graphql(default)]
    items: Vec<Item>,
    #[graphql(default)]
    transactions: Vec<String>,
    #[graphql(default)]
    discounts: Vec<String>,
    billing_address: Address,
    shipping_address: Address,
    #[graphql(default)]
    tax_rate: f32,
    #[graphql(default)]
    shipping_tax: f32,
    #[graphql(default)]
    shipping_discount: f32,
    #[graphql(default)]
    shipping_price: f32,
    #[graphql(default)]
    item_tax_included: bool,
    #[graphql(default)]
    shipping_tax_included: bool,
    #[graphql(default)]
    closed: bool,
    #[graphql(default)]
    cancelled: bool,
    #[graphql(default)]
    delivered: bool,
    #[graphql(default)]
    paid: bool,
    #[graphql(default)]
    hold: bool,
    #[graphql(default = true)]
    draft: bool,
    #[graphql(default)]
    first_name: String,
    #[graphql(default)]
    last_name: String,
    password: Option<String>,
    #[graphql(default)]
    email: String,
    #[graphql(default)]
    mobile: u16,
    #[graphql(default)]
    referrer_url: String,
    #[graphql(default)]
    landing_url: String,
    #[graphql(default)]
    channel: String,
    #[graphql(default)]
    note: String,
    #[graphql(default)]
    comments: String,
    #[graphql(skip)]
    coupon: Option<Thing>,
    #[graphql(default)]
    tracking_number: u16,
    #[graphql(skip)]
    customer_id: Thing,
    #[graphql(skip)]
    status_id: Thing,
    #[graphql(skip)]
    payment_method_id: Thing,
    #[graphql(skip)]
    shipping_method_id: Thing,
    #[graphql(default)]
    tags: Vec<String>,
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
