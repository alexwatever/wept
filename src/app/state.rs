use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

// Modules
use crate::graphql::models::cart::cart_query;

/// # Global State Signal
///
/// A global signal that holds the state of the application.
/// Designed to be accessed from any component in the application.
pub static STATE: GlobalSignal<State> = Global::new(State::default);

/// # Session Token Key
///
/// The key for the WooCommerce session token in local storage.
pub const SESSION_TOKEN_KEY: &str = "woocommerce-session";

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<cart_query::CartQueryCartContentsNodes>,
    pub total: String,
    pub subtotal: String,
}

/// # Global State
///
/// The global state of the application.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// Backend host
    backend_host: String,
    /// Backend path
    backend_path: String,
    /// Cart
    pub cart: Cart,
}

impl State {
    /// # Default State
    ///
    /// Create a default state instance.
    pub fn default() -> Self {
        let cart = LocalStorage::get("cart").unwrap_or_default();

        Self {
            backend_host: Env::backend_host_default(),
            backend_path: Env::backend_path_default(),
            cart,
        }
    }

    /// # Get Backend Host
    ///
    /// Get the backend host from the state.
    pub fn get_backend_host() -> String {
        #[cfg(target_family = "wasm")]
        {
            // In WASM context, use the GlobalSignal or fallback
            let state = STATE.try_read();
            match state {
                Ok(state) => state.backend_host.clone(),
                Err(err) => {
                    tracing::error!("Error reading state for backend host: {err}");
                    Env::backend_host_default()
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let value = STATE.read();
            value.backend_host.clone()
        }
    }

    /// # Get Backend Path
    ///
    /// Get the backend path from the state.
    pub fn get_backend_path() -> String {
        #[cfg(target_family = "wasm")]
        {
            // In WASM context, use the GlobalSignal or fallback
            let state = STATE.try_read();
            match state {
                Ok(state) => state.backend_path.clone(),
                Err(err) => {
                    tracing::error!("Error reading state for backend path: {err}");
                    Env::backend_path_default()
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let value = STATE.signal();
            let value = value.read();
            value.backend_path.clone()
        }
    }

    /// # Save Cart
    ///
    /// Save the cart to local storage. This should be called
    /// whenever the cart state is modified.
    pub fn save_cart(&self) {
        if let Err(e) = LocalStorage::set("cart", &self.cart) {
            tracing::error!("Failed to save cart to local storage: {}", e);
        }
    }
}

/// # Environment Variables
///
/// The environment variables for the application.
#[derive(Deserialize, Default, Debug)]
struct Env {}

impl Env {
    /// # Fallback Backend Host
    const FALLBACK_BACKEND_HOST: &str = "http://localhost:8080";

    /// # Fallback Backend Path
    const FALLBACK_BACKEND_PATH: &str = "graphql";

    /// # Default Backend Host
    ///
    /// Provides a fallback for the backend WordPress host.
    fn backend_host_default() -> String {
        Self::FALLBACK_BACKEND_HOST.to_string()
    }

    /// # Default Backend Path
    ///
    /// Provides a fallback for the backend GraphQL API path.
    fn backend_path_default() -> String {
        Self::FALLBACK_BACKEND_PATH.to_string()
    }
}
