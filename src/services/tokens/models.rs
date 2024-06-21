use swd::{ComplexObject, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing};

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenMethod {
    #[default]
    READ,
    WRITE,
}

#[derive(Clone, Copy, Default, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenItem {
    #[default]
    SELECT,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "TokenScopeInput")]
pub struct TokenScope {
    pub method: TokenMethod,
    pub item: TokenItem,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "ProductInput")]
pub struct Token {
    name: String,
    email: String,
    scopes: TokenScope,
    expiration: u16,
}

#[derive(Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct TokenRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub token: Token,
}

#[ComplexObject]
impl TokenRecord {
    async fn id(&self) -> String {
        format!("{}:{}", &self.id.tb, &self.id.id)
    }
}
