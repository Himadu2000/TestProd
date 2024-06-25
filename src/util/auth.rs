use super::graphql::Headers;
use swd::{async_graphql::Context, Object};

pub async fn is_in_the_store<'ctx>(
    ctx: &Context<'ctx>,
    _scope: String,
) -> Result<(), &'static str> {
    let headers = ctx.data::<Headers>().unwrap();

    if headers.store_id == "Bearer token03124701209" {
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
