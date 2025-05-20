use dioxus::prelude::*;

// Modules
use crate::views::{
    layouts::MainLayout,
    pages::{errors::NotFoundPage, home::HomePage, post::PostPage, product::ProductPage},
};

/// Routes
#[derive(Clone, Routable, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
#[rustfmt::skip]
pub enum Routes {
    /// Apply the base layout.
    #[layout(MainLayout)]

    /// Route for the Home page.
    #[route("/")]
    HomePage {},

    /// Route for a Product page.
    #[route("/product/:product_slug")]
    ProductPage { product_slug: String },
    
    /// Route for a Post page.
    #[route("/post/:post_slug")]
    PostPage { post_slug: String },
    
    /// Catch-all route (404).
    #[route("/:..route")]
    NotFoundPage { route: Vec<String>, message: Option<String> },
}
