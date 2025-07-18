#![allow(clippy::all)]

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/footer_query.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize"
)]
pub struct FooterQuery;
