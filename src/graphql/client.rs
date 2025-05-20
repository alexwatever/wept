use crate::app::state::State;
use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

/// GraphQL client for making requests to the WordPress API
#[derive(Default, Debug)]
pub struct GraphQLClient {
    /// HTTP client for making network requests
    pub client: Client,
}

impl GraphQLClient {
    /// Create a new GraphQL client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Execute a GraphQL query
    pub async fn execute_query<V, Q, R>(&self, variables: V) -> Result<R, String>
    where
        V: Serialize,
        Q: GraphQLQuery<Variables = V>,
        R: DeserializeOwned,
    {
        let payload: QueryBody<V> = Q::build_query(variables);
        self.execute_request::<R>(payload).await
    }

    /// Execute a GraphQL request
    pub async fn execute_request<R>(&self, payload: QueryBody<impl Serialize>) -> Result<R, String>
    where
        R: DeserializeOwned,
    {
        self.client
            // Get the endpoint
            .post(Self::get_endpoint())
            // Set the payload
            .json(&payload)
            // Send the request
            .send()
            .await
            // Handle request errors
            .map_err(|err| format!("Request failed: {err}"))?
            // Parse the response
            .json::<Response<R>>()
            .await
            // Handle response content
            .map(|response| {
                // Check the response for errors
                if let Some(errors) = response.errors {
                    if !errors.is_empty() {
                        return Err(format!("GraphQL error: {errors:?}"));
                    }
                }

                // Check the response data
                match response.data {
                    Some(data) => Ok(data),
                    None => Err("No data in response".to_string()),
                }
            })
            // Handle response parsing errors
            .map_err(|err| format!("Failed to parse response: {err}"))?
    }

    /// Get the GraphQL endpoint URL
    fn get_endpoint() -> String {
        format!(
            "{}/{}",
            State::get_backend_host(),
            State::get_backend_path()
        )
    }
}
