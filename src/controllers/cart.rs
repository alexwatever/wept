use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    graphql::{
        client::{GraphQLClient, Response},
        models::cart::{add_to_cart, cart_query, AddToCart, CartQuery},
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;
}

const SESSION_TOKEN_KEY: &str = "woocommerce-session";

#[derive(Clone)]
pub struct CartController {
    client: GraphQLClient,
}

impl CartController {
    pub fn new() -> Self {
        let mut client = GraphQLClient::new();
        if let Some(token) = Self::get_session_token() {
            client.set_session_token(token);
        }
        Self { client }
    }

    pub fn get_session_token() -> Option<String> {
        getItem(SESSION_TOKEN_KEY)
    }

    pub fn set_session_token(token: &str) {
        setItem(SESSION_TOKEN_KEY, token);
    }

    pub async fn get_cart(&self) -> Result<Option<cart_query::ResponseData>, AppError> {
        let variables = cart_query::Variables {};
        self.client
            .execute_query::<_, CartQuery, _>(variables)
            .await
            .map_err(|e| {
                AppError::new_with_source(
                    AppErrorKind::GraphQL,
                    "An error occurred while fetching the cart.".to_string(),
                    Some("Failed to execute get_cart query.".to_string()),
                    GraphQLErrorWrapper(e),
                )
            })
    }

    pub async fn add_to_cart(
        &mut self,
        product_id: i64,
        quantity: i64,
    ) -> Result<Option<add_to_cart::ResponseData>, AppError> {
        let variables = add_to_cart::Variables {
            product_id,
            quantity: Some(quantity),
        };
        let response = self
            .client
            .execute_mutation::<_, AddToCart>(variables)
            .await
            .map_err(|e| {
                AppError::new_with_source(
                    AppErrorKind::GraphQL,
                    "An error occurred while adding the item to the cart.".to_string(),
                    Some(format!(
                        "Failed to execute add_to_cart mutation for product_id '{}'.",
                        product_id
                    )),
                    GraphQLErrorWrapper(e),
                )
            })?;

        if let Some(token) = response.headers().get("woocommerce-session") {
            if let Ok(token_str) = token.to_str() {
                Self::set_session_token(token_str);
                self.client.set_session_token(token_str.to_string());
            }
        }

        let response_body: Response<add_to_cart::ResponseData> =
            response.json().await.map_err(|e| {
                AppError::new(
                    AppErrorKind::Parse,
                    "An error occurred while parsing the cart response.".to_string(),
                    Some(format!("Failed to parse add_to_cart response: {}", e)),
                    None,
                )
            })?;

        if let Some(errors) = response_body.errors {
            if !errors.is_empty() {
                return Err(AppError::new(
                    AppErrorKind::GraphQL,
                    "An error occurred while adding the item to the cart.".to_string(),
                    Some(format!("GraphQL error: {:?}", errors)),
                    None,
                ));
            }
        }

        Ok(response_body.data)
    }
}
