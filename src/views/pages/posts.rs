use dioxus::{hooks::UseResourceState, prelude::*};

// Modules
use crate::{
    controllers::{common::EntityController, post::PostController},
    models::{pagination::Pagination, post::Post},
    views::components::post::post_card::PostCard,
};

/// Posts page component
#[component]
pub fn PostsPage() -> Element {
    let mut all_posts = use_signal(Vec::<Post>::new);
    let mut current_cursor = use_signal(|| None::<String>);
    let mut page_info = use_signal(|| None::<Pagination>);

    let mut posts_resource = use_resource(move || {
        let current_cursor_cloned = current_cursor.read().clone();
        async move {
            PostController::new()
                .get_list(Some(10), current_cursor_cloned)
                .await
        }
    });

    use_effect(move || {
        if let Some(Ok(fetched_data)) = posts_resource.value().read().as_ref() {
            if let Some(last_fetched_slug) =
                fetched_data.posts.first().and_then(|p| p.slug.as_ref())
            {
                if !all_posts
                    .read()
                    .iter()
                    .any(|ap| ap.slug.as_ref() == Some(last_fetched_slug))
                {
                    all_posts.write().extend(fetched_data.posts.clone());
                }
            } else {
                all_posts.set(fetched_data.posts.clone());
            }
            page_info.set(fetched_data.page_info.clone());
        }
    });

    let handle_load_more = move |_| {
        if let Some(pi) = page_info.read().as_ref() {
            if pi.has_next_page {
                if let Some(end_cursor) = pi.end_cursor.as_ref() {
                    current_cursor.set(Some(end_cursor.clone()));
                    posts_resource.restart();
                }
            }
        }
    };

    let is_loading = matches!(*posts_resource.state().read(), UseResourceState::Pending);

    rsx! {
        section { class: "py-12 bg-gray-50",
            div { class: "container mx-auto px-4",
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl md:text-5xl font-bold text-gray-800", "Posts" }
                }
                div { class: "flex flex-wrap -m-4",
                    for post in all_posts.read().iter() {
                        PostCard { post: post.clone() }
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
                                "Load More Posts"
                            }
                        }
                    }
                }
            }
        }
    }
}
