use dioxus::prelude::*;

use crate::models::product::{Product, ProductImage};

#[component]
pub fn ProductImageGallery(product: Product) -> Element {
    let Product {
        name,
        image,
        gallery_images,
        ..
    } = product;

    let name = name.unwrap_or_default();
    let image = image
        .as_ref()
        .and_then(|img: &ProductImage| img.source_url.clone());

    rsx! {
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
    }
}
