use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{
        common::EntityController, post_controller::PostController,
        product_controller::ProductController,
    },
    models::{post::Posts, product::Products},
    views::components::common::{entity_list::EntityDisplayListComponent, loader::LoaderComponent},
};

/// Home page component
#[component]
pub fn HomePage() -> Element {
    // Fetch posts
    let posts_data: Resource<Result<Posts, AppError>> = use_resource(move || {
        let controller: PostController = PostController::new();
        async move { controller.get_list(Some(3), None).await }
    });

    // Fetch products
    let products_data: Resource<Result<Products, AppError>> = use_resource(move || {
        let controller: ProductController = ProductController::new();
        async move { controller.get_list(Some(3), None).await }
    });

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold my-4", "Welcome to Wept" }

            // Display Posts
            h2 { class: "text-2xl font-semibold my-3", "Latest Posts" }
            {
                match &*posts_data.read() {
                    Some(Ok(Posts(posts_vec))) => {
                        if posts_vec.is_empty() {
                            rsx! { p { "No posts found." } }
                        } else {
                            rsx! { EntityDisplayListComponent { entities: posts_vec.clone() } }
                        }
                    }
                    Some(Err(app_error)) => rsx! {
                        p { class: "text-red-600", "Error loading posts: {app_error.public_message}" }
                    },
                    None => rsx! { LoaderComponent {} },
                }
            }

            // Display Products
            h2 { class: "text-2xl font-semibold my-3", "Featured Products" }
            {
                match &*products_data.read() {
                    Some(Ok(Products(products_vec))) => {
                        if products_vec.is_empty() {
                            rsx! { p { "No products found." } }
                        } else {
                            rsx! { EntityDisplayListComponent { entities: products_vec.clone() } }
                        }
                    }
                    Some(Err(app_error)) => rsx! {
                        p { class: "text-red-600", "Error loading products: {app_error.public_message}" }
                    },
                    None => rsx! { LoaderComponent {} },
                }
            }
        }
    }
}
