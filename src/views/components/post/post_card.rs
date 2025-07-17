use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{models::post::Post, routes::Routes, views::components::common::card::Card};

/// Post card component
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
        Card {
            title: post.title.unwrap_or_default(),
            if let Some(date) = &post.date {
                p { class: "text-gray-500 text-sm mb-3",
                    "{date}"
                }
            }
            p { class: "leading-relaxed mb-5",
                "{content_preview}"
            }
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
