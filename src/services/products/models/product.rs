use swd::{
    surrealdb::sql::Bytes, ComplexObject, Datetime, Deserialize, Enum, InputObject, Serialize,
    SimpleObject, Thing,
};

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "ImageInput")]
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

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "DimensionsInput")]
pub struct Dimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "AttributeInput")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "ValueInput")]
pub struct Value {
    pub name: String,
}

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductOptionControl {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "ProductOptionInput")]
pub struct ProductOption {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub position: u8,
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct VariantOption {
    pub option_id: Thing,
    pub value_id: Thing,
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
