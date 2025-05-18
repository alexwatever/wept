use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
pub(crate) fn NotFoundPage(route: Vec<String>, log: Option<String>) -> Element {
    let route: String = route.join("/");

    // # Render page
    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto",
                h2 { class: "text-3xl font-bold", "Page Not Found" }
                p { "The page you requested doesn't exist." }

                if route.len() > 0 {
                    p {
                        "Requested route:\n{route}"
                    }
                }
                if let Some(log) = log {
                    p {
                        "Log:\n{log}"
                    }
                }

                p {
                    a {
                        class: "font-bold",
                        href: "/",
                        "Return Home"
                    }
                }
            }
        }
    }
}
