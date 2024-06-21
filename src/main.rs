mod services;
mod util;

use crate::{
    services::{Mutation, Query},
    util::graphql::graphql,
};
use surrealdb::{engine::local::Mem, Surreal};
use swd::{
    async_graphql::{EmptySubscription, Schema},
    index,
    rocket::{
        build,
        fairing::AdHoc,
        http::{hyper::header::AUTHORIZATION, Header},
        launch, routes,
    },
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

    build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
