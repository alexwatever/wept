use crate::graphql::{
    client::GraphQLClient,
    models::navigation::{navigation_query, NavigationQuery},
};

#[derive(Clone)]
pub struct NavigationController {
    client: GraphQLClient,
}

impl NavigationController {
    pub fn new() -> Self {
        Self {
            client: GraphQLClient::new(),
        }
    }

    pub async fn get_menu(
        &self,
        menu_name: &str,
    ) -> anyhow::Result<Option<navigation_query::ResponseData>> {
        let variables = navigation_query::Variables {
            id: menu_name.to_string(),
        };
        let response_body = self
            .client
            .execute_query::<_, NavigationQuery, _>(variables)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(response_body)
    }
}
