use swd::{ComplexObject, Deserialize, Enum, InputObject, Serialize, Thing};

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductFilterStockStatus {
    AVAILABLE,
}

#[derive(Clone, Copy, Enum, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProductFilterSort {
    PriceMinToMax,
    PriceMaxToMin,
    StockHighest,
    StockLowest,
}

#[derive(InputObject)]
#[graphql(complex)]
pub struct Filter {
    #[allow(dead_code)]
    #[graphql(skip)]
    category_id: Option<Thing>,
    active: Option<bool>,
    discontinued: Option<bool>,
    search: Option<String>,
    on_sale: Option<bool>,
    stock_status: ProductFilterStockStatus,
    price_from: Option<f32>,
    price_to: Option<f32>,
    sku: Option<String>,
    #[allow(dead_code)]
    #[graphql(skip)]
    ids: Option<Vec<Thing>>,
    sort: Option<ProductFilterSort>,
}

#[ComplexObject]
impl Filter {
    async fn id(&self) -> String {
        String::new()
    }
}
