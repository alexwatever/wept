use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{common::EntityController, product::ProductController},
    models::product::{Product, ProductImage, SimpleProduct},
    views::components::common::loader::LoaderComponent,
    views::pages::errors::NotFoundPage,
};

/// Product page component
#[component]
pub fn ProductPage(product_slug: String) -> Element {
    // Fetch the product
    let slug_for_resource: String = product_slug.clone();
    let product_resource: Resource<Result<Product, AppError>> = use_resource(move || {
        let slug_for_async_operation: String = slug_for_resource.clone();
        async move {
            ProductController::new()
                .get_by_slug(&slug_for_async_operation)
                .await
        }
    });

    // Wait for product data
    let product_data = product_resource.read();
    match product_data.as_ref() {
        // Loading state
        None => rsx! { LoaderComponent {} },
        // Error state
        Some(Err(app_error)) => app_error.render(vec!["product".to_string(), product_slug]),
        // Product found
        Some(Ok(product)) => {
            // Get Product values
            let Product {
                sku,
                name,
                short_description,
                image,
                gallery_images,
                simple_product,
                ..
            } = product;

            // Get SimpleProduct values
            let SimpleProduct {
                price,
                sale_price,
                regular_price,
                on_sale,
                stock_status,
                stock_quantity,
                ..
            } = match simple_product {
                Some(simple_product) => simple_product,
                None => {
                    return rsx! {
                        NotFoundPage { route: vec!["product".to_string(), product_slug], message: Some("Simple product data not found".to_string()) }
                    };
                }
            };

            // Product name
            let name: String = name.clone().unwrap_or_default();

            // Product image
            let image: Option<String> = image
                .as_ref()
                .and_then(|img: &ProductImage| img.source_url.clone());

            // Produce price
            let price: Option<String> = if on_sale.unwrap_or(false) {
                if let (Some(regular_price), Some(sale_price)) =
                    (regular_price.as_ref(), sale_price.as_ref())
                {
                    Some(format!("{} (Sale: {})", regular_price, sale_price))
                } else {
                    None
                }
            } else {
                price.clone()
            };

            // Product stock status
            let stock_info: String = match (stock_status.as_deref(), stock_quantity) {
                (Some("IN_STOCK"), Some(qty)) if qty > &0 => {
                    format!("In Stock ({} available)", qty)
                }
                (Some("IN_STOCK"), _) => "In Stock".to_string(),
                (Some("OUT_OF_STOCK"), _) => "Out of Stock".to_string(),
                (Some("ON_BACKORDER"), _) => "Available on Backorder".to_string(),
                _ => "Status Unknown".to_string(),
            };

            // Render page
            rsx! {
                section { class: "py-20",
                    div { class: "container mx-auto px-4",
                        div { class: "flex flex-wrap -mx-4 mb-24",
                            div { class: "w-full md:w-1/2 px-4 mb-8 md:mb-0",
                                div { class: "relative mb-10", style: "height: 564px;",
                                    // Image
                                    if let Some(image) = image {
                                        img {
                                            class: "object-cover w-full h-full",
                                            alt: "{name.clone()}",
                                            src: "{image}",
                                        }
                                    }
                                }

                                // Gallery thumbnails
                                if let Some(gallery) = gallery_images.as_ref() {
                                    div { class: "flex -mx-2",
                                        for img in gallery.iter().take(4) {
                                            if let Some(source_url) = img.source_url.as_ref() {
                                                div { class: "w-1/4 px-2",
                                                    img {
                                                        class: "w-full h-24 object-cover rounded",
                                                        src: "{source_url}",
                                                        alt: "{img.alt_text.as_ref().map(|text| text.clone()).unwrap_or_default()}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "w-full md:w-1/2 px-4",
                                div { class: "lg:pl-20",
                                    div { class: "mb-10 pb-10 border-b",
                                        // Product name
                                        h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                            "{name}"
                                        }

                                        // Price
                                        if let Some(price) = price {
                                            p { class: "inline-block mb-4 text-2xl font-bold font-heading text-blue-500",
                                                "{price}"
                                            }
                                        }

                                        // Stock status
                                        p { class: "mb-8 text-sm",
                                            span {
                                                class: if stock_status.as_deref() == Some("IN_STOCK") { "text-green-600 font-semibold" } else { "text-red-600 font-semibold" },
                                                "{stock_info}"
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

                                    // Add to cart
                                    div { class: "flex flex-wrap -mx-4 mb-14 items-center",
                                        div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
                                            a { class: "block bg-orange-300 hover:bg-orange-400 text-center text-white font-bold font-heading py-5 px-8 rounded-md uppercase transition duration-200",
                                                href: "#",
                                                "Add to cart"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
