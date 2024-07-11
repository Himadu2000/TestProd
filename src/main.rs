// mod services;
// mod util;

// use crate::{
//     services::{Mutation, Query},
//     util::{files::files, get_db, graphql::graphql},
// };
// use swd::{
//     async_graphql::{EmptySubscription, Schema},
//     index,
//     rocket::{
//         build,
//         data::{Limits, ToByteUnit},
//         launch, routes, Config,
//     },
//     surrealdb::{engine::remote::http::Http, opt::auth::Root, Surreal},
//     Cors,
// };
use rocket::{
    build,
    data::{Limits, ToByteUnit},
    get, launch, routes, Config,
};

#[get("/graphql")]
fn index() -> &'static str {
    "Hello World!"
}

#[launch]
async fn rocket() -> _ {
    // let db_url = get_db();

    // // Create database connection
    // let db = Surreal::new::<Http>(db_url.url).await.unwrap();

    // // Sign in as root
    // db.signin(Root {
    //     username: &db_url.user,
    //     password: &db_url.pass,
    // })
    // .await
    // .unwrap();

    // // Select a specific namespace / database
    // db.use_ns(db_url.ns).use_db(db_url.db).await.unwrap();

    // let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    //     .data(db.clone())
    //     .finish();

    build()
        .configure(Config {
            limits: Limits::default().limit("data-form", 10.megabytes()),
            ..Default::default()
        })
        // .attach(Cors)
        // .manage(schema)
        // .manage(db)
        .mount("/", routes![index])
    // .mount("/graphql", graphql())
}
