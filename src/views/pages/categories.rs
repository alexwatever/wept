use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    app::error::AppError,
    controllers::{
        category::CategoryController, common::EntityController, product::ProductController,
    },
    models::{
        category::{ProductCategories, ProductCategory},
        pagination::Pagination,
        product::{Product, Products},
    },
    routes::Routes,
    views::components::{common::loader::LoaderComponent, product::product_card::ProductCard},
};

/// Categories page component
#[component]
pub fn CategoriesPage() -> Element {
    // CATEGORY Signals
    let mut cat_is_initial_load = use_signal(|| true);
    let mut all_categories = use_signal(Vec::<ProductCategory>::new);
    let mut cat_current_cursor = use_signal(|| None::<String>);
    let mut cat_page_info = use_signal(|| None::<Pagination>);

    // PRODUCT Signals
    let mut prod_is_initial_load = use_signal(|| true);
    let mut all_products = use_signal(Vec::<Product>::new);
    let mut prod_current_cursor = use_signal(|| None::<String>);
    let mut prod_page_info = use_signal(|| None::<Pagination>);

    // Categories API resource
    let mut categories_resource: Resource<Result<ProductCategories, AppError>> =
        use_resource(move || {
            let current_cursor_cloned = cat_current_cursor.read().clone();
            async move {
                CategoryController::new()
                    .get_list(Some(10), current_cursor_cloned)
                    .await
            }
        });

    // Products API resource
    let mut products_resource: Resource<Result<Products, AppError>> = use_resource(move || {
        let current_cursor_cloned = prod_current_cursor.read().clone();
        async move {
            ProductController::new()
                .get_list(Some(8), current_cursor_cloned)
                .await
        }
    });

    // Process category data
    match categories_resource.value().read().as_ref() {
        Some(Ok(fetched_data)) => {
            if cat_is_initial_load() {
                all_categories.set(fetched_data.categories.clone());
                cat_is_initial_load.set(false);
            } else {
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
                }
            }
            cat_page_info.set(fetched_data.page_info.clone());
        }
        Some(Err(_)) => {
            if cat_is_initial_load() {
                cat_is_initial_load.set(false);
            }
        }
        None => {}
    }

    // Process product data
    match products_resource.value().read().as_ref() {
        Some(Ok(fetched_data)) => {
            if prod_is_initial_load() {
                all_products.set(fetched_data.products.clone());
                prod_is_initial_load.set(false);
            } else {
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
                }
            }
            prod_page_info.set(fetched_data.page_info.clone());
        }
        Some(Err(_)) => {
            if prod_is_initial_load() {
                prod_is_initial_load.set(false);
            }
        }
        None => {}
    }

    rsx! {
        section { class: "py-12 bg-gray-50",
            div { class: "container mx-auto px-4",
                // Categories Section
                div { class: "mb-16",
                    div { class: "text-center mb-12",
                        h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "Product Categories" }
                    }
                    {
                        let categories_resource_read = categories_resource.read();
                        match categories_resource_read.as_ref() {
                            None if cat_is_initial_load() => rsx! { LoaderComponent {} },
                            Some(Err(e)) => e.render(vec!["categories".to_string()]),
                            _ => {
                                let categories_to_display = all_categories.read();
                                if categories_to_display.is_empty() && !cat_is_initial_load() {
                                    rsx! {
                                        div { class: "text-center py-8",
                                            p { class: "text-lg text-gray-500", "No categories found." }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-8",
                                            for category_item in categories_to_display.iter() {
                                                // Category Card
                                                div { class: "bg-white rounded-lg shadow-md overflow-hidden transform hover:scale-105 transition-transform duration-300",
                                                    Link {
                                                        to: Routes::CategoryPage { slug: category_item.slug.as_ref().cloned().unwrap_or_default() },
                                                        if let Some(img_src) = category_item.image.as_ref().and_then(|img| img.source_url.as_ref()) {
                                                            img { class: "w-full h-40 object-cover", src: "{img_src}", alt: "{category_item.name.as_deref().unwrap_or(\"Category Image\")}" }
                                                        } else {
                                                            div { class: "w-full h-40 bg-gray-200" }
                                                        }
                                                        div { class: "p-4",
                                                            h3 { class: "text-lg font-semibold text-gray-800 truncate", title: "{category_item.name.as_deref().unwrap_or_default()}", "{category_item.name.as_deref().unwrap_or_default()}" }
                                                            p { class: "text-sm text-gray-600", "Products: {category_item.count.map(|c| c.to_string()).as_deref().unwrap_or(\"N/A\")}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // Category Load More
                                        {
                                            let show_load_more = cat_page_info.read().as_ref().is_some_and(|pi| pi.has_next_page);
                                            let is_loading_more = matches!(*categories_resource.state().read(), UseResourceState::Pending) && !cat_is_initial_load();

                                            if is_loading_more {
                                                rsx! { div { class: "flex justify-center py-8", LoaderComponent {} } }
                                            } else if show_load_more {
                                                rsx! {
                                                    div { class: "text-center mt-8",
                                                        button {
                                                            class: "px-6 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300",
                                                            onclick: move |_| {
                                                                if let Some(pi) = cat_page_info.read().as_ref() {
                                                                    if pi.has_next_page {
                                                                        if let Some(end_cursor) = pi.end_cursor.as_ref() {
                                                                            cat_current_cursor.set(Some(end_cursor.clone()));
                                                                            cat_is_initial_load.set(false);
                                                                            categories_resource.restart();
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "Load More Categories"
                                                        }
                                                    }
                                                }
                                            } else {
                                                rsx! { Fragment {} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Products Section
                div {
                    div { class: "text-center mb-12 pt-8 border-t border-gray-200",
                        h2 { class: "text-4xl md:text-5xl font-bold text-gray-800", "All Products" }
                    }
                    {
                        let products_resource_read = products_resource.read();
                        match products_resource_read.as_ref() {
                            None if prod_is_initial_load() => rsx! { LoaderComponent {} },
                            Some(Err(e)) => e.render(vec!["products".to_string()]),
                            _ => {
                                let products_to_display = all_products.read();
                                if products_to_display.is_empty() && !prod_is_initial_load() {
                                    rsx! {
                                        div { class: "text-center py-8",
                                            p { class: "text-lg text-gray-500", "No products found." }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8",
                                            for product_item in products_to_display.iter() {
                                                ProductCard { product: product_item.clone() }
                                            }
                                        }
                                        // Product Load More
                                        {
                                            let show_load_more = prod_page_info.read().as_ref().is_some_and(|pi| pi.has_next_page);
                                            let is_loading_more = matches!(*products_resource.state().read(), UseResourceState::Pending) && !prod_is_initial_load();

                                            if is_loading_more {
                                                rsx! { div { class: "flex justify-center items-center py-8", LoaderComponent {} } }
                                            } else if show_load_more {
                                                rsx! {
                                                    div { class: "text-center mt-12 py-8",
                                                        button {
                                                            class: "px-8 py-3 bg-indigo-600 text-white font-semibold rounded-lg shadow-md hover:bg-indigo-700",
                                                            onclick: move |_| {
                                                                if let Some(pi) = prod_page_info.read().as_ref() {
                                                                    if pi.has_next_page {
                                                                        if let Some(end_cursor) = pi.end_cursor.as_ref() {
                                                                            prod_current_cursor.set(Some(end_cursor.clone()));
                                                                            prod_is_initial_load.set(false);
                                                                            products_resource.restart();
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "Load More Products"
                                                        }
                                                    }
                                                }
                                            } else if !products_to_display.is_empty() {
                                                rsx! {
                                                    div { class: "text-center mt-12 py-8",
                                                        p { class: "text-lg text-gray-500", "All products have been loaded." }
                                                    }
                                                }
                                            } else {
                                                rsx!{ Fragment {} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
