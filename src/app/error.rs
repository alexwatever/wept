use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use std::{
    error::Error,
    fmt::{self, Display},
};
use tracing::error;
use uuid::Uuid;

// Modules
use crate::views::pages::errors::{GenericErrorPage, GenericErrorPageProps, NotFoundPage};

/// Represents a application error.
///
/// Errors are logged automatically upon creation of an AppError instance.
#[derive(Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
    pub public_message: String,
    pub internal_message: Option<String>,
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

/// Represents the kind of application error.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum AppErrorKind {
    /// Errors originating from API interactions
    Api,
    /// General errors that don't have a specific kind.
    #[default]
    General,
    /// Errors specific to GraphQL operations.
    GraphQL,
    /// Errors related to data not being found.
    NotFound,
    /// Errors during data parsing or transformation.
    Parse,
}

impl Display for AppErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErrorKind::Api => write!(f, "API Error"),
            AppErrorKind::General => write!(f, "General Error"),
            AppErrorKind::GraphQL => write!(f, "GraphQL Error"),
            AppErrorKind::NotFound => write!(f, "Not Found Error"),
            AppErrorKind::Parse => write!(f, "Parse Error"),
        }
    }
}

impl AppError {
    /// Creates a new AppError and logs it.
    ///
    /// **Arguments**
    ///
    /// * `kind` - The category of the error.
    /// * `public_message` - A user-friendly message.
    /// * `internal_message` - A detailed internal message for logging.
    /// * `source` - An optional source error.
    ///
    /// **Returns**
    ///
    /// * `AppError` - A new AppError instance.
    pub fn new<S: Into<String>>(
        kind: AppErrorKind,
        public_message: S,
        internal_message: Option<S>,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        let id: Uuid = Uuid::new_v4();
        let timestamp: DateTime<Utc> = Utc::now();
        let public_message: String = public_message.into();
        let internal_message: Option<String> = internal_message.map(|s| s.into());

        // Log the error
        error!(
            target: "app_error",
            error_id = %id,
            timestamp = %timestamp.to_rfc3339(),
            error_kind = %kind,
            public_message = %public_message,
            internal_message = %internal_message.as_ref().unwrap_or(&"".to_string()),
            source_error = ?source.as_ref().map(|s| s.to_string()),
            "AppError created"
        );

        Self {
            kind,
            public_message,
            internal_message,
            source,
        }
    }

    /// Creates a new AppError with a source error and logs it.
    ///
    /// **Arguments**
    ///
    /// * `kind` - The category of the error.
    /// * `public_message` - A user-friendly message.
    /// * `internal_message` - A detailed internal message for logging.
    /// * `source` - An optional source error.
    ///
    /// **Returns**
    ///
    /// * `AppError` - A new AppError instance.
    pub fn new_with_source<S: Into<String>, E: std::error::Error + Send + Sync + 'static>(
        kind: AppErrorKind,
        public_message: S,
        internal_message: Option<S>,
        source: E,
    ) -> Self {
        Self::new(
            kind,
            public_message,
            internal_message,
            Some(Box::new(source)),
        )
    }

    /// Renders an appropriate error page based on the AppError.
    ///
    /// This function centralizes the logic for deciding which error page to show.
    ///
    /// **Arguments**
    ///
    /// * `route_segments` - The route segments to pass to the error page.
    ///
    /// **Returns**
    ///
    /// * `Element` - The rendered error page.
    pub fn render(&self, route_segments: Vec<String>) -> Element {
        match self.kind {
            // 404 Not Found
            AppErrorKind::NotFound => {
                rsx! {
                    NotFoundPage {
                        message: self.internal_message.clone(),
                        route: route_segments,
                    }
                }
            }
            // Generic error
            _ => {
                let error_props: GenericErrorPageProps = GenericErrorPageProps {
                    kind: self.kind.clone(),
                    message: self.public_message.clone(),
                    route: Some(route_segments),
                };
                rsx! { GenericErrorPage { ..error_props } }
            }
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.public_message)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|b| b.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// Wraps a GraphQL error message.
///
/// This struct is used to wrap a GraphQL error.
#[derive(Debug)]
pub struct GraphQLErrorWrapper(pub String);

impl Display for GraphQLErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GraphQLErrorWrapper {}
