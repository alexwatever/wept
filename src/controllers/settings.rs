use crate::graphql::{
    client::GraphQLClient,
    models::footer::{self, FooterQuery},
};

/// Settings controller
#[derive(Clone, Debug)]
pub struct SettingsController;

impl SettingsController {
    /// Creates a new instance of the `SettingsController`.
    pub fn new() -> Self {
        Self {}
    }

    /// Fetches the footer settings from the GraphQL backend.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `footer_query::ResponseData` if the query is
    /// successful, or an `anyhow::Error` if it fails.
    pub async fn get_footer_settings(
        &self,
    ) -> anyhow::Result<Option<footer::footer_query::ResponseData>> {
        let client = GraphQLClient::new();
        let variables = footer::footer_query::Variables {};
        let response_body = client
            .execute_query::<_, FooterQuery, _>(variables)
            .await
            .map_err(|e: String| anyhow::anyhow!(e))?;

        let response_data = response_body;
        Ok(response_data)
    }
}

impl Default for SettingsController {
    fn default() -> Self {
        Self::new()
    }
}
