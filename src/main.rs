mod services;
mod util;

use crate::{
    services::{Mutation, Query},
    util::graphql::graphql,
};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use swd::{
    async_graphql::{EmptySubscription, Schema},
    index,
    rocket::{build, launch, routes},
    Cors,
};

#[launch]
fn rocket() -> _ {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();

    build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index])
        .mount("/graphql", graphql())
}
