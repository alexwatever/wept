use dioxus::prelude::*;

// Modules
use crate::{
    controllers::category::CategoryController,
    models::category::{ProductCategory, ProductCategoryImage},
    models::product::Product,
    routes::Routes,
    views::components::common::loader::LoaderComponent,
};

/// Category page component
#[component]
pub fn CategoryPage(slug: String) -> Element {
    let slug_prop = slug;
    let current_after_cursor = use_signal::<Option<String>>(|| None);
    let accumulated_products = use_signal::<Vec<Product>>(Vec::new);
    let category_details = use_signal::<Option<ProductCategory>>(|| None);
    let is_loading_more = use_signal(|| false);
    let load_more_error = use_signal::<Option<String>>(|| None);

    let category_resource = use_resource({
        let slug_prop_clone_for_outer_closure = slug_prop.clone();
        let current_after_cursor_signal = current_after_cursor.to_owned();
        let accumulated_products_signal = accumulated_products.to_owned();
        let category_details_signal = category_details.to_owned();
        let is_loading_more_signal = is_loading_more.to_owned();
        let mut load_more_error_writer = load_more_error.to_owned();

        move || {
            let slug_for_async_call = slug_prop_clone_for_outer_closure.clone();
            let current_after_cursor_for_call = current_after_cursor_signal.read().clone();
            let mut accumulated_products_writer = accumulated_products_signal.to_owned();
            let mut category_details_writer = category_details_signal.to_owned();
            let mut is_loading_more_writer = is_loading_more_signal.to_owned();
            let mut load_more_error_clear_writer = load_more_error_writer.to_owned();

            async move {
                is_loading_more_writer.set(true);
                load_more_error_clear_writer.set(None);

                let result = CategoryController::new()
                    .get_with_products(
                        &slug_for_async_call,
                        Some(10),
                        current_after_cursor_for_call.clone(),
                    )
                    .await;
                is_loading_more_writer.set(false);

                match result {
                    Ok(fetched_category_data) => {
                        let products_from_fetch = fetched_category_data
                            .products
                            .as_ref()
                            .map(|p_struct| p_struct.products.clone())
                            .unwrap_or_default();
                        let page_info_from_fetch = fetched_category_data.page_info.clone();

                        if current_after_cursor_for_call.is_none() {
                            accumulated_products_writer.set(products_from_fetch);
                            let initial_details = ProductCategory {
                                id: fetched_category_data.id,
                                database_id: fetched_category_data.database_id,
                                name: fetched_category_data.name,
                                slug: fetched_category_data.slug,
                                description: fetched_category_data.description,
                                count: fetched_category_data.count,
                                image: fetched_category_data.image,
                                products: None,
                                page_info: page_info_from_fetch,
                            };
                            category_details_writer.set(Some(initial_details));
                        } else {
                            let mut current_prods = accumulated_products_writer.read().clone();
                            current_prods.extend(products_from_fetch);
                            accumulated_products_writer.set(current_prods);

                            let mut global_cat_details_write = category_details_writer.write();
                            if let Some(cd_val) = global_cat_details_write.as_mut() {
                                cd_val.page_info = page_info_from_fetch;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching category/products: {:?}", e);
                        if current_after_cursor_for_call.is_none() {
                            category_details_writer.set(None);
                            accumulated_products_writer.set(Vec::new());
                        } else {
                            load_more_error_writer.set(Some(format!(
                                "Failed to load more products: {}",
                                e.public_message
                            )));
                        }
                    }
                }
            }
        }
    });

    let maybe_category_header_info_for_render = category_details.read().clone();

    if maybe_category_header_info_for_render.is_none()
        && category_resource.value().read().is_none()
        && !*is_loading_more.read()
    {
        return rsx! { LoaderComponent {} };
    }

    match maybe_category_header_info_for_render {
        Some(cat_info_for_render) => {
            let category_name = cat_info_for_render
                .name
                .as_ref()
                .cloned()
                .unwrap_or_else(|| "Unnamed Category".to_string());
            let category_description = cat_info_for_render
                .description
                .as_ref()
                .cloned()
                .unwrap_or_default();
            let category_image_url = cat_info_for_render
                .image
                .as_ref()
                .and_then(|img: &ProductCategoryImage| img.source_url.clone());
            let total_product_count = cat_info_for_render
                .count
                .map_or_else(|| "N/A".to_string(), |c| c.to_string());

            rsx! {
                section { class: "py-12 bg-gray-50",
                    div { class: "container mx-auto px-4",
                        div { class: "text-center mb-12",
                            if let Some(img_url) = category_image_url {
                                div { class: "mb-6 w-48 h-48 mx-auto rounded-lg overflow-hidden shadow-lg",
                                    img { class: "object-cover w-full h-full", src: "{img_url}", alt: "{category_name}" }
                                }
                            }
                            h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "{category_name}" }
                            p { class: "text-lg text-gray-600 mt-2", "Total products: {total_product_count}" }
                        }
                        if !category_description.is_empty() {
                            div { class: "max-w-3xl mx-auto bg-white p-8 rounded-lg shadow-md mb-12",
                                div { class: "prose lg:prose-xl", dangerous_inner_html: "{category_description}" }
                            }
                        } else {
                            div { class: "text-center text-gray-500 py-8 mb-12", "No description available for this category." }
                        }
                        h2 { class: "text-3xl font-semibold text-gray-700 mb-8 text-center", "Products" }
                        if accumulated_products.read().is_empty() && !*is_loading_more.read() {
                            p { class: "text-center text-gray-500", "No products found in this category." }
                        }
                        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8",
                            for product_item in accumulated_products.read().iter().filter(|p| p.slug.is_some() && !p.slug.as_ref().unwrap().is_empty()) {
                                div { class: "bg-white rounded-lg shadow-lg overflow-hidden transition-transform duration-300 hover:scale-105",
                                    if let Some(ref image) = product_item.image {
                                        if let Some(ref src_url) = image.source_url {
                                            Link { to: Routes::ProductPage { product_slug: product_item.slug.as_ref().cloned().unwrap() },
                                                img { class: "w-full h-56 object-cover", src: "{src_url}", alt: "{product_item.name.clone().unwrap_or_default()}" }
                                            }
                                        }
                                    } else {
                                        div { class: "w-full h-56 bg-gray-200 flex items-center justify-center", span { class: "text-gray-500", "No image" } }
                                    }
                                    div { class: "p-6",
                                        h3 { class: "text-xl font-semibold text-gray-800 mb-2 truncate",
                                            title: "{product_item.name.clone().unwrap_or_default()}",
                                            Link { to: Routes::ProductPage { product_slug: product_item.slug.as_ref().cloned().unwrap() },
                                                "{product_item.name.clone().unwrap_or_default()}"
                                            }
                                        }
                                        if let Some(ref simple_prod) = product_item.simple_product {
                                            if let Some(ref price) = simple_prod.price {
                                                p { class: "text-lg font-bold text-indigo-600 mb-4", "{price}" }
                                            } else { p { class: "text-lg text-gray-500 mb-4", "Price not available" } }
                                        } else { p { class: "text-lg text-gray-500 mb-4", "Price not available" } }
                                        Link { class: "block w-full text-center bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 transition-colors",
                                            to: Routes::ProductPage { product_slug: product_item.slug.as_ref().cloned().unwrap() },
                                            "View Details"
                                        }
                                    }
                                }
                            }
                        }
                        // Display Load More Error if any
                        {
                            if let Some(error_msg_ref) = load_more_error.read().as_ref() {
                                let error_message_for_render = error_msg_ref.clone();
                                rsx! {
                                    div { class: "text-center py-4 text-red-600",
                                        p { "{error_message_for_render}" }
                                    }
                                }
                            } else {
                                rsx! { Fragment {} } // Render nothing if no error
                            }
                        }
                        // Load More button/loader section
                        {
                            if *is_loading_more.read() {
                                rsx! { div { class: "text-center py-8", LoaderComponent {} } }
                            } else {
                                let mut show_load_more_button_flag = false;
                                if let Some(ref page_info_snapshot) = cat_info_for_render.page_info {
                                    if page_info_snapshot.has_next_page {
                                        show_load_more_button_flag = true;
                                    }
                                }

                                if show_load_more_button_flag {
                                    let category_details_onclick_signal = category_details.to_owned();
                                    let mut current_after_cursor_onclick_signal = current_after_cursor.to_owned();
                                    let mut category_resource_onclick_signal = category_resource.to_owned();

                                    rsx! {
                                        div { class: "text-center py-8",
                                            button {
                                                class: "px-6 py-3 bg-orange-500 text-white font-semibold rounded-lg shadow-md hover:bg-orange-600 transition-colors focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-opacity-50",
                                                onclick: move |_| {
                                                    if let Some(cd_latest) = category_details_onclick_signal.read().as_ref() {
                                                        if let Some(pi_latest) = &cd_latest.page_info {
                                                            if let Some(ref cursor_latest) = pi_latest.end_cursor {
                                                                current_after_cursor_onclick_signal.set(Some(cursor_latest.clone()));
                                                                category_resource_onclick_signal.restart();
                                                            }
                                                        }
                                                    }
                                                },
                                                "Load More Products"
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
        None => {
            if category_resource.value().read().is_none() && !*is_loading_more.read() {
                rsx! { crate::views::pages::errors::GenericErrorPage {
                    kind: crate::app::error::AppErrorKind::General,
                    message: "Could not load category information. Please try again later.".to_string(),
                    route: Some(vec!["category".to_string(), slug_prop.clone()])
                }}
            } else {
                rsx! { LoaderComponent {} }
            }
        }
    }
}
