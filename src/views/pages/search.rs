use dioxus::prelude::*;

// Modules
use crate::{
    controllers::product::ProductController, views::components::product::product_grid::ProductGrid,
};

/// Search page component
#[component]
pub fn SearchPage(query: String) -> Element {
    // Initialize the product controller
    let product_controller = ProductController::new();

    // Initialize the search results resource
    let query_for_async = query.clone();
    let search_results = use_resource(move || {
        let product_controller = product_controller.clone();
        let query = query_for_async.clone();
        async move { product_controller.search_products(&query).await }
    });

    rsx! {
        div {
            class: "container mx-auto p-4",

            h1 {
                class: "text-2xl font-bold mb-4",
                "Search Results for: ",
                span {
                    class: "font-normal italic",
                    "{query}"
                }
            }

            match &*search_results.value().read_unchecked() {
                Some(Ok(products)) => {
                    rsx! {
                        ProductGrid {
                            products: products.products.clone(),
                            page_info: None,
                            is_loading: false,
                            on_load_more: |_| {}
                        }
                    }
                }
                Some(Err(e)) => {
                    rsx! {
                        div {
                            class: "text-red-500",
                            "Error: {e}"
                        }
                    }
                }
                None => {
                    rsx! {
                        ProductGrid {
                            products: vec![],
                            page_info: None,
                            is_loading: true,
                            on_load_more: |_| {}
                        }
                    }
                }
            }
        }
    }
}
