use dioxus::{launch, prelude::*};
use serde::Deserialize;
use tracing::{error, warn};

// # Modules
mod controller;
mod model;
mod routes;
mod view;
use routes::Routes;
use view::components::{loader::ChildrenOrLoading, nav::Nav};

fn main() {
    launch(move || {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }

            Nav {}

            ChildrenOrLoading {
                Router::<Routes> {}
            }
        }
    });
}

/// # Global State Signal
///
/// A global signal that holds the state of the application.
static STATE: GlobalSignal<State> = Global::new(|| State::default());

/// # Global State
///
/// The global state of the application.
#[derive(Debug, Clone)]
pub struct State {
    env: Env,
}

impl State {
    /// # Default State
    ///
    /// Create a default state instance.
    pub fn default() -> Self {
        Self {
            env: Env::default(),
        }
    }

    /// # Get Backend Host
    ///
    /// Get the backend host from the configuration.
    pub fn get_backend_host() -> String {
        // In WASM context, use the GlobalSignal or fallback
        let state = STATE.try_read();

        #[cfg(target_family = "wasm")]
        {
            // In WASM context, use the GlobalSignal or fallback
            let state = STATE.try_read();
            match state {
                Ok(state) => state.env.backend_host.clone(),
                Err(err) => {
                    error!("Error reading state for backend host: {err}");
                    Env::backend_host_default()
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let value = STATE.signal();
            let value = value.read();
            value.env.backend_host.clone()
        }
    }

    /// # Get Backend Path
    ///
    /// Get the backend path from the configuration.
    pub fn get_backend_path() -> String {
        #[cfg(target_family = "wasm")]
        {
            // In WASM context, use the GlobalSignal or fallback
            let state = STATE.try_read();
            match state {
                Ok(state) => state.env.backend_path.clone(),
                Err(err) => {
                    error!("Error reading state for backend path: {err}");
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
#[derive(Deserialize, Default, Clone, Debug)]
pub struct Env {
    #[serde(default = "Env::backend_host_default")]
    backend_host: String,
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
    /// Create a default configuration instance based on platform.
    fn default() -> Self {
        #[cfg(not(target_family = "wasm"))]
        {
            // Load environment variables for non-WASM platforms
            let mut env: Self = envy::from_env().unwrap_or_else(|error| {
                warn!("Error loading configuration: {error}");
                Self::default_with_fallbacks()
            });

            // Apply fallbacks if empty
            if env.backend_host.is_empty() {
                let host: String = Self::backend_host_default();
                warn!("No backend host set, using fallback: {host}");
                env.backend_host = host;
            }

            if env.backend_path.is_empty() {
                let path: String = Self::backend_path_default();
                warn!("No backend path set, using fallback: {path}");
                env.backend_path = path;
            }

            env
        }

        #[cfg(target_family = "wasm")]
        {
            // For WASM, use fallback values directly
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
