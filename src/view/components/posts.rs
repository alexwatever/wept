use dioxus::prelude::*;

// # Modules
use super::entity_list::{EntityDisplay, EntityDisplayListComponent};
use crate::model::posts::{Post, Posts};

/// # Posts Component
///
/// This component displays a list of posts.  
///
/// **Arguments**  
///
/// * `posts` - A signal containing a list of posts.
///
/// **Returns**  
///
/// The PostsComponent element.
#[component]
#[allow(non_snake_case)]
pub(crate) fn PostsComponent(posts: Signal<Posts>) -> Element {
    let items: Signal<Vec<Post>> = Signal::new(posts.read().0.clone());

    rsx! {
        EntityDisplayListComponent {
            entities: items
        }
    }
}

impl EntityDisplay for Post {
    fn render(&self) -> Element {
        let Post {
            id,
            content,
            slug,
            title,
            ..
        } = self.clone();

        rsx! {
            section { class: "p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
                div { class: "pl-4 text-left text-ellipsis",
                    a {
                        href: "/post/{slug.as_ref().unwrap_or(&id)}",
                        class: "w-full text-center font-bold text-xl",
                        "{title:?}"
                    }
                    p {
                        class: "w-full text-sm text-gray-500",
                        "{slug:?}"
                    }
                    p {
                        class: "w-full text-sm overflow-hidden line-clamp-3",
                        "{content:?}"
                    }
                }
            }
        }
    }
}
