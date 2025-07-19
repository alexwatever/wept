use dioxus::prelude::*;

// Modules
use crate::views::{
    layouts::MainLayout,
    pages::{
        cart::CartPage, categories::CategoriesPage, category::CategoryPage, errors::NotFoundPage,
        home::HomePage, list::PagesListPage, page::PagePage, post::PostPage, posts::PostsPage,
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

    /// Route for the Cart page.
    #[route("/cart")]
    CartPage {},

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

    /// Route for the Posts page.
    #[route("/posts")]
    PostsPage {},

    /// Route for the Search page.
    #[route("/search/:query")]
    SearchPage { query: String },

    /// Route for the Page page. Must be last before NotFoundPage.
    #[route("/:slug")]
    PagePage { slug: String },

    /// Catch-all route (404).
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
