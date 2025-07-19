use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{entity::EntityController, product::ProductController},
    models::product::Product,
    views::components::{
        common::loader::LoaderComponent,
        product::{
            add_to_cart_form::AddToCartForm, details::ProductDetails,
            image_gallery::ProductImageGallery,
        },
    },
};

/// Product page component
#[component]
pub fn ProductPage(product_slug: String) -> Element {
    // Fetch the product
    let slug_for_resource: String = product_slug.clone();
    let product_resource: Resource<Result<Product, AppError>> = use_resource(move || {
        let slug_for_async_operation = slug_for_resource.clone();
        async move {
            ProductController::new()
                .get_by_slug(&slug_for_async_operation)
                .await
        }
    });

    let rendered = match &*product_resource.read() {
        Some(Ok(product)) => {
            let product = product.clone();
            rsx! {
                section { class: "py-20",
                    div { class: "container mx-auto px-4",
                        div { class: "flex flex-wrap -mx-4 mb-24",
                            ProductImageGallery { product: product.clone() }
                            ProductDetails { product: product.clone() }
                            AddToCartForm { product: product.clone() }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => rsx! {
            p { "Error: {e}" }
        },
        None => rsx! {
            LoaderComponent {}
        },
    };
    rendered
}
