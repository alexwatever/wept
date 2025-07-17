use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{
        category::CategoryController, common::EntityController, page::PageController,
        post::PostController, product::ProductController,
    },
    models::{category::ProductCategories, page::Pages, post::Posts, product::Products},
    views::components::common::{
        entity_list::{EntityDisplayListComponent, EntityList},
        loader::LoaderComponent,
    },
};

macro_rules! render_entity_section {
    ($rsx:expr, $title:expr, $resource:expr) => {
        $rsx.push(rsx! {
            section {
                header {
                    h2 { class: "text-2xl font-semibold my-3", $title }
                }
                match &*$resource.read() {
                    Some(Ok(data)) => {
                        let entities = data.as_slice().to_vec();
                        if entities.is_empty() {
                            rsx! { p { "No items found." } }
                        } else {
                            rsx! { EntityDisplayListComponent { entities: entities } }
                        }
                    }
                    Some(Err(app_error)) => rsx! {
                        p { class: "text-red-600", "Error loading data: {app_error.public_message}" }
                    },
                    None => rsx! { LoaderComponent {} },
                }
            }
        })
    };
}

/// Home page component
#[component]
pub fn HomePage() -> Element {
    let posts_resource: Resource<Result<Posts, AppError>> =
        use_resource(move || async move { PostController::new().get_list(Some(3), None).await });
    let pages_resource: Resource<Result<Pages, AppError>> =
        use_resource(move || async move { PageController::new().get_list(Some(3), None).await });
    let products_resource: Resource<Result<Products, AppError>> =
        use_resource(move || async move { ProductController::new().get_list(Some(3), None).await });
    let categories_resource: Resource<Result<ProductCategories, AppError>> = use_resource(
        move || async move { CategoryController::new().get_list(Some(3), None).await },
    );

    let mut sections = Vec::new();
    render_entity_section!(sections, "Latest Posts", posts_resource);
    render_entity_section!(sections, "Pages", pages_resource);
    render_entity_section!(sections, "Featured Products", products_resource);
    render_entity_section!(sections, "Featured Categories", categories_resource);

    rsx! {
        div { class: "container mx-auto p-4",
            for section in sections {
                {section}
            }
        }
    }
}
