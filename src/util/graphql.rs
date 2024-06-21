use crate::services::{Mutation, Query};
use swd::{
    async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse},
    graphiql,
    rocket::{
        async_trait, get, post,
        request::{FromRequest, Outcome, Request},
        routes, Route,
    },
    GQLSchema,
};

type GraphqlSchema = GQLSchema<Query, Mutation>;

#[get("/?<query..>")]
async fn graphql_query(schema: &GraphqlSchema, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

pub struct Headers {
    pub authorization: String,
    pub store_id: String,
}

#[async_trait]
impl<'req> FromRequest<'req> for Headers {
    type Error = String;

    async fn from_request(req: &'req Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = req.headers();

        let authorization = headers.get_one("Authorization").unwrap_or_default();
        let store_id = headers.get_one("store_id").unwrap_or_default();

        Outcome::Success(Headers {
            authorization: authorization.to_owned(),
            store_id: store_id.to_owned(),
        })
    }
}

#[post("/", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &GraphqlSchema,
    request: GraphQLRequest,
    headers: Headers,
) -> GraphQLResponse {
    request.data(headers).execute(schema.inner()).await
}

pub fn graphql() -> impl Into<Vec<Route>> {
    routes![graphiql, graphql_query, graphql_request]
}
