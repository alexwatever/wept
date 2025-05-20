use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{models::product::Product, routes::Routes};

/// Product card component
///
/// Displays a product within a list.
///
/// **Arguments**  
///
/// * `product` - The product to display.
///
/// **Returns**  
///
/// A component that displays the product.
#[component]
pub fn ProductCard(product: Product) -> Element {
    let image_url = product
        .image
        .as_ref()
        .and_then(|img| img.source_url.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "p-4 w-full md:w-1/3",
            div { class: "rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-shadow duration-300 ease-in-out",
                // Image
                img { class: "w-full h-64 object-cover",
                    src: "{image_url}",
                    alt: "{product.name.as_ref().unwrap_or(&String::new())}"
                }

                div { class: "p-6",
                    // Name
                    if let Some(name) = &product.name {
                        h2 { class: "text-xl font-bold mb-2",
                            "{name}"
                        }
                    }

                    // Price
                    if let Some(price) = &product.simple_product.map(|p| p.price) {
                        if let Some(price) = price.as_ref() {
                            p { class: "text-blue-500 font-bold",
                                "{price}"
                            }
                        }
                    }

                    // View product
                    if let Some(slug) = &product.slug {
                        div { class: "mt-4",
                            Link {
                                    class: "inline-block px-6 py-2 bg-blue-500 text-white font-medium text-sm rounded hover:bg-blue-600 transition duration-300 ease-in-out",
                                to: Routes::ProductPage { product_slug: slug.clone() },
                                "View Product"
                            }
                        }
                    }
                }
            }
        }
    }
}
