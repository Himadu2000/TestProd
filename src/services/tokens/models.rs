use swd::{
    surrealdb::sql::Datetime, ComplexObject, Deserialize, Enum, InputObject, Serialize,
    SimpleObject, Thing,
};

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductOptionControl {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "VariantInput")]
pub struct Variant {
    pub sku: Option<String>,
    pub price: f32,
    pub stock_quantity: u16,
    pub weight: Option<f32>,
    #[graphql(skip)]
    pub options: Vec<VariantOption>,
}

#[ComplexObject]
impl Variant {
    async fn options(&self) -> String {
        String::new()
    }
}

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "ProductInput")]
pub struct Product {
    pub name: Option<String>,
    pub description: Option<String>,
    pub meta_description: Option<String>,
    pub meta_title: Option<String>,
    #[graphql(default)]
    pub tags: Vec<String>,
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
