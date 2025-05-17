use dioxus::prelude::*;

// # Modules
use crate::{controller::products::fetch_product, model::products::Product};

#[component]
#[allow(non_snake_case)]
pub(crate) fn ProductPage(product_slug: ReadOnlySignal<String>) -> Element {
    let product = use_resource(move || async move { fetch_product(product_slug()).await });

    let product_data = product.read();
    match product_data.as_ref() {
        None => rsx! { // Loading state
            div { class: "container mx-auto px-4 py-20 flex justify-center",
                div { class: "animate-pulse",
                    h2 { class: "text-2xl font-bold", "Loading Product..." }
                }
            }
        },
        Some(Err(error)) => rsx! { // Error state
            div { class: "container mx-auto px-4 py-20",
                h2 { class: "text-3xl font-bold text-red-500", "Error Loading Product" }
                p { class: "text-lg", "There was an error loading the product: {error}" }
                a {
                    class: "mt-4 inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    href: "/",
                    "Return to Home"
                }
            }
        },
        Some(Ok(p)) => {
            let Product {
                sku,
                name,
                description,
                short_description,
                image,
                gallery_images,
                simple_product,
                ..
            } = p;

            if simple_product.is_none() {
                return rsx! {
                    div { class: "container mx-auto px-4 py-20",
                        h2 { class: "text-3xl font-bold text-red-500", "Product Not Available" }
                        p { class: "text-lg", "This product is not available or has incomplete data." }
                        a {
                            class: "mt-4 inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            href: "/",
                            "Return to Home"
                        }
                    }
                };
            }

            // Get SimpleProduct values
            let simple_product_data = simple_product.as_ref().unwrap();
            let price = &simple_product_data.price;
            let sale_price = &simple_product_data.sale_price;
            let regular_price = &simple_product_data.regular_price;
            let on_sale = simple_product_data.on_sale;
            let stock_status = &simple_product_data.stock_status;
            let stock_quantity = simple_product_data.stock_quantity;
            let weight = &simple_product_data.weight;

            // Product image source
            let image_src = image
                .as_ref()
                .and_then(|img| img.source_url.clone())
                .unwrap_or_default();

            // Format price display
            let price_display = if on_sale.unwrap_or(false) {
                format!(
                    "{} (Sale: {})",
                    regular_price.as_ref().unwrap_or(&String::new()),
                    sale_price.as_ref().unwrap_or(&String::new())
                )
            } else {
                price.as_ref().unwrap_or(&String::new()).clone()
            };

            // Format stock status
            let stock_info = match (stock_status.as_deref(), stock_quantity) {
                (Some("IN_STOCK"), Some(qty)) if qty > 0 => {
                    format!("In Stock ({} available)", qty)
                }
                (Some("IN_STOCK"), _) => "In Stock".to_string(),
                (Some("OUT_OF_STOCK"), _) => "Out of Stock".to_string(),
                (Some("ON_BACKORDER"), _) => "Available on Backorder".to_string(),
                _ => "Status Unknown".to_string(),
            };

            rsx! {
                section { class: "py-20",
                    div { class: "container mx-auto px-4",
                        div { class: "flex flex-wrap -mx-4 mb-24",
                            div { class: "w-full md:w-1/2 px-4 mb-8 md:mb-0",
                                div { class: "relative mb-10",
                                    style: "height: 564px;",
                                    a { class: "absolute top-1/2 left-0 ml-8 transform translate-1/2",
                                        href: "#",
                                        icons::icon_0 {}
                                    }
                                    img {
                                        class: "object-cover w-full h-full",
                                        alt: "{name.clone().unwrap_or_default()}",
                                        src: "{image_src}",
                                    }
                                    a { class: "absolute top-1/2 right-0 mr-8 transform translate-1/2",
                                        href: "#",
                                        icons::icon_1 {}
                                    }
                                }
                                // Gallery thumbnails if available
                                {
                                    if let Some(gallery) = gallery_images.as_ref() {
                                        rsx! {
                                            div { class: "flex -mx-2",
                                                for img in gallery.iter().take(4) {
                                                    div { class: "w-1/4 px-2",
                                                        img {
                                                            class: "w-full h-24 object-cover rounded",
                                                            src: "{img.source_url.clone().unwrap_or_default()}",
                                                            alt: "{img.alt_text.clone().unwrap_or_default()}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! { "" }
                                    }
                                }
                            }
                            div { class: "w-full md:w-1/2 px-4",
                                div { class: "lg:pl-20",
                                    div { class: "mb-10 pb-10 border-b",
                                        h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                            "{name.clone().unwrap_or_default()}"
                                        }
                                        // Price
                                        p { class: "inline-block mb-4 text-2xl font-bold font-heading text-blue-500",
                                            "{price_display}"
                                        }
                                        // Stock status
                                        p { class: "mb-8 text-sm",
                                            span {
                                                class: if stock_status.as_deref() == Some("IN_STOCK") { "text-green-600 font-semibold" } else { "text-red-600 font-semibold" },
                                                "{stock_info}"
                                            }
                                        }
                                        // Short description
                                        {
                                            if let Some(short_desc) = short_description.as_ref() {
                                                rsx! {
                                                    p { class: "max-w-md mb-8 text-gray-500",
                                                        "{short_desc}"
                                                    }
                                                }
                                            } else {
                                                rsx! { "" }
                                            }
                                        }
                                        // Product SKU
                                        {
                                            if let Some(sku_val) = sku.as_ref() {
                                                rsx! {
                                                    p { class: "text-sm text-gray-400",
                                                        "SKU: {sku_val}"
                                                    }
                                                }
                                            } else {
                                                rsx! { "" }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-wrap -mx-4 mb-14 items-center",
                                        div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
                                            a { class: "block bg-orange-300 hover:bg-orange-400 text-center text-white font-bold font-heading py-5 px-8 rounded-md uppercase transition duration-200",
                                                href: "#",
                                                "Add to cart"
                                            }
                                        }
                                    }
                                    div { class: "flex items-center",
                                        span { class: "mr-8 text-gray-500 font-bold font-heading uppercase",
                                            "SHARE IT"
                                        }
                                        a { class: "mr-1 w-8 h-8",
                                            href: "#",
                                            img {
                                                alt: "",
                                                src: "https://shuffle.dev/yofte-assets/buttons/facebook-circle.svg",
                                            }
                                        }
                                        a { class: "mr-1 w-8 h-8",
                                            href: "#",
                                            img {
                                                alt: "",
                                                src: "https://shuffle.dev/yofte-assets/buttons/instagram-circle.svg",
                                            }
                                        }
                                        a { class: "w-8 h-8",
                                            href: "#",
                                            img {
                                                src: "https://shuffle.dev/yofte-assets/buttons/twitter-circle.svg",
                                                alt: "",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            ul { class: "flex flex-wrap mb-16 border-b-2",
                                li { class: "w-1/2 md:w-auto",
                                    a { class: "inline-block py-6 px-10 bg-white text-gray-500 font-bold font-heading shadow-2xl",
                                        href: "#",
                                        "Description"
                                    }
                                }
                                li { class: "w-1/2 md:w-auto",
                                    a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                        href: "#",
                                        "Customer reviews"
                                    }
                                }
                                li { class: "w-1/2 md:w-auto",
                                    a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                        href: "#",
                                        "Shipping &amp; returns"
                                    }
                                }
                                li { class: "w-1/2 md:w-auto",
                                    a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
                                        href: "#",
                                        "Brand"
                                    }
                                }
                            }
                            h3 { class: "mb-8 text-3xl font-bold font-heading text-blue-300",
                                "{name.clone().unwrap_or_default()} Details"
                            }
                            // Full description
                            div { class: "max-w-2xl text-gray-500 mb-10",
                                dangerous_inner_html: "{description.clone().unwrap_or_default()}"
                            }
                            // Additional details
                            {
                                if weight.is_some() {
                                    rsx! {
                                        div { class: "mb-10",
                                            h4 { class: "mb-4 text-xl font-bold", "Specifications" }
                                            table { class: "w-full border-collapse",
                                                tbody {
                                                    // Weight
                                                    if let Some(w) = weight.as_ref() {
                                                        tr { class: "border-b",
                                                            td { class: "py-2 pr-4 font-semibold", "Weight" }
                                                            td { class: "py-2 pl-4", "{w}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! { "" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

mod icons {
    use super::*;

    pub(super) fn icon_0() -> Element {
        rsx! {
            svg { class: "w-6 h-6",
                view_box: "0 0 24 23",
                xmlns: "http://www.w3.org/2000/svg",
                height: "23",
                fill: "none",
                width: "24",
                path {
                    stroke: "black",
                    fill: "black",
                    d: "M2.01328 18.9877C2.05682 16.7902 2.71436 12.9275 6.3326 9.87096L6.33277 9.87116L6.33979 9.86454L6.3398 9.86452C6.34682 9.85809 8.64847 7.74859 13.4997 7.74859C13.6702 7.74859 13.8443 7.75111 14.0206 7.757L14.0213 7.75702L14.453 7.76978L14.6331 7.77511V7.59486V3.49068L21.5728 10.5736L14.6331 17.6562V13.6558V13.5186L14.4998 13.4859L14.1812 13.4077C14.1807 13.4075 14.1801 13.4074 14.1792 13.4072M2.01328 18.9877L14.1792 13.4072M2.01328 18.9877C7.16281 11.8391 14.012 13.3662 14.1792 13.4072M2.01328 18.9877L14.1792 13.4072M23.125 10.6961L23.245 10.5736L23.125 10.4512L13.7449 0.877527L13.4449 0.571334V1V6.5473C8.22585 6.54663 5.70981 8.81683 5.54923 8.96832C-0.317573 13.927 0.931279 20.8573 0.946581 20.938L0.946636 20.9383L1.15618 22.0329L1.24364 22.4898L1.47901 22.0885L2.041 21.1305L2.04103 21.1305C4.18034 17.4815 6.71668 15.7763 8.8873 15.0074C10.9246 14.2858 12.6517 14.385 13.4449 14.4935V20.1473V20.576L13.7449 20.2698L23.125 10.6961Z",
                    stroke_width: "0.35",
                }
            }
        }
    }

    pub(super) fn icon_1() -> Element {
        rsx! {
            svg { class: "w-6 h-6",
                height: "27",
                view_box: "0 0 27 27",
                fill: "none",
                width: "27",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M13.4993 26.2061L4.70067 16.9253C3.9281 16.1443 3.41815 15.1374 3.24307 14.0471C3.06798 12.9568 3.23664 11.8385 3.72514 10.8505V10.8505C4.09415 10.1046 4.63318 9.45803 5.29779 8.96406C5.96241 8.47008 6.73359 8.14284 7.54782 8.00931C8.36204 7.87578 9.19599 7.93978 9.98095 8.19603C10.7659 8.45228 11.4794 8.89345 12.0627 9.48319L13.4993 10.9358L14.9359 9.48319C15.5192 8.89345 16.2327 8.45228 17.0177 8.19603C17.8026 7.93978 18.6366 7.87578 19.4508 8.00931C20.265 8.14284 21.0362 8.47008 21.7008 8.96406C22.3654 9.45803 22.9045 10.1046 23.2735 10.8505V10.8505C23.762 11.8385 23.9306 12.9568 23.7556 14.0471C23.5805 15.1374 23.0705 16.1443 22.298 16.9253L13.4993 26.2061Z",
                    stroke: "black",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
