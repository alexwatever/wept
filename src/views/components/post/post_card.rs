use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{models::post::Post, routes::Routes};

/// Post card component
///
/// Displays a post within a list
///
/// **Arguments**
///
/// * `post` - The post to display
///
/// **Returns**  
///
/// A component that displays the post
#[component]
pub fn PostCard(post: Post) -> Element {
    // Create content preview
    let content_preview: String = post
        .content
        .as_ref()
        .map(|c: &String| {
            if c.len() > 100 {
                format!("{}...", &c[..100])
            } else {
                c.clone()
            }
        })
        .unwrap_or_default();

    rsx! {
        div { class: "p-4 w-full md:w-1/2 lg:w-1/3",
            div { class: "h-full border-2 border-gray-200 border-opacity-60 rounded-lg overflow-hidden",
                div { class: "p-6",
                    // Title
                    if let Some(title) = &post.title {
                        h2 { class: "text-xl font-bold mb-2",
                            "{title}"
                        }
                    }

                    // Date
                    if let Some(date) = &post.date {
                        p { class: "text-gray-500 text-sm mb-3",
                            "{date}"
                        }
                    }

                    // Content preview
                    p { class: "leading-relaxed mb-5",
                        "{content_preview}"
                    }

                    // Read more link
                    if let Some(slug) = &post.slug {
                        div { class: "mt-4",
                            Link {
                                class: "inline-block text-blue-500 hover:text-blue-700",
                                to: Routes::PostPage { post_slug: slug.clone() },
                                "Read more →"
                            }
                        }
                    }
                }
            }
        }
    }
}
