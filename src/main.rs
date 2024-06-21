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
    rocket::{build, fairing::AdHoc, http::hyper::header::AUTHORIZATION, launch, routes},
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
        .attach(AdHoc::on_response(
            "Set Authorization and store_id headers to response!",
            |req, res| {
                let headers = req.headers();

                let authorization = headers.get_one("Authorization").unwrap_or_default();
                let store_id = headers.get_one("store_id").unwrap_or_default();

                res.adjoin_header();
                res.adjoin_header();
            },
        ))
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
