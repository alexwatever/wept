use dioxus::prelude::*;

// # Modules
use crate::{controller::posts::fetch_post, model::posts::Post};

#[component]
#[allow(non_snake_case)]
pub(crate) fn PostPage(post_slug: ReadOnlySignal<String>) -> Element {
    let post = use_resource(move || async move { fetch_post(post_slug()).await });

    let post_data = post.read();
    match post_data.as_ref() {
        None => rsx! { // Loading state
            div { class: "container mx-auto px-4 py-20 flex justify-center",
                div { class: "animate-pulse",
                    h2 { class: "text-2xl font-bold", "Loading Post..." }
                }
            }
        },
        Some(Err(error)) => rsx! { // Error state
            div { class: "container mx-auto px-4 py-20",
                h2 { class: "text-3xl font-bold text-red-500", "Error Loading Post" }
                p { class: "text-lg", "There was an error loading the post: {error}" }
                a {
                    class: "mt-4 inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    href: "/",
                    "Return to Home"
                }
            }
        },
        Some(Ok(p)) => {
            let Post {
                id,
                content,
                slug,
                title,
                date,
                ..
            } = p;

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
                                        // We've removed the author and categories sections to match the simplified model
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
