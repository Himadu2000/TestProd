mod filter;
mod product;

pub use filter::Filter;
pub use product::Image;
use product::{Attribute, ImageOutput, Variant};
use swd::{
    async_graphql::ID, ComplexObject, Datetime, Deserialize, InputObject, Serialize, SimpleObject,
    Thing,
};

#[derive(Default, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(complex, input_name = "ProductInput")]
pub struct Product {
    #[graphql(default)]
    pub name: String,
    #[graphql(default)]
    pub description: String,
    #[graphql(default)]
    pub slug: String,
    #[graphql(default)]
    pub meta_title: String,
    #[graphql(default)]
    pub meta_description: String,
    #[graphql(default)]
    pub regular_price: f32,
    #[graphql(default)]
    pub sale_price: f32,
    #[graphql(skip)]
    pub date_sale_from: Datetime,
    #[graphql(skip)]
    pub date_sale_to: Datetime,
    #[graphql(default)]
    pub sku: String,
    #[graphql(default)]
    pub stock_quantity: u8,
    #[graphql(default)]
    pub weight: f32,
    #[graphql(skip)]
    pub date_stock_expected: Datetime,
    #[graphql(default)]
    pub stock_tracking: bool,
    #[graphql(default)]
    pub stock_preorder: bool,
    #[graphql(default)]
    pub stock_backorder: bool,
    #[graphql(default)]
    pub discontinued: bool,
    #[graphql(default)]
    pub enabled: bool,
    #[graphql(default)]
    pub attributes: Vec<Attribute>,
    #[graphql(default)]
    pub variants: Vec<Variant>,
    #[graphql(skip)]
    pub category_ids: Vec<Thing>,
    #[graphql(default)]
    pub tags: Vec<String>,
    #[graphql(default)]
    pub position: u8,
    #[graphql(skip)]
    pub related_products: Vec<Thing>,
    #[graphql(skip)]
    #[serde(default)]
    pub images: Vec<Image>,
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

#[derive(Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct ProductRecord {
    #[allow(dead_code)]
    #[graphql(skip)]
    pub id: Thing,
    #[graphql(skip)]
    pub store_id: Thing,
    #[serde(flatten)]
    #[graphql(flatten)]
    pub product: Product,
}

#[ComplexObject]
impl ProductRecord {
    async fn id(&self) -> ID {
        ID::from(&self.id.id)
    }

    async fn images(&self) -> Vec<ImageOutput> {
        self.product
            .images
            .iter()
            .enumerate()
            .map(|(index, image)| ImageOutput {
                file: format!(
                    "/files/product/{}/{}/{}",
                    &self.store_id.id, &self.id.id, index
                ),
                mime: image.mime.clone(),
                alt: image.alt.clone(),
            })
            .collect()
    }
}

//? To include store_id when creating a product.
#[derive(Serialize, Deserialize, SimpleObject)]
pub struct ProductDbRecord {
    #[graphql(skip)]
    pub store_id: Thing,
    #[serde(flatten)]
    pub product: Product,
}
