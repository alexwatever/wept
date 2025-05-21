use graphql_client::GraphQLQuery;

/// Product Category GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/category_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    variables_derives = "Debug, Clone, PartialEq",
    enums_derive = "Debug, Clone, PartialEq, Eq, Serialize, Deserialize"
)]
pub struct ProductCategory;

/// Product Categories GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema/schema.graphql",
    query_path = "src/graphql/schema/categories_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    variables_derives = "Debug, Clone, PartialEq",
    enums_derive = "Debug, Clone, PartialEq, Eq, Serialize, Deserialize"
)]
pub struct ProductCategories;
