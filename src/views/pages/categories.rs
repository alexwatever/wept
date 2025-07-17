use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    controllers::{
        category::CategoryController, common::EntityController, product::ProductController,
    },
    models::{category::ProductCategory, pagination::Pagination, product::Product},
    views::components::{
        category::category_list::CategoryList, product::product_grid::ProductGrid,
    },
};

/// Categories page component
#[component]
pub fn CategoriesPage() -> Element {
    // CATEGORY Signals
    let mut all_categories = use_signal(Vec::<ProductCategory>::new);
    let mut cat_current_cursor = use_signal(|| None::<String>);
    let mut cat_page_info = use_signal(|| None::<Pagination>);

    // PRODUCT Signals
    let mut all_products = use_signal(Vec::<Product>::new);
    let mut prod_current_cursor = use_signal(|| None::<String>);
    let mut prod_page_info = use_signal(|| None::<Pagination>);

    // Categories API resource
    let mut categories_resource = use_resource(move || {
        let current_cursor_cloned = cat_current_cursor.read().clone();
        async move {
            CategoryController::new()
                .get_list(Some(10), current_cursor_cloned)
                .await
        }
    });

    // Products API resource
    let mut products_resource = use_resource(move || {
        let current_cursor_cloned = prod_current_cursor.read().clone();
        async move {
            ProductController::new()
                .get_list(Some(8), current_cursor_cloned)
                .await
        }
    });

    // Process category data
    use_effect(move || {
        if let Some(Ok(fetched_data)) = categories_resource.value().read().as_ref() {
            if let Some(last_fetched_slug) = fetched_data
                .categories
                .first()
                .and_then(|c| c.slug.as_ref())
            {
                if !all_categories
                    .read()
                    .iter()
                    .any(|ac| ac.slug.as_ref() == Some(last_fetched_slug))
                {
                    all_categories
                        .write()
                        .extend(fetched_data.categories.clone());
                }
            } else {
                all_categories.set(fetched_data.categories.clone());
            }
            cat_page_info.set(fetched_data.page_info.clone());
        }
    });

    // Process product data
    use_effect(move || {
        if let Some(Ok(fetched_data)) = products_resource.value().read().as_ref() {
            if let Some(last_fetched_slug) =
                fetched_data.products.first().and_then(|p| p.slug.as_ref())
            {
                if !all_products
                    .read()
                    .iter()
                    .any(|ap| ap.slug.as_ref() == Some(last_fetched_slug))
                {
                    all_products.write().extend(fetched_data.products.clone());
                }
            } else {
                all_products.set(fetched_data.products.clone());
            }
            prod_page_info.set(fetched_data.page_info.clone());
        }
    });

    let handle_cat_load_more = move |_| {
        if let Some(pi) = cat_page_info.read().as_ref() {
            if pi.has_next_page {
                if let Some(end_cursor) = pi.end_cursor.as_ref() {
                    cat_current_cursor.set(Some(end_cursor.clone()));
                    categories_resource.restart();
                }
            }
        }
    };

    let handle_prod_load_more = move |_| {
        if let Some(pi) = prod_page_info.read().as_ref() {
            if pi.has_next_page {
                if let Some(end_cursor) = pi.end_cursor.as_ref() {
                    prod_current_cursor.set(Some(end_cursor.clone()));
                    products_resource.restart();
                }
            }
        }
    };

    let is_cat_loading = matches!(
        *categories_resource.state().read(),
        UseResourceState::Pending
    );
    let is_prod_loading = matches!(*products_resource.state().read(), UseResourceState::Pending);

    rsx! {
        section { class: "py-12 bg-gray-50",
            div { class: "container mx-auto px-4",
                div { class: "mb-16",
                    div { class: "text-center mb-12",
                        h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "Product Categories" }
                    }
                    CategoryList {
                        categories: all_categories.read().clone(),
                        page_info: cat_page_info.read().clone(),
                        is_loading: is_cat_loading,
                        on_load_more: handle_cat_load_more,
                    }
                }
                div {
                    div { class: "text-center mb-12 pt-8 border-t border-gray-200",
                        h2 { class: "text-4xl md:text-5xl font-bold text-gray-800", "All Products" }
                    }
                    ProductGrid {
                        products: all_products.read().clone(),
                        page_info: prod_page_info.read().clone(),
                        is_loading: is_prod_loading,
                        on_load_more: handle_prod_load_more,
                    }
                }
            }
        }
    }
}
