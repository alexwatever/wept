use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{entity::EntityController, page::PageController},
    models::page::Page,
    views::components::common::loader::LoaderComponent,
};

/// Page - page component
#[component]
pub fn PagePage(slug: String) -> Element {
    // Fetch the page
    let slug_for_resource: String = slug.clone();
    let page_resource: Resource<Result<Page, AppError>> = use_resource(move || {
        let slug_for_async_operation: String = slug_for_resource.clone();
        async move {
            PageController::new()
                .get_by_slug(&slug_for_async_operation)
                .await
        }
    });

    // Wait for page data
    let page_data = page_resource.read();
    match page_data.as_ref() {
        // Loading state
        None => rsx! { LoaderComponent {} },
        // Error state
        Some(Err(app_error)) => app_error.render(vec!["page".to_string(), slug]),
        // Page found
        Some(Ok(page)) => {
            // Get Page values
            let Page {
                id,
                content,
                slug,
                title,
                date,
                ..
            } = page;

            // Render page
            rsx! {
                section { class: "py-20",
                    div { class: "container mx-auto px-4",
                        div { class: "flex flex-wrap -mx-4 mb-24",
                            div { class: "w-full md:w-1/2 px-4",
                                div { class: "lg:pl-20",
                                    div { class: "mb-10 pb-10 border-b",
                                        // Title
                                        if let Some(title) = title {
                                            h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                                "{title}"
                                            }
                                        }

                                        // Date
                                        if let Some(page_date) = date {
                                            p { class: "mb-4 text-sm text-gray-500",
                                                "Published: {page_date}"
                                            }
                                        }

                                        // Slug
                                        if let Some(slug) = slug {
                                            p { class: "inline-block mb-8 text-gray-600",
                                                span {
                                                    "Slug: {slug}"
                                            }
                                            span { class: "ml-2 text-gray-400",
                                                    "(ID: {id})"
                                                }
                                            }
                                        }

                                        // Content
                                        if let Some(content) = content {
                                            div { class: "max-w-md",
                                                dangerous_inner_html: "{content}"
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
