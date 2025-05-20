use graphql_client::GraphQLQuery;

/// Product GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/product_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct ProductQuery;

/// Products GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/products_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct ProductsQuery;
