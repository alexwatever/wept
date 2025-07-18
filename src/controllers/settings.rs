use crate::graphql::{
    client::GraphQLClient,
    models::settings::{self, WeptSettingsQuery},
};

/// Settings controller
#[derive(Clone, Debug)]
pub struct SettingsController;

impl SettingsController {
    /// Creates a new instance of the `SettingsController`.
    pub fn new() -> Self {
        Self {}
    }

    /// Fetches the settings from the GraphQL backend.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `wept_settings_query::ResponseData` if the query is
    /// successful, or an `anyhow::Error` if it fails.
    pub async fn get(&self) -> anyhow::Result<Option<settings::wept_settings_query::ResponseData>> {
        let client = GraphQLClient::new();
        let variables = settings::wept_settings_query::Variables {};
        let response_body = client
            .execute_query::<_, WeptSettingsQuery, _>(variables)
            .await
            .map_err(|e: String| anyhow::anyhow!(e))?;

        let response_data = response_body;
        Ok(response_data)
    }
}
