use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{
        category::CategoryController, common::EntityController, post::PostController,
        product::ProductController,
    },
    models::{category::ProductCategories, post::Posts, product::Products},
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

    // Fetch categories
    let categories_data: Resource<Result<ProductCategories, AppError>> = use_resource(move || {
        let controller: CategoryController = CategoryController::new();
        async move { controller.get_list(Some(3), None).await }
    });

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold my-4", "Welcome to Wept" }

            // Display Posts
            h2 { class: "text-2xl font-semibold my-3", "Latest Posts" }
            {
                match &*posts_data.read() {
                    Some(Ok(Posts { posts, .. })) => {
                        if posts.is_empty() {
                            rsx! { p { "No posts found." } }
                        } else {
                            rsx! { EntityDisplayListComponent { entities: posts.clone() } }
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
                    Some(Ok(Products { products, .. })) => {
                        if products.is_empty() {
                            rsx! { p { "No products found." } }
                        } else {
                            rsx! { EntityDisplayListComponent { entities: products.clone() } }
                        }
                    }
                    Some(Err(app_error)) => rsx! {
                        p { class: "text-red-600", "Error loading products: {app_error.public_message}" }
                    },
                    None => rsx! { LoaderComponent {} },
                }
            }

            // Display Categories
            h2 { class: "text-2xl font-semibold my-3", "Featured Categories" }
            {
                match &*categories_data.read() {
                    Some(Ok(ProductCategories { categories, .. })) => {
                        rsx! { EntityDisplayListComponent { entities: categories.clone() } }
                    }
                    Some(Err(app_error)) => rsx! {
                        p { class: "text-red-600", "Error loading categories: {app_error.public_message}" }
                    },
                    None => rsx! { LoaderComponent {} },
                }
            }
        }
    }
}
