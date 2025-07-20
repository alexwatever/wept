use wasm_bindgen::prelude::*;

// Modules
use crate::{
    app::{
        error::{AppError, GraphQLErrorWrapper},
        state::SESSION_TOKEN_KEY,
    },
    graphql::{
        client::{GraphQLClient, Response},
        models::cart::{
            add_to_cart, cart_query, remove_items_from_cart, update_item_quantities, AddToCart,
            CartQuery, RemoveItemsFromCart, UpdateItemQuantities,
        },
    },
};

#[wasm_bindgen]
extern "C" {
    // LocalStorage setItem
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);

    // LocalStorage getItem
    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;
}

/// Cart controller
#[derive(Clone)]
pub struct CartController {
    client: GraphQLClient,
}

impl CartController {
    /// Creates a new cart controller.
    ///
    /// **Returns**
    ///
    /// * `CartController` - The new cart controller.
    pub fn new() -> Self {
        let mut client = GraphQLClient::new();
        if let Some(token) = Self::get_session_token() {
            client.set_session_token(token);
        }
        Self { client }
    }

    /// Gets the session token from local storage.
    ///
    /// **Returns**
    ///
    /// * `Option<String>` - The session token.
    pub fn get_session_token() -> Option<String> {
        getItem(SESSION_TOKEN_KEY)
    }

    /// Sets the session token in local storage.
    ///
    /// **Arguments**
    ///
    /// * `token` - The session token to set.
    pub fn set_session_token(token: &str) {
        setItem(SESSION_TOKEN_KEY, token);
    }

    /// Gets the cart from the API.
    ///
    /// **Returns**
    ///
    /// * `Result<Option<cart_query::ResponseData>, AppError>` - The cart data.
    pub async fn get_cart(&self) -> Result<Option<cart_query::ResponseData>, AppError> {
        let variables = cart_query::Variables {};
        self.client
            .execute_query::<_, CartQuery, _>(variables)
            .await
            .map_err(|err| AppError::from(GraphQLErrorWrapper(err)))
    }

    /// Adds an item to the cart.
    ///
    /// **Arguments**
    ///
    /// * `product_id` - The ID of the product to add.
    /// * `quantity` - The quantity of the product to add.
    ///
    /// **Returns**
    ///
    /// * `Result<Option<add_to_cart::ResponseData>, AppError>` - The cart data.
    pub async fn add_to_cart(
        &mut self,
        product_id: i64,
        quantity: i64,
    ) -> Result<Option<add_to_cart::ResponseData>, AppError> {
        // Build the variables for the mutation.
        let variables = add_to_cart::Variables {
            product_id,
            quantity: Some(quantity),
        };

        // Execute the mutation.
        let response = self
            .client
            .execute_mutation::<_, AddToCart>(variables)
            .await
            .map_err(GraphQLErrorWrapper)?;

        // Set the session token in local storage.
        if let Some(token) = response.headers().get(SESSION_TOKEN_KEY) {
            if let Ok(token_str) = token.to_str() {
                Self::set_session_token(token_str);
                self.client.set_session_token(token_str.to_string());
            }
        }

        // Parse the response.
        let response_body: Response<add_to_cart::ResponseData> = response.json().await?;

        // Check for errors.
        if let Some(errors) = response_body.errors {
            if !errors.is_empty() {
                Err(errors)?
            }
        }

        // Return the cart data.
        Ok(response_body.data)
    }

    /// Updates the quantity of an item in the cart.
    ///
    /// **Arguments**
    ///
    /// * `key` - The key of the item to update.
    /// * `quantity` - The quantity of the item to update.
    ///
    /// **Returns**
    ///
    /// * `Result<Option<update_item_quantities::ResponseData>, AppError>` - The cart data.
    pub async fn update_item_quantity(
        &mut self,
        key: String,
        quantity: i64,
    ) -> Result<Option<update_item_quantities::ResponseData>, AppError> {
        // Build the variables for the mutation.
        let variables = update_item_quantities::Variables {
            items: vec![Some(update_item_quantities::CartItemQuantityInput {
                key: key.clone(),
                quantity,
            })],
        };

        // Execute the mutation.
        let response = self
            .client
            .execute_mutation::<_, UpdateItemQuantities>(variables)
            .await
            .map_err(GraphQLErrorWrapper)?;

        // Parse the response.
        let response_body: Response<update_item_quantities::ResponseData> = response.json().await?;

        // Check for errors.
        if let Some(errors) = response_body.errors {
            if !errors.is_empty() {
                Err(errors)?
            }
        }

        // Return the cart data.
        Ok(response_body.data)
    }

    /// Removes an item from the cart.
    ///
    /// **Arguments**
    ///
    /// * `key` - The key of the item to remove.
    ///
    /// **Returns**
    ///
    /// * `Result<Option<remove_items_from_cart::ResponseData>, AppError>` - The cart data.
    pub async fn remove_item_from_cart(
        &mut self,
        key: String,
    ) -> Result<Option<remove_items_from_cart::ResponseData>, AppError> {
        // Build the variables for the mutation.
        let variables = remove_items_from_cart::Variables {
            keys: vec![Some(key.clone())],
        };

        // Execute the mutation.
        let response = self
            .client
            .execute_mutation::<_, RemoveItemsFromCart>(variables)
            .await
            .map_err(GraphQLErrorWrapper)?;

        // Parse the response.
        let response_body: Response<remove_items_from_cart::ResponseData> = response.json().await?;

        // Check for errors.
        if let Some(errors) = response_body.errors {
            if !errors.is_empty() {
                Err(errors)?
            }
        }

        // Return the cart data.
        Ok(response_body.data)
    }
}
