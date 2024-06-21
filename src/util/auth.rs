use swd::Object;

pub async fn is_authorized(token: String) -> Result<(), &'static str> {
    if token == String::from("token03124701209") {
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
