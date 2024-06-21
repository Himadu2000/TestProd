use swd::{async_graphql::Context, Object};

pub async fn is_authorized<'ctx>(ctx: &Context<'ctx>) -> Result<(), &'static str> {
    let token = ctx
        .insert_http_header("Authorization", None)?
        .to_str()
        .ok()?;

    if token == "Bearer token03124701209" {
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
