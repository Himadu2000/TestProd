use swd::{surrealdb::sql::Bytes, Deserialize, Enum, InputObject, Serialize, SimpleObject};

#[derive(Serialize, Deserialize, Clone)]
pub struct Image {
    #[serde(skip_deserializing)]
    pub file: Bytes,
    #[serde(alias = "file")]
    #[serde(skip_serializing)]
    pub file_as_vec: Vec<u8>,
    pub mime: String,
    // #[allow(dead_code)]
    // #[serde(skip_serializing)]
    // pub hash: String,
    pub alt: String,
}

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct ImageOutput {
    pub file: String,
    pub mime: String,
    // #[graphql(skip)]
    // #[allow(dead_code)]
    // #[serde(skip_serializing)]
    // pub hash: String,
    pub alt: String,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "AttributeInput")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductOptionControl {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "ProductOptionInput")]
pub struct VariantOption {
    pub name: String,
    pub control: ProductOptionControl,
    pub required: bool,
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "VariantInput")]
pub struct Variant {
    pub sku: String,
    pub price: f32,
    pub stock_quantity: u16,
    pub weight: f32,
    pub options: Vec<VariantOption>,
}
