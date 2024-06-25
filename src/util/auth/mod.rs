mod models;

use super::graphql::Headers;
pub use models::Token;
use swd::{async_graphql::Context, Object};

pub async fn is_authorized<'ctx>(ctx: &Context<'ctx>, _scope: String) -> Result<(), &'static str> {
    // let token = ctx
    //     .insert_http_header("Authorization", "")
    //     .ok_or("Authorization header not set...!")?;

    // let token = token
    //     .to_str()
    //     .map_err(|_| "Incorrect Authorization header...!")?;

    let headers = ctx.data::<Headers>().unwrap();

    if headers.authorization == "Bearer token03124701209" {
        return Ok(());
    }

    Err("Not authorized...!")
}

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn login(&self, #[graphql(validator(email))] email: String) -> Result<String, &str> {
        if email == String::from("03124701209@gmail.com") {
            return Ok(String::from("Bearer token03124701209"));
        }

        Err("Not given access...!")
    }
}
