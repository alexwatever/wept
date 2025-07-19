use dioxus::prelude::*;

use crate::models::product::Product;

#[component]
pub fn ProductDetails(product: Product) -> Element {
    let Product {
        name,
        simple_product,
        short_description,
        sku,
        ..
    } = product;

    let name = name.unwrap_or_default();
    let simple_product = simple_product.unwrap();

    rsx! {
        div { class: "w-full md:w-1/2 px-4",
            div { class: "lg:pl-20",
                div { class: "mb-10 pb-10 border-b",
                    // Product name
                    h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                        "{name}"
                    }

                    // Price
                    if let Some(price) = simple_product.price() {
                        p { class: "inline-block mb-4 text-2xl font-bold font-heading text-blue-500",
                            "{price}"
                        }
                    }

                    // Stock status
                    p { class: "mb-8 text-sm",
                        span {
                            class: if simple_product.is_in_stock() {
                                "text-green-600 font-semibold"
                            } else {
                                "text-red-600 font-semibold"
                            },
                            "{simple_product.stock_info()}"
                        }
                    }

                    // Short description
                    if let Some(short_desc) = short_description.as_ref() {
                        p { class: "max-w-md mb-8 text-gray-500",
                            "{short_desc}"
                        }
                    }

                    // SKU
                    if let Some(sku) = sku.as_ref() {
                        p { class: "text-sm text-gray-400",
                            "SKU: {sku}"
                        }
                    }
                }
            }
        }
    }
}
