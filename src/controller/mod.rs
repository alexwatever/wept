use dioxus::Result;

// # Modules
pub mod base;
pub mod posts;
pub mod products;
use crate::model::pagination::PageSort;

/// # Controller
///
/// A trait for controllers that interact with the backend.
pub trait Controller {
    /// # Fallback page size
    ///
    /// The fallback page size for the controller.
    const PAGE_SIZE: usize = 20;

    /// # Returned entity
    ///
    /// The entity returned by the controller.
    type ReturnedEntity;

    /// # Fetch a page of data
    ///
    /// Fetch a page of data from the controller.
    async fn get_page(
        page_size: Option<usize>,
        sort_direction: Option<PageSort>,
    ) -> Result<Self::ReturnedEntity>;
}
