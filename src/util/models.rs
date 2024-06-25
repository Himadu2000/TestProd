use swd::{ComplexObject, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing};

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenMethod {
    READ,
    WRITE,
}

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenItem {
    CustomerGroups,
    Customers,
    Files,
    OrderStatuses,
    Orders,
    Pages,
    PaymentMethods,
    ProductCategories,
    Products,
    Settings,
    ShippingMethods,
    Sitemap,
    Theme,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "TokenScopeInput")]
pub struct TokenScope {
    pub method: TokenMethod,
    pub item: TokenItem,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "TokenInput")]
pub struct Token {
    #[graphql(default)]
    pub name: String,
    pub email: String,
    #[graphql(default)]
    pub scopes: Vec<TokenScope>,
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
