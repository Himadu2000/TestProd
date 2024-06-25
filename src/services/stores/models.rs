use swd::{ComplexObject, Deserialize, InputObject, Serialize, SimpleObject, Thing};

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "StoreInput")]
pub struct Store {
    #[graphql(default = "Untitled")]
    pub name: String,
    #[graphql(skip)]
    pub users: Vec<Thing>,
}

#[ComplexObject]
impl Store {
    async fn users(&self) -> String {
        String::new()
    }
}

#[derive(Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct StoreRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub store: Store,
}

#[ComplexObject]
impl StoreRecord {
    async fn id(&self) -> String {
        format!("{}:{}", &self.id.tb, &self.id.id)
    }
}
