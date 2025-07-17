use serde::{Deserialize, Serialize};

// Modules
use crate::{
    graphql::models::page::{
        page_query::PageQueryPage,
        pages_query::{PagesQueryPages, PagesQueryPagesEdgesNode, PagesQueryPagesPageInfo},
    },
    models::pagination::Pagination,
};

/// Page entity representing a WordPress page
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Page {
    /// Page ID
    pub id: String,
    /// Page content
    pub content: Option<String>,
    /// Page slug
    pub slug: Option<String>,
    /// Page title
    pub title: Option<String>,
    /// Page date
    pub date: Option<String>,
}

impl From<PageQueryPage> for Page {
    /// Convert a PageQueryPage to a Page
    ///
    /// **Arguments**
    ///
    /// * `page` - The GraphQL response to convert
    ///
    /// **Returns**
    ///
    /// * `Page` - The converted Page
    fn from(page: PageQueryPage) -> Self {
        Self {
            id: page.id,
            content: page.content,
            slug: page.slug,
            title: page.title,
            date: page.date,
        }
    }
}

impl From<PagesQueryPagesEdgesNode> for Page {
    /// Convert a PagesQueryPagesEdgesNode to a Page
    ///
    /// **Arguments**
    ///
    /// * `page` - The GraphQL page to convert
    ///
    /// **Returns**
    ///
    /// * `Page` - The converted Page
    fn from(page: PagesQueryPagesEdgesNode) -> Self {
        Self {
            id: page.id,
            content: None,
            slug: page.slug,
            title: page.title,
            date: None,
        }
    }
}

/// Collection of pages with pagination information
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Pages {
    pub pages: Vec<Page>,
    pub page_info: Option<Pagination>,
}

impl From<PagesQueryPages> for Pages {
    /// Convert a PagesQueryPages to Pages
    ///
    /// **Arguments**
    ///
    /// * `pages` - The GraphQL pages to convert
    ///
    /// **Returns**
    ///
    /// * `Pages` - The converted Pages
    fn from(pages: PagesQueryPages) -> Self {
        let page_info: Option<Pagination> = Some(Pagination::from(pages.page_info));
        let pages: Vec<Page> = pages
            .edges
            .into_iter()
            .map(|edge| Page::from(edge.node))
            .collect();

        Self { pages, page_info }
    }
}

impl From<PagesQueryPagesPageInfo> for Pagination {
    /// Convert a PagesQueryPagesPageInfo to a Pagination
    ///
    /// **Arguments**
    ///
    /// * `page_info` - The GraphQL page info to convert
    ///
    /// **Returns**
    ///
    /// * `Pagination` - The converted Pagination
    fn from(page_info: PagesQueryPagesPageInfo) -> Self {
        Self {
            end_cursor: page_info.end_cursor,
            has_next_page: page_info.has_next_page,
        }
    }
}
