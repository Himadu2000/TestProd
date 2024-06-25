pub mod auth;
pub mod files;
pub mod graphql;
pub mod store;

use swd::async_graphql::Error;

pub fn error(_: Error) -> &'static str {
    "Connection error...!"
}
