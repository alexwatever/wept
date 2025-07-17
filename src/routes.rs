use dioxus::prelude::*;

// Modules
use crate::views::{
    layouts::MainLayout,
    pages::{
        categories::CategoriesPage, category::CategoryPage, errors::NotFoundPage, home::HomePage,
        list::PagesListPage, page::PagePage, post::PostPage, posts::PostsPage,
        product::ProductPage,
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

    /// Catch-all route (404).
    #[route("/:..route")]
    NotFoundPage { route: Vec<String>, message: Option<String> },
}
