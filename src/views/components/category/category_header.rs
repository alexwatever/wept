use crate::models::category::{ProductCategory, ProductCategoryImage};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryHeaderProps {
    category: ProductCategory,
}

#[component]
pub fn CategoryHeader(props: CategoryHeaderProps) -> Element {
    let category_name = props
        .category
        .name
        .as_ref()
        .cloned()
        .unwrap_or_else(|| "Unnamed Category".to_string());
    let category_description = props
        .category
        .description
        .as_ref()
        .cloned()
        .unwrap_or_default();
    let category_image_url = props
        .category
        .image
        .as_ref()
        .and_then(|img: &ProductCategoryImage| img.source_url.clone());
    let total_product_count = props
        .category
        .count
        .map_or_else(|| "N/A".to_string(), |c| c.to_string());

    rsx! {
        div { class: "text-center mb-12",
            if let Some(img_url) = category_image_url {
                div { class: "mb-6 w-48 h-48 mx-auto rounded-lg overflow-hidden shadow-lg",
                    img { class: "object-cover w-full h-full", src: "{img_url}", alt: "{category_name}" }
                }
            }
            h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "{category_name}" }
            p { class: "text-lg text-gray-600 mt-2", "Total products: {total_product_count}" }
        }
        if !category_description.is_empty() {
            div { class: "max-w-3xl mx-auto bg-white p-8 rounded-lg shadow-md mb-12",
                div { class: "prose lg:prose-xl", dangerous_inner_html: "{category_description}" }
            }
        } else {
            div { class: "text-center text-gray-500 py-8 mb-12", "No description available for this category." }
        }
    }
}
