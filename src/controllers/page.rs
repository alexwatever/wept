use async_trait::async_trait;

// Modules
use crate::{
    app::error::{AppError, AppErrorKind, GraphQLErrorWrapper},
    controllers::common::EntityController,
    graphql::{
        client::GraphQLClient,
        models::page::{page_query, pages_query, PageQuery, PagesQuery},
    },
    models::page::{Page, Pages},
};

/// Page controller
#[derive(Debug)]
pub struct PageController {
    /// The GraphQL client used for API communication
    client: GraphQLClient,
}

impl PageController {
    /// Creates a new page controller
    pub fn new() -> Self {
        Self {
            client: GraphQLClient::new(),
        }
    }
}

/// Page controller implementation
#[async_trait(?Send)]
impl EntityController for PageController {
    /// A single page entity
    type Entity = Page;
    /// A collection of pages
    type EntityCollection = Pages;

    /// Get a page by slug
    ///
    /// **Arguments**
    ///
    /// * `slug` - The slug of the page to get
    ///
    /// **Returns**
    ///
    /// * `Self::Entity` - The page entity
    async fn get_by_slug(&self, slug: &str) -> Result<Self::Entity, AppError> {
        // Build the request
        let request: page_query::Variables = page_query::Variables {
            slug: slug.to_string(),
        };
        let request = self
            .client
            .execute_query::<_, PageQuery, page_query::ResponseData>(request);

        // Execute the request
        let request: page_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the page.".to_string(),
                Some(format!(
                    "Failed to execute get_page query for slug '{slug}'"
                )),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the page
        let request: Option<Page> = request.page.map(Page::from);

        // Return the page
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested page could not be found.".to_string(),
                Some(format!(
                    "Page with slug '{slug}' not found in GraphQL response."
                )),
                None,
            )
        })
    }

    /// Get multiple pages
    ///
    /// **Arguments**
    ///
    /// * `page_size` - The number of pages to get
    /// * `after` - The cursor to get the next page of pages
    ///
    /// **Returns**
    ///
    /// * `Self::EntityCollection` - The collection of pages
    async fn get_list(
        &self,
        page_size: Option<usize>,
        after: Option<String>,
    ) -> Result<Self::EntityCollection, AppError> {
        // Build the request
        let request = pages_query::Variables {
            first: Some(page_size.unwrap_or(10) as i64),
            after: after.clone(),
        };
        let request = self
            .client
            .execute_query::<_, PagesQuery, pages_query::ResponseData>(request);

        // Execute the request
        let request: pages_query::ResponseData = request.await.map_err(|err| {
            AppError::new_with_source(
                AppErrorKind::GraphQL,
                "An error occurred while fetching the list of pages.".to_string(),
                Some(format!(
                    "Failed to execute get_pages query. Page size: '{page_size:?}', After: '{after:?}'"
                )),
                GraphQLErrorWrapper(err),
            )
        })?;

        // Cast the pages
        let request: Option<Pages> = request.pages.map(Pages::from);

        // Return the pages
        request.ok_or_else(|| {
            AppError::new(
                AppErrorKind::NotFound,
                "The requested pages could not be found.".to_string(),
                Some("Pages not found in GraphQL response.".to_string()),
                None,
            )
        })
    }
}
