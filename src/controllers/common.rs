use crate::app::error::AppError;
use async_trait::async_trait;

/// Entity Controller trait
///
/// A trait for controllers that handle fetching entities.
#[async_trait(?Send)]
pub trait EntityController {
    /// The type of the single entity (e.g., Post, Product).
    type Entity;
    /// The type of the collection of entities (e.g., Posts, Products).
    type EntityCollection;

    /// Fetch a single entity by its slug.
    async fn get_by_slug(&self, slug: &str) -> Result<Self::Entity, AppError>;

    /// Fetch a list of entities.
    ///
    /// **Arguments**
    ///
    /// * `page_size` - Optional number of items to fetch.
    /// * `after` - Optional cursor for pagination to fetch items after this cursor.
    ///
    /// **Returns**
    ///
    /// * `Self::EntityCollection` - A list of entities.
    async fn get_list(
        &self,
        page_size: Option<usize>,
        after: Option<String>,
    ) -> Result<Self::EntityCollection, AppError>;
}
