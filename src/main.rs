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
    rocket::{build, fairing::AdHoc, launch, routes},
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
            "Set Authorization and store_id header to response!",
            |req, res| {
                res.set_header(req.headers().get_one("Authorization"));
                res.set_header(req.headers().get_one("store_id"));
            },
        ))
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
