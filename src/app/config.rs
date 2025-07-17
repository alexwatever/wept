use serde::Deserialize;

/// Application configuration constants and settings
///
/// This structure holds configuration values that can be used throughout the application.
#[derive(Debug)]
pub struct AppConfig {
    /// Application name from Cargo.toml
    _name: &'static str,
    /// Application version from Cargo.toml
    _version: &'static str,
}

impl Default for AppConfig {
    /// # Default Configuration
    ///
    /// Create a default configuration instance.
    fn default() -> Self {
        let config: Config = Config::default();
        Self {
            _name: config.name,
            _version: config.version,
        }
    }
}

/// # Config
///
/// The configuration for the application.
#[derive(Deserialize, Default, Debug)]
struct Config {
    /// Application name from Cargo.toml
    #[serde(default = "Config::name_default")]
    name: &'static str,
    /// Application version from Cargo.toml
    #[serde(default = "Config::version_default")]
    version: &'static str,
}

impl Config {
    /// # Default Configuration Variables
    fn default() -> Self {
        Self {
            name: Self::name_default(),
            version: Self::version_default(),
        }
    }

    /// # Application Name
    fn name_default() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// # Application Version
    fn version_default() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
