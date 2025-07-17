use crate::{
    models::{pagination::Pagination, product::Product},
    views::components::{common::loader::LoaderComponent, product::product_card::ProductCard},
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProductGridProps {
    products: Vec<Product>,
    page_info: Option<Pagination>,
    is_loading: bool,
    on_load_more: EventHandler<()>,
}

#[component]
pub fn ProductGrid(props: ProductGridProps) -> Element {
    let products_to_display = &props.products;
    let show_load_more_button = props.page_info.as_ref().is_some_and(|pi| pi.has_next_page);
    let is_loading_more = props.is_loading;

    rsx! {
        if products_to_display.is_empty() && is_loading_more {
            LoaderComponent {}
        } else if products_to_display.is_empty() {
            div { class: "text-center py-8",
                p { class: "text-lg text-gray-500", "No products found." }
            }
        } else {
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8",
                for product_item in products_to_display.iter() {
                    ProductCard { product: product_item.clone() }
                }
            }
            if is_loading_more {
                div { class: "flex justify-center items-center py-8", LoaderComponent {} }
            }
            if show_load_more_button && !is_loading_more {
                div { class: "text-center mt-12 py-8",
                    button {
                        class: "px-8 py-3 bg-indigo-600 text-white font-semibold rounded-lg shadow-md hover:bg-indigo-700",
                        onclick: move |_| props.on_load_more.call(()),
                        "Load More Products"
                    }
                }
            } else if !is_loading_more && !products_to_display.is_empty() && !show_load_more_button {
                div { class: "text-center mt-12 py-8",
                    p { class: "text-lg text-gray-500", "All products have been loaded." }
                }
            }
        }
    }
}
