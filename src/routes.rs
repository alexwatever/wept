use dioxus::prelude::*;

// # Modules
use crate::view::pages::{
    errors::NotFoundPage, home::HomePage, post::PostPage, product::ProductPage,
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub(crate) enum Routes {
    #[route("/")]
    HomePage {},

    #[route("/product/:product_slug")]
    ProductPage { product_slug: String },
    
    #[route("/post/:post_slug")]
    PostPage { post_slug: String },
    
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
