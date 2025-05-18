use dioxus::{prelude::*, CapturedError};

// # Modules
use super::{super::components::loader::LoaderComponent, errors::NotFoundPage};
use crate::{controller::posts::fetch_post, model::posts::Post};

#[component]
#[allow(non_snake_case)]
pub(crate) fn PostPage(post_slug: ReadOnlySignal<String>) -> Element {
    // Fetch the post
    let post: Resource<Result<Post, CapturedError>> =
        use_resource(move || async move { fetch_post(post_slug()).await });

    let post_data = post.read();
    match post_data.as_ref() {
        // Loading state
        None => rsx! {
            LoaderComponent {}
        },
        // Error state
        Some(Err(error)) => rsx! {
            NotFoundPage { route: vec!["post".to_string(), post_slug()], log: Some(error.to_string()) }
        },
        // Post found
        Some(Ok(post)) => {
            // Unwrap the post
            let Post {
                id,
                content,
                slug,
                title,
                date,
                ..
            } = post;

            // # Render page
            rsx! {
                section { class: "py-20",
                    div { class: "container mx-auto px-4",
                        div { class: "flex flex-wrap -mx-4 mb-24",
                            div { class: "w-full md:w-1/2 px-4",
                                div { class: "lg:pl-20",
                                    div { class: "mb-10 pb-10 border-b",
                                        // Post title
                                        h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                            "{title.clone().unwrap_or_default()}"
                                        }

                                        // Post metadata
                                        {
                                            if let Some(post_date) = date.as_ref() {
                                                rsx! {
                                                    p { class: "mb-4 text-sm text-gray-500",
                                                        "Published: {post_date}"
                                                    }
                                                }
                                            } else {
                                                rsx! { "" }
                                            }
                                        }

                                        // Post slug
                                        p { class: "inline-block mb-8 text-gray-600",
                                            span {
                                                "Slug: {slug.clone().unwrap_or_default()}"
                                            }
                                            span { class: "ml-2 text-gray-400",
                                                "(ID: {id})"
                                            }
                                        }

                                        // Post content
                                        div { class: "max-w-md",
                                            dangerous_inner_html: "{content.clone().unwrap_or_default()}"
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
