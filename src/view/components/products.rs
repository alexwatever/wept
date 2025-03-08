use dioxus::prelude::*;

// # Modules
use crate::{
    model::product::{Product, Products},
    view::components::{loader::LoaderComponent, product_item::product_item},
};

/// # Products Component
///
/// This component displays a list of products.  
///
/// **Arguments**  
///
/// * `products` - A signal containing a list of products.
///
/// **Returns**  
///
/// The ProductsComponent element.
#[component]
#[allow(non_snake_case)]
pub(crate) fn ProductsComponent(products: Signal<Products>) -> Element {
    let items: &Vec<Product> = &products.read().0;

    // Render products
    if !items.is_empty() {
        rsx! {
            section { class: "p-10",
                for product in &items {
                    product_item {
                        product: product.clone(),
                    }
                }
            }
        }
    } else {
        // Render empty/loading state
        rsx! {
            section { class: "p-10",
                section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
                    img {
                        class: "object-scale-down w-1/6 h-full",
                        alt: "...",
                    }
                    div { class: "pl-4 text-left text-ellipsis",
                        LoaderComponent {}
                    }
                }
            }
        }
    }
}
