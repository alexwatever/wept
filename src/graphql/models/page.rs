use graphql_client::GraphQLQuery;

/// Page GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/page_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct PageQuery;

/// Pages GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/pages_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct PagesQuery;
