use swd::{ComplexObject, Deserialize, Enum, InputObject, Serialize, SimpleObject, Thing};

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "StoresInput")]
pub struct Stores {
    #[graphql(default = "Untitled")]
    pub name: String,
    #[graphql(skip)]
    pub users: Vec<Thing>,
    #[graphql(skip)]
    pub products: Vec<Thing>,
}

#[ComplexObject]
impl Stores {
    async fn users(&self) -> String {
        String::new()
    }

    async fn products(&self) -> String {
        String::new()
    }
}

#[derive(Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct StoresRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub stores: Stores,
}

#[ComplexObject]
impl StoresRecord {
    async fn id(&self) -> String {
        format!("{}:{}", &self.id.tb, &self.id.id)
    }
}
