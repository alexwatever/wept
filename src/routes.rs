use dioxus::prelude::*;

// # Modules
use crate::view::pages::{errors::NotFoundPage, home::HomePage, post::PostPage, product::ProductPage};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub(crate) enum Routes {
    #[route("/")]
    HomePage {},

    #[route("/products/:product_id")]
    ProductPage { product_id: usize },
    
    #[route("/posts/:post_id")]
    PostPage { post_id: String },
    
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
