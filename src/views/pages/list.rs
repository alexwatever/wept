use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    controllers::{common::EntityController, page::PageController},
    models::{page::Page, pagination::Pagination},
    views::components::page::page_card::PageCard,
};

/// Pages list page component
#[component]
pub fn PagesListPage() -> Element {
    let mut all_pages = use_signal(Vec::<Page>::new);
    let mut current_cursor = use_signal(|| None::<String>);
    let mut page_info = use_signal(|| None::<Pagination>);

    let mut pages_resource = use_resource(move || {
        let current_cursor_cloned = current_cursor.read().clone();
        async move {
            PageController::new()
                .get_list(Some(10), current_cursor_cloned)
                .await
        }
    });

    use_effect(move || {
        if let Some(Ok(fetched_data)) = pages_resource.value().read().as_ref() {
            if let Some(last_fetched_slug) =
                fetched_data.pages.first().and_then(|p| p.slug.as_ref())
            {
                if !all_pages
                    .read()
                    .iter()
                    .any(|ap| ap.slug.as_ref() == Some(last_fetched_slug))
                {
                    all_pages.write().extend(fetched_data.pages.clone());
                }
            } else {
                all_pages.set(fetched_data.pages.clone());
            }
            page_info.set(fetched_data.page_info.clone());
        }
    });

    let handle_load_more = move |_| {
        if let Some(pi) = page_info.read().as_ref() {
            if pi.has_next_page {
                if let Some(end_cursor) = pi.end_cursor.as_ref() {
                    current_cursor.set(Some(end_cursor.clone()));
                    pages_resource.restart();
                }
            }
        }
    };

    let is_loading = matches!(*pages_resource.state().read(), UseResourceState::Pending);

    rsx! {
        section { class: "py-12 bg-gray-50",
            div { class: "container mx-auto px-4",
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "Pages" }
                }
                div { class: "flex flex-wrap",
                    for page in all_pages.read().iter() {
                        div {
                            class: "p-4 w-full md:w-1/2 lg:w-1/3",
                            PageCard { page: page.clone() }
                        }
                    }
                }
                if is_loading {
                    div { class: "flex justify-center items-center py-8",
                        crate::views::components::common::loader::LoaderComponent {}
                    }
                }
                if let Some(pi) = page_info.read().as_ref() {
                    if pi.has_next_page && !is_loading {
                        div { class: "text-center mt-12 py-8",
                            button {
                                class: "px-8 py-3 bg-indigo-600 text-white font-semibold rounded-lg shadow-md hover:bg-indigo-700",
                                onclick: handle_load_more,
                                "Load More Pages"
                            }
                        }
                    }
                }
            }
        }
    }
}
