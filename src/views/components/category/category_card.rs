use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{
    models::category::{ProductCategory, ProductCategoryImage},
    routes::Routes,
};

/// Category card component
///
/// Displays a category within a list
///
/// **Arguments**  
///
/// * `category` - The category to display
///
/// **Returns**  
///
/// A component that displays the category
#[component]
pub fn ProductCategoryCard(category: ProductCategory) -> Element {
    // Get the image URL
    let image_url: String = category
        .image
        .as_ref()
        .and_then(|img: &ProductCategoryImage| img.source_url.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "p-4 w-full md:w-1/3",
            div { class: "rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-shadow duration-300 ease-in-out",
                // Image
                img { class: "w-full h-64 object-cover",
                    src: "{image_url}",
                    alt: "{category.name.as_ref().unwrap_or(&String::new())}"
                }

                div { class: "p-6",
                    // Name
                    if let Some(name) = &category.name {
                        h2 { class: "text-xl font-bold mb-2",
                            "{name}"
                        }
                    }

                    // View category
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
    }
}
