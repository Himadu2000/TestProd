mod services;
mod util;

use crate::{
    services::{Mutation, Query},
    util::graphql::graphql,
};
use std::env::set_var;
use swd::{
    async_graphql::{EmptySubscription, Schema},
    index,
    rocket::{build, launch, routes},
    surrealdb::{engine::remote::http::Http, opt::auth::Root, Surreal},
    Cors,
};

#[launch]
async fn rocket() -> _ {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await.unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.unwrap();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();

    set_var("ROCKET_LIMITS", "{data-form=\"10MiB\"}");

    build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
