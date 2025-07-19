#![allow(clippy::all)]

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/cart_query.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize"
)]
pub struct CartQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/add_to_cart_mutation.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize"
)]
pub struct AddToCart;
