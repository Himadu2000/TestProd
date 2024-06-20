mod services;
mod util;

use crate::{
    services::{Mutation, Query},
    util::graphql::graphql,
};
use swd::{
    async_graphql::{EmptySubscription, Schema},
    index,
    rocket::{build, launch, routes},
    Cors,
};

#[launch]
fn rocket() -> _ {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

    build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
