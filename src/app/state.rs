use dioxus::prelude::*;
use serde::Deserialize;
use tracing::warn;

// # Modules
use super::config::AppConfig;

/// # Global State Signal
///
/// A global signal that holds the state of the application.
/// Designed to be accessed from any component in the application.
pub static STATE: GlobalSignal<State> = Global::new(State::default);

/// # Global State
///
/// The global state of the application.
#[derive(Debug)]
pub struct State {
    env: Env,
    config: AppConfig,
}

impl State {
    /// # Default State
    ///
    /// Create a default state instance.
    pub fn default() -> Self {
        Self {
            env: Env::default(),
            config: AppConfig::default(),
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
                Ok(state) => state.env.backend_host.clone(),
                Err(err) => {
                    tracing::error!("Error reading state for backend host: {err}");
                    Env::backend_host_default()
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let value = STATE.read();
            value.env.backend_host.clone()
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
                Ok(state) => state.env.backend_path.clone(),
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
            value.env.backend_path.clone()
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            match envy::from_env::<Env>() {
                Ok(env_vars) => env_vars,
                Err(err) => {
                    warn!("Failed to load .env file ({err:?}), using fallback configuration.");
                    Self::default_with_fallbacks()
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, always use fallbacks as environment variables are not available.
            warn!("Running in WASM, using fallback configuration for Env.");
            Self::default_with_fallbacks()
        }
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
