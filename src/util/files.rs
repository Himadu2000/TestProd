use super::is_store_id_valid;
use swd::{
    rocket::{
        get,
        http::{ContentType, Status},
        response::status::NotFound,
        State,
    },
    surrealdb::{engine::remote::http::Client, Surreal},
    Deserialize,
};

#[get("/files/product/<store_id>/<product_id>/<index>")]
pub async fn files(
    db: &State<Surreal<Client>>,
    store_id: &str,
    product_id: &str,
    index: u8,
) -> (Status, (ContentType, Vec<u8>)) {
    is_store_id_valid(&store_id.to_owned())
        .map_err(|error| NotFound(error.to_owned()))
        .unwrap();

    is_store_id_valid(&product_id.to_owned())
        .map_err(|error| NotFound(error.to_owned()))
        .unwrap();

    #[derive(Deserialize)]
    pub struct Image {
        pub file: Vec<u8>,
        pub mime: String,
        #[allow(dead_code)]
        pub alt: String,
    }

    #[derive(Deserialize)]
    pub struct Product {
        pub images: Image,
    }

    let image: Option<Option<Product>> = db
        .query(format!(
            "SELECT images[{index}] FROM product:{product_id} WHERE store_id = store:{store_id};"
        ))
        .await
        .unwrap()
        .take(0)
        .ok();

    let image = match image {
        Some(value) => value,
        None => return (Status::NotFound, (ContentType::Bytes, vec![])),
    };

    let images = image.unwrap().images;

    let mime = ContentType::parse_flexible(&images.mime).unwrap_or(ContentType::PNG);

    (Status::Ok, (mime, images.file.to_vec()))
}
