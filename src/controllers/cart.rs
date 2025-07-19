use crate::graphql::{
    client::{GraphQLClient, Response},
    models::cart::{add_to_cart, cart_query, AddToCart, CartQuery},
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

    pub async fn get_cart(&self) -> anyhow::Result<Option<cart_query::ResponseData>> {
        let variables = cart_query::Variables {};
        let response_body = self
            .client
            .execute_query::<_, CartQuery, _>(variables)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(response_body)
    }

    pub async fn add_to_cart(
        &self,
        product_id: i64,
        quantity: i64,
    ) -> anyhow::Result<Option<add_to_cart::ResponseData>> {
        let variables = add_to_cart::Variables {
            product_id,
            quantity: Some(quantity),
        };
        let response = self
            .client
            .execute_mutation::<_, AddToCart, add_to_cart::ResponseData>(variables)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        if let Some(token) = response.headers().get("woocommerce-session") {
            if let Ok(token_str) = token.to_str() {
                Self::set_session_token(token_str);
                let mut new_client = self.client.clone();
                new_client.set_session_token(token_str.to_string());
            }
        }

        let response_body: Response<add_to_cart::ResponseData> =
            response.json().await.map_err(|e| anyhow::anyhow!(e))?;

        Ok(response_body.data)
    }
}
