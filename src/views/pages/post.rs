use dioxus::prelude::*;

// Modules
use crate::{
    app::error::AppError,
    controllers::{common::EntityController, post::PostController},
    models::post::Post,
    views::components::common::loader::LoaderComponent,
};

/// Post page component
#[component]
pub fn PostPage(post_slug: String) -> Element {
    // Fetch the post
    let slug_for_resource: String = post_slug.clone();
    let post_resource: Resource<Result<Post, AppError>> = use_resource(move || {
        let slug_for_async_operation: String = slug_for_resource.clone();
        async move {
            PostController::new()
                .get_by_slug(&slug_for_async_operation)
                .await
        }
    });

    // Wait for post data
    let post_data = post_resource.read();
    match post_data.as_ref() {
        // Loading state
        None => rsx! { LoaderComponent {} },
        // Error state
        Some(Err(app_error)) => app_error.render(vec!["post".to_string(), post_slug]),
        // Post found
        Some(Ok(post)) => {
            // Get Post values
            let Post {
                id,
                content,
                slug,
                title,
                date,
                ..
            } = post;

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
                                        if let Some(post_date) = date {
                                            p { class: "mb-4 text-sm text-gray-500",
                                                "Published: {post_date}"
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
