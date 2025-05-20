/// # Error Pages Module
///
/// This module contains components for displaying various error pages,
pub mod not_found;
pub use not_found::NotFoundPage;
pub mod generic_error;
pub use generic_error::{GenericErrorPage, GenericErrorPageProps};
