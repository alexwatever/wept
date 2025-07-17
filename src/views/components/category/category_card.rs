use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{
    models::category::{ProductCategory, ProductCategoryImage},
    routes::Routes,
    views::components::common::card::Card,
};

/// Category card component
#[component]
pub fn ProductCategoryCard(category: ProductCategory) -> Element {
    // Get the image URL
    let image_url: String = category
        .image
        .as_ref()
        .and_then(|img: &ProductCategoryImage| img.source_url.clone())
        .unwrap_or_default();

    rsx! {
        Card {
            title: category.name.clone().unwrap_or_default(),
            img {
                class: "w-full h-64 object-cover",
                src: "{image_url}",
                alt: "{category.name.as_ref().unwrap_or(&String::new())}"
            }
            if let Some(slug) = &category.slug {
                div { class: "mt-4",
                    Link {
                        class: "inline-block px-6 py-2 bg-blue-500 text-white font-medium text-sm rounded hover:bg-blue-600 transition duration-300 ease-in-out",
                        to: Routes::CategoryPage { slug: slug.clone() },
                        "View Category"
                    }
                }
            }
        }
    }
}
