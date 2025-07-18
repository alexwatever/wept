use graphql_client::GraphQLQuery;

/// Product GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/product_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct ProductQuery;

/// Products GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/products_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct ProductsQuery;

/// Search Products GraphQL Query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/product/search_products_query.graphql",
    response_derives = "Debug, PartialEq, Clone, Serialize, Deserialize"
)]
pub struct SearchProductsQuery;
