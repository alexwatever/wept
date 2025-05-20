use graphql_client::GraphQLQuery;

/// Post GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/post_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct PostQuery;

/// Posts GraphQL Query
#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/posts_query.graphql",
    response_derives = "Serialize, Deserialize, PartialEq, Eq, Clone, Debug",
    enums_derive = "Debug, Clone"
)]
pub struct PostsQuery;
