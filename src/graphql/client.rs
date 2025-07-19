pub use graphql_client::Response;
use graphql_client::{GraphQLQuery, QueryBody};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

// Modules
use crate::app::state::State;

/// GraphQL client for making requests to the WordPress API
#[derive(Default, Clone, Debug)]
pub struct GraphQLClient {
    /// HTTP client for making network requests
    pub client: Client,
    /// WooCommerce session token
    session_token: Option<String>,
}

impl GraphQLClient {
    /// Create a new GraphQL client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            session_token: None,
        }
    }

    /// Set the session token
    pub fn set_session_token(&mut self, token: String) {
        self.session_token = Some(token);
    }

    /// Execute a GraphQL query
    pub async fn execute_query<V, Q, R>(&self, variables: V) -> Result<R, String>
    where
        V: Serialize,
        Q: GraphQLQuery<Variables = V>,
        R: DeserializeOwned,
    {
        let payload: QueryBody<V> = Q::build_query(variables);
        let response = self.execute_request::<R>(payload).await?;

        if let Some(errors) = response.errors {
            if !errors.is_empty() {
                return Err(format!("GraphQL error: {errors:?}"));
            }
        }

        response
            .data
            .ok_or_else(|| "No data in response".to_string())
    }

    /// Execute a GraphQL mutation
    pub async fn execute_mutation<V, Q, R>(&self, variables: V) -> Result<reqwest::Response, String>
    where
        V: Serialize,
        Q: GraphQLQuery<Variables = V>,
    {
        let payload: QueryBody<V> = Q::build_query(variables);
        self.execute_mutation_request(payload).await
    }

    /// Execute a GraphQL request
    async fn execute_request<R>(
        &self,
        payload: QueryBody<impl Serialize>,
    ) -> Result<Response<R>, String>
    where
        R: DeserializeOwned,
    {
        let mut builder = self.client.post(Self::get_endpoint());
        if let Some(token) = &self.session_token {
            builder = builder.header("woocommerce-session", format!("Session {}", token));
        }

        builder
            .json(&payload)
            .send()
            .await
            .map_err(|err| format!("Request failed: {err}"))?
            .json::<Response<R>>()
            .await
            .map_err(|err| format!("Failed to parse response: {err}"))
    }

    /// Execute a GraphQL mutation request
    async fn execute_mutation_request(
        &self,
        payload: QueryBody<impl Serialize>,
    ) -> Result<reqwest::Response, String> {
        let mut builder = self.client.post(Self::get_endpoint());
        if let Some(token) = &self.session_token {
            builder = builder.header("woocommerce-session", format!("Session {}", token));
        }

        builder
            .json(&payload)
            .send()
            .await
            .map_err(|err| format!("Request failed: {err}"))
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
