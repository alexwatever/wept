use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
pub(crate) fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24 text-center",
                    h1 { "Page not found" }
                    p { "We are terribly sorry, but the page you requested doesn't exist." }
                    pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
                }
            }
        }
    }
}
