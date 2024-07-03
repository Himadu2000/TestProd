mod services;
mod util;

use crate::{
    services::{Mutation, Query},
    util::{files::files, get_db, graphql::graphql},
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
    let db_url = get_db();

    // Create database connection
    let db = Surreal::new::<Http>(db_url.url).await.unwrap();

    // Sign in as root
    db.signin(Root {
        username: &db_url.user,
        password: &db_url.pass,
    })
    .await
    .unwrap();

    // Select a specific namespace / database
    db.use_ns(db_url.ns).use_db(db_url.db).await.unwrap();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();

    set_var("ROCKET_LIMITS", "{data-form=\"10MiB\"}");

    build()
        .attach(Cors)
        .manage(schema)
        .mount("/", routes![index, files])
        .mount("/graphql", graphql())
}
