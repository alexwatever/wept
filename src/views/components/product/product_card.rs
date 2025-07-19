use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{
    models::product::{Product, ProductImage, ProductSimpleProduct},
    routes::Routes,
    views::components::common::card::Card,
};

/// Product card component
#[component]
pub fn ProductCard(product: Product) -> Element {
    // Get the image URL
    let image_url: String = product
        .image
        .as_ref()
        .and_then(|img: &ProductImage| img.source_url.clone())
        .unwrap_or_default();

    rsx! {
        Card {
            title: product.name.clone().unwrap_or_default(),
            img {
                class: "w-full h-64 object-cover",
                src: "{image_url}",
                alt: "{product.name.as_ref().unwrap_or(&String::new())}"
            }
            if let Some(ProductSimpleProduct::SimpleProduct(simple_product)) = &product.simple_product {
                if let Some(price) = simple_product.price.as_ref() {
                    p { class: "text-blue-500 font-bold",
                        "{price}"
                    }
                }
            }
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
