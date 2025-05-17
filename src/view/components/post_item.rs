use dioxus::prelude::*;

// # Modules
use crate::model::posts::Post;

#[component]
pub(crate) fn post_item(post: Post) -> Element {
    let Post {
        id,
        content,
        slug,
        title,
        ..
    } = post;

    rsx! {
        section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
            div { class: "pl-4 text-left text-ellipsis",
                a {
                    href: "/posts/{id}",
                    class: "w-full text-center",
                    "{title:?}"
                }
                p {
                    class: "w-full",
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
