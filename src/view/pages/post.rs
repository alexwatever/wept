use dioxus::prelude::*;

// # Modules
use crate::{controller::posts::fetch_post, model::posts::Post};

#[component]
#[allow(non_snake_case)]
pub(crate) fn PostPage(post_id: ReadOnlySignal<String>) -> Element {
    let post = use_server_future(move || fetch_post(post_id()))?;

    let Post {
        id,
        content,
        slug,
        title,
        ..
    } = post().unwrap()?;

    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24",
                    div { class: "w-full md:w-1/2 px-4",
                        div { class: "lg:pl-20",
                            div { class: "mb-10 pb-10 border-b",
                                h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
                                    "{title:?}"
                                }
                                p { class: "inline-block mb-8 text-2xl font-bold font-heading text-blue-300",
                                    span {
                                        "{slug:?}"
                                    }
                                    span {
                                        "{id}"
                                    }
                                }
                                div { class: "max-w-md text-gray-500",
                                    "{content:?}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
