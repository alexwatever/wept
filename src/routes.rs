use dioxus::prelude::*;

// Modules
use crate::views::{
    layouts::MainLayout,
    pages::{
        categories::CategoriesPage, category::CategoryPage, errors::NotFoundPage, home::HomePage,
        list::PagesListPage, page::PagePage, post::PostPage, posts::PostsPage,
        product::ProductPage, search::SearchPage,
    },
};

/// Routes
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(clippy::enum_variant_names)]
#[rustfmt::skip]
pub enum Routes {
    /// Base layout
    #[layout(MainLayout)]
    /// Route for the Home page.
    #[route("/")]
    HomePage {},

    /// Route for the Product page.
    #[route("/product/:product_slug")]
    ProductPage { product_slug: String },
    
    /// Route for the Post page.
    #[route("/post/:post_slug")]
    PostPage { post_slug: String },

    /// Route for the Categories page.
    #[route("/categories")]
    CategoriesPage {},

    /// Route for the Category page.
    #[route("/category/:slug")]
    CategoryPage { slug: String },
    
    /// Route for the Pages page.
    #[route("/pages")]
    PagesListPage {},

    /// Route for the Page page.
    #[route("/page/:slug")]
    PagePage { slug: String },

    /// Route for the Posts page.
    #[route("/posts")]
    PostsPage {},

    /// Route for the Search page.
    #[route("/search/:query")]
    SearchPage { query: String },

    /// Catch-all route (404).
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

impl Routes {
    pub fn from_path(path: &str) -> Self {
        let path = path.trim_start_matches('/');
        let mut segments = path.split('/');

        match segments.next() {
            Some("product") => {
                if let Some(slug) = segments.next() {
                    Routes::ProductPage {
                        product_slug: slug.to_string(),
                    }
                } else {
                    Routes::NotFoundPage {
                        route: vec![path.to_string()],
                    }
                }
            }
            Some("post") => {
                if let Some(slug) = segments.next() {
                    Routes::PostPage {
                        post_slug: slug.to_string(),
                    }
                } else {
                    Routes::NotFoundPage {
                        route: vec![path.to_string()],
                    }
                }
            }
            Some("category") => {
                if let Some(slug) = segments.next() {
                    Routes::CategoryPage {
                        slug: slug.to_string(),
                    }
                } else {
                    Routes::CategoriesPage {}
                }
            }
            Some("categories") => Routes::CategoriesPage {},
            Some("page") => {
                if let Some(slug) = segments.next() {
                    Routes::PagePage {
                        slug: slug.to_string(),
                    }
                } else {
                    Routes::PagesListPage {}
                }
            }
            Some("pages") => Routes::PagesListPage {},
            Some("posts") => Routes::PostsPage {},
            Some("search") => {
                if let Some(query) = segments.next() {
                    Routes::SearchPage {
                        query: query.to_string(),
                    }
                } else {
                    Routes::HomePage {}
                }
            }
            Some("") | None => Routes::HomePage {},
            _ => Routes::NotFoundPage {
                route: path.split('/').map(String::from).collect(),
            },
        }
    }
}
