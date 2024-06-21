mod models;

use models::Product;
use swd::Object;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, #[graphql(validator(email))] email: String) -> Result<String, &str> {
        if email == String::from("03124701209@gmail.com") {
            return Ok(String::from("token03124701209"));
        }

        Err("Not given access...!")
    }
}
