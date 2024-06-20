use crate::services::{Mutation, Query};
use swd::{
    async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse},
    graphiql,
    rocket::{get, post, routes, Route},
    GQLSchema,
};

type GraphqlSchema = GQLSchema<Query, Mutation>;

#[get("/?<query..>")]
async fn graphql_query(schema: &GraphqlSchema, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[post("/", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &GraphqlSchema, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

pub fn graphql() -> impl Into<Vec<Route>> {
    routes![graphiql, graphql_query, graphql_request]
}
