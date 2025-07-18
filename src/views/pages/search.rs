use dioxus::prelude::*;

// Modules
use crate::{
    controllers::product::ProductController, views::components::product::product_grid::ProductGrid,
};

/// Search page component
#[component]
pub fn SearchPage(query: String) -> Element {
    // When the `query` prop changes, update the signal
    let mut query_signal = use_signal(|| query.clone());
    if *query_signal.read() != query {
        query_signal.set(query.clone());
    }

    // Initialize the search results resource
    let search_results = ProductController::new();
    let search_results = use_resource(move || {
        let search_results = search_results.clone();
        async move { search_results.search_products(&query_signal.read()).await }
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
