use swd::Object;

pub async fn is_authorized(token: String) -> Result<(), &str> {
    if token == String::from("token03124701209") {
        Ok(())
    }

    Err("Not authorized...!")
}

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
