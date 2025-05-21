use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    app::error::AppError,
    controllers::{category::CategoryController, common::EntityController},
    models::{
        category::{ProductCategories, ProductCategory},
        pagination::Pagination,
    },
    routes::Routes,
    views::components::common::loader::LoaderComponent,
};

/// Categories page component
#[component]
pub fn CategoriesPage() -> Element {
    let mut all_categories = use_signal(Vec::<ProductCategory>::new);
    let mut current_cursor = use_signal(|| None::<String>);
    let mut page_info = use_signal(|| None::<Pagination>);
    let mut is_initial_load = use_signal(|| true);

    let mut categories_resource: Resource<Result<ProductCategories, AppError>> =
        use_resource(move || {
            let current_cursor_cloned = current_cursor.read().clone();
            async move {
                CategoryController::new()
                    .get_list(Some(20), current_cursor_cloned)
                    .await
            }
        });

    match categories_resource.value().read().as_ref() {
        Some(Ok(fetched_data)) => {
            if is_initial_load() {
                all_categories.set(fetched_data.categories.clone());
                is_initial_load.set(false);
            } else {
                // Check if the fetched data is different before appending to avoid duplicates on hot-reload
                if let Some(last_fetched_slug) = fetched_data
                    .categories
                    .first()
                    .as_ref()
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
                } else if fetched_data.categories.is_empty()
                    && fetched_data
                        .page_info
                        .as_ref()
                        .is_some_and(|pi| pi.has_next_page)
                {
                    // Potentially an empty page but more to load, do nothing to all_categories
                } else if fetched_data.categories.is_empty()
                    && !fetched_data
                        .page_info
                        .as_ref()
                        .is_some_and(|pi| pi.has_next_page)
                {
                    // No new categories and no more pages.
                } else if !fetched_data.categories.is_empty() {
                    // Fallback for non-empty new categories if slug check is problematic
                    all_categories
                        .write()
                        .extend(fetched_data.categories.clone());
                }
            }
            page_info.set(fetched_data.page_info.clone());
        }
        Some(Err(_)) => {
            // Error already handled by the render block below
            if is_initial_load() {
                is_initial_load.set(false); // Stop initial load indication on error
            }
        }
        None => {
            // Still loading
            if !is_initial_load() {
                // This is a subsequent load (e.g. "Load More")
                // The LoaderComponent will be shown by the render block
            }
        }
    }

    // Handle resource states: Loading, Error, Success
    let categories_resource_read = categories_resource.read();
    match categories_resource_read.as_ref() {
        None if is_initial_load() => rsx! { LoaderComponent {} }, // Initial loading state
        Some(Err(app_error)) => app_error.render(vec!["categories".to_string()]),
        _ => {
            // Covers Some(Ok(_)) and subsequent loads (None but not initial_load)
            let categories_to_display = all_categories.read();

            if categories_to_display.is_empty()
                && !is_initial_load()
                && !page_info.read().as_ref().is_some_and(|pi| pi.has_next_page)
            {
                return rsx! {
                    div { class: "container mx-auto px-4 py-20 text-center",
                        h1 { class: "text-3xl font-bold text-gray-700 mb-4", "No Categories Found" }
                        p { class: "text-lg text-gray-500", "There are currently no product categories to display." }
                    }
                };
            }

            let show_load_more_button =
                page_info.read().as_ref().is_some_and(|pi| pi.has_next_page);
            let is_loading_more = matches!(
                *categories_resource.state().read(),
                UseResourceState::Pending
            ) && !is_initial_load();

            rsx! {
                section { class: "py-12 bg-gray-50",
                    div { class: "container mx-auto px-4",
                        div { class: "text-center mb-12",
                            h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "Product Categories" }
                        }

                        if categories_to_display.is_empty() && is_initial_load() {
                            // This case should be covered by the initial loader, but as a fallback:
                            LoaderComponent {}
                        } else if categories_to_display.is_empty() {
                             div { class: "container mx-auto px-4 py-20 text-center",
                                h1 { class: "text-3xl font-bold text-gray-700 mb-4", "No Categories Found" }
                                p { class: "text-lg text-gray-500", "There are currently no product categories to display." }
                            }
                        } else {
                            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8",
                                for category_item in categories_to_display.iter() {
                                    div { class: "bg-white rounded-lg shadow-md overflow-hidden transform hover:scale-105 transition-transform duration-300",
                                        Link {
                                            to: Routes::CategoryPage {
                                                slug: category_item.slug.as_ref().cloned().unwrap_or_default()
                                            },
                                            if let Some(img_src) = category_item.image.as_ref().and_then(|img| img.source_url.as_ref()) {
                                                img {
                                                    class: "w-full h-48 object-cover",
                                                    src: "{img_src}",
                                                    alt: "{category_item.name.as_deref().unwrap_or(\"Category Image\")}"
                                                }
                                            } else {
                                                div { class: "w-full h-48 bg-gray-200 flex items-center justify-center",
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        class: "h-16 w-16 text-gray-400",
                                                        fill: "none",
                                                        view_box: "0 0 24 24",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        path {
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "p-6",
                                                h3 {
                                                    class: "text-xl font-semibold text-gray-800 mb-2 truncate",
                                                    title: "{category_item.name.as_deref().unwrap_or(\"Unnamed Category\")}",
                                                    "{category_item.name.as_deref().unwrap_or(\"Unnamed Category\")}"
                                                }
                                                p {
                                                    class: "text-sm text-gray-600",
                                                    "Products: {category_item.count.map(|c| c.to_string()).as_deref().unwrap_or(\"N/A\")}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if is_loading_more {
                           div { class: "flex justify-center items-center py-8", LoaderComponent {} }
                        }

                        if show_load_more_button && !is_loading_more {
                            div { class: "text-center mt-12 py-8",
                                button {
                                    class: "px-8 py-3 bg-indigo-600 text-white font-semibold rounded-lg shadow-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-75 transition-colors duration-300",
                                    onclick: move |_| {
                                        if let Some(pi) = page_info.read().as_ref() {
                                            if pi.has_next_page {
                                                if let Some(end_cursor) = pi.end_cursor.as_ref() {
                                                    current_cursor.set(Some(end_cursor.clone()));
                                                    // Ensure we are not in initial load state
                                                    is_initial_load.set(false);
                                                    categories_resource.restart();
                                                }
                                            }
                                        }
                                    },
                                    "Load More Categories"
                                }
                            }
                        } else if !is_loading_more && !categories_to_display.is_empty() && page_info.read().as_ref().is_none_or(|pi| !pi.has_next_page) {
                            // All categories loaded
                            div { class: "text-center mt-12 py-8",
                                p { class: "text-lg text-gray-500", "All categories have been loaded." }
                            }
                        }
                    }
                }
            }
        }
    }
}
