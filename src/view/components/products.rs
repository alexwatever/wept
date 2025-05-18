use dioxus::prelude::*;

// # Modules
use super::entity_list::{EntityDisplay, EntityDisplayListComponent};
use crate::model::products::{Product, Products};

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
    let items: Signal<Vec<Product>> = Signal::new(products.read().0.clone());

    rsx! {
        EntityDisplayListComponent {
            entities: items
        }
    }
}

impl EntityDisplay for Product {
    fn render(&self) -> Element {
        let Product {
            name,
            slug,
            description,
            image,
            simple_product,
            ..
        } = self.clone();

        let price = simple_product.as_ref().and_then(|s| s.price.clone());

        rsx! {
            section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
                // Display product image if available
                {
                    if let Some(img) = image {
                        rsx! {
                            img {
                                class: "object-scale-down w-1/6 h-full",
                                src: "{img.source_url.clone().unwrap_or_default()}",
                                alt: "{img.alt_text.clone().unwrap_or_default()}"
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "w-1/6 h-full bg-gray-200 flex items-center justify-center",
                                "No Image"
                            }
                        }
                    }
                }
                div { class: "pl-4 text-left text-ellipsis",
                    a {
                        href: "/product/{slug.clone().unwrap_or_default()}",
                        class: "w-full text-center font-bold text-xl",
                        "{name.clone().unwrap_or_default()}"
                    }
                    {
                        if let Some(price_str) = price {
                            rsx! {
                                p {
                                    class: "w-full text-sm font-bold text-green-600",
                                    "{price_str}"
                                }
                            }
                        } else {
                            rsx! { "" }
                        }
                    }
                    p {
                        class: "w-full text-sm overflow-hidden line-clamp-3",
                        "{description.clone().unwrap_or_default()}"
                    }
                }
            }
        }
    }
}
