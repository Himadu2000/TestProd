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
    pub name: Option<String>,
    pub description: Option<String>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
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
