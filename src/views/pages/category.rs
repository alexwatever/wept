use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    app::error::AppErrorKind,
    controllers::category::CategoryController,
    models::{category::ProductCategory, product::Product},
    views::{
        components::{
            category::category_header::CategoryHeader, common::loader::LoaderComponent,
            product::product_grid::ProductGrid,
        },
        pages::errors::GenericErrorPage,
    },
};

/// Category page component
#[component]
pub fn CategoryPage(slug: String) -> Element {
    let slug_for_error = slug.clone();
    let mut current_after_cursor = use_signal::<Option<String>>(|| None);
    let mut accumulated_products = use_signal::<Vec<Product>>(Vec::new);
    let mut category_details = use_signal::<Option<ProductCategory>>(|| None);

    let mut category_resource = use_resource(move || {
        let slug_clone = slug.clone();
        let after_clone = current_after_cursor.read().clone();
        async move {
            CategoryController::new()
                .get_with_products(&slug_clone, Some(12), after_clone)
                .await
        }
    });

    use_effect(move || {
        if let Some(Ok(fetched_data)) = category_resource.value().read().as_ref() {
            let products_from_fetch = fetched_data
                .products
                .as_ref()
                .map(|p_struct| p_struct.products.clone())
                .unwrap_or_default();

            let page_info_from_fetch = fetched_data.page_info.clone();

            if current_after_cursor.read().is_none() {
                accumulated_products.set(products_from_fetch);
                let initial_details = ProductCategory {
                    page_info: page_info_from_fetch,
                    ..fetched_data.clone()
                };
                category_details.set(Some(initial_details));
            } else {
                let mut current_prods = accumulated_products.read().clone();
                current_prods.extend(products_from_fetch);
                accumulated_products.set(current_prods);

                if let Some(cd_val) = category_details.write().as_mut() {
                    cd_val.page_info = page_info_from_fetch;
                }
            }
        }
    });

    let handle_prod_load_more = move |_| {
        if let Some(cd_latest) = category_details.read().as_ref() {
            if let Some(pi_latest) = &cd_latest.page_info {
                if let Some(ref cursor_latest) = pi_latest.end_cursor {
                    current_after_cursor.set(Some(cursor_latest.clone()));
                    category_resource.restart();
                }
            }
        }
    };

    let is_loading = matches!(*category_resource.state().read(), UseResourceState::Pending);

    let cloned_details = category_details.read().clone();
    match cloned_details {
        Some(cat_info_for_render) => rsx! {
            section { class: "py-12 bg-gray-50",
                div { class: "container mx-auto px-4",
                    CategoryHeader { category: cat_info_for_render.clone() }
                    h2 { class: "text-3xl font-semibold text-gray-700 mb-8 text-center", "Products" }
                    ProductGrid {
                        products: accumulated_products.read().clone(),
                        page_info: cat_info_for_render.page_info,
                        is_loading: is_loading,
                        on_load_more: handle_prod_load_more,
                    }
                }
            }
        },
        None => {
            if is_loading {
                rsx! { LoaderComponent {} }
            } else {
                rsx! { GenericErrorPage {
                    kind: AppErrorKind::Unknown,
                    message: "Could not load category information. Please try again later.".to_string(),
                    route: Some(vec!["category".to_string(), slug_for_error.clone()])
                }}
            }
        }
    }
}
