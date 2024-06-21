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
    number: u16,
    shipping_status: String,
    items: Vec<Item>,
    transactions: Vec<String>,
    discounts: Vec<String>,
    billing_address: Address,
    shipping_address: Address,
    tax_rate: f32,
    shipping_tax: f32,
    shipping_discount: f32,
    shipping_price: f32,
    item_tax_included: bool,
    shipping_tax_included: bool,
    closed: bool,
    cancelled: bool,
    delivered: bool,
    paid: bool,
    hold: bool,
    draft: bool,
    first_name: String,
    last_name: String,
    password: Option<String>,
    email: String,
    mobile: u16,
    referrer_url: String,
    landing_url: String,
    channel: String,
    note: String,
    comments: String,
    #[graphql(skip)]
    coupon: Option<Thing>,
    tracking_number: u16,
    #[graphql(skip)]
    customer_id: Thing,
    #[graphql(skip)]
    status_id: Thing,
    #[graphql(skip)]
    payment_method_id: Thing,
    #[graphql(skip)]
    shipping_method_id: Thing,
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
