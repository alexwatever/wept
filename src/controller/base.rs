use dioxus::Result as DxResult;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

// # Modules
use crate::{model::pagination::PageSort, State};

/// # GraphQL Entity Trait
///
/// This trait provides a standard interface for GraphQL entity queries.
pub trait GraphQLEntity: Sized {
    /// Variables type for the GraphQL query
    type Variables: Serialize;

    /// Query type for the GraphQL query
    type Query: GraphQLQuery<Variables = Self::Variables>;

    /// Response data type for the GraphQL query
    type ResponseData: DeserializeOwned;

    /// Nodes type from the GraphQL response
    type Nodes;

    /// Build the GraphQL query
    fn build_query(variables: Self::Variables) -> QueryBody<Self::Variables> {
        Self::Query::build_query(variables)
    }

    /// Extract nodes from the response
    fn extract_nodes(response: Response<Self::ResponseData>) -> DxResult<Vec<Self::Nodes>>;

    /// Convert nodes to the entity
    fn from_nodes(nodes: Vec<Self::Nodes>) -> Self;

    /// Make a GraphQL request
    async fn make_request(
        page_size: Option<usize>,
        _sort_direction: Option<PageSort>,
    ) -> DxResult<Self> {
        // Build the payload
        let first = page_size.unwrap_or(20) as i64;
        let variables = Self::create_variables(first, None);
        let payload: QueryBody<Self::Variables> = Self::build_query(variables);

        // Build the endpoint
        let endpoint: String = format!(
            "{host}/{path}",
            host = State::get_backend_host(),
            path = State::get_backend_path()
        );

        // Make the request
        let request = Client::new().post(endpoint).json(&payload).send().await?;

        // Parse the response
        let response = request.json::<Response<Self::ResponseData>>().await?;
        let nodes = Self::extract_nodes(response)?;

        // Build the entity
        Ok(Self::from_nodes(nodes))
    }

    /// Create variables for the query
    fn create_variables(first: i64, after: Option<String>) -> Self::Variables;
}
