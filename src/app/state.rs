use crate::graphql::models::cart::cart_query;
use dioxus::prelude::*;
use serde::Deserialize;
use tracing::warn;

/// # Global State Signal
///
/// A global signal that holds the state of the application.
/// Designed to be accessed from any component in the application.
pub static STATE: GlobalSignal<State> = Global::new(State::default);

#[derive(Clone, Default, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cart {
    pub items: Vec<cart_query::CartQueryCartContentsNodes>,
    pub total: String,
    pub subtotal: String,
}

/// # Global State
///
/// The global state of the application.
#[derive(Clone, Default, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct State {
    /// Backend host
    backend_host: String,
    /// Backend path
    backend_path: String,
    /// Cart
    pub cart: Cart,
}

impl State {
    /// Initialize the global state
    pub fn init() {
        // This function will be implemented later
    }

    /// # Default State
    ///
    /// Create a default state instance.
    pub fn default() -> Self {
        Self {
            backend_host: Env::backend_host_default(),
            backend_path: Env::backend_path_default(),
            cart: Cart::default(),
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
}

/// # Environment Variables
///
/// The environment variables for the application.
#[derive(Deserialize, Default, Debug)]
struct Env {
    /// WordPress backend host URL
    #[serde(default = "Env::backend_host_default")]
    backend_host: String,
    /// GraphQL API path relative to the host
    #[serde(default = "Env::backend_path_default")]
    backend_path: String,
}

impl Env {
    /// # Fallback Backend Host
    const FALLBACK_BACKEND_HOST: &str = "http://localhost:8080";

    /// # Fallback Backend Path
    const FALLBACK_BACKEND_PATH: &str = "graphql";

    /// # Default Environment Variables
    ///
    /// Attempts to load from .env file first (on non-WASM), then uses fallbacks.
    fn default() -> Self {
        warn!("Using fallback configuration for Env.");
        Self::default_with_fallbacks()
    }

    /// # Default with Fallbacks
    ///
    /// Creates a default configuration using fallback values.
    fn default_with_fallbacks() -> Self {
        Self {
            backend_host: Self::backend_host_default(),
            backend_path: Self::backend_path_default(),
        }
    }

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
