use dioxus::prelude::*;
use tracing::{error, info};

// # Modules
use super::super::components::{posts::PostsComponent, products::ProductsComponent};
use crate::{
    controller::Controller,
    model::{
        pagination::PageSort, posts::Posts as PostsModel, products::Products as ProductsModel,
    },
};

#[allow(non_snake_case)]
pub(crate) fn HomePage() -> Element {
    let mut products: Signal<ProductsModel> = use_signal(|| ProductsModel(vec![]));
    let mut posts: Signal<PostsModel> = use_signal(|| PostsModel(vec![]));

    use_effect(move || {
        info!("Home page mounted (from use_effect, runs once)");
    });

    // Get products asyncronously
    let _get_products: Resource<()> = use_resource(move || async move {
        let update: ProductsModel = ProductsModel::get_page(Some(10), Some(PageSort::Ascending))
            .await
            .unwrap_or_else(|err| {
                error!("Error fetching products: {err}");
                ProductsModel(vec![])
            });

        // Update the `products` signal
        products.set(update);
    });

    // Get posts asyncronously
    let _get_posts: Resource<()> = use_resource(move || async move {
        let update: PostsModel = PostsModel::get_page(Some(10), Some(PageSort::Ascending))
            .await
            .unwrap_or_else(|err| {
                error!("Error fetching posts: {err}");
                PostsModel(vec![])
            });

        // Update the `posts` signal
        posts.set(update);
    });

    // # Render page
    rsx! {
        div { class: "container mx-auto",
            h1 { class: "text-3xl font-bold text-center my-8", "Products" }
            ProductsComponent { products }

            h1 { class: "text-3xl font-bold text-center my-8", "Posts" }
            PostsComponent { posts }
        }
    }
}
