use dioxus::prelude::*;

// Modules
use crate::routes::Routes::HomePage;

/// Not Found page component
///
/// **Arguments**  
///
/// * `message` - The log message to pass to the error page.
/// * `route` - The route segments to pass to the error page.
///
/// **Returns**  
///
/// * `Element` - The rendered error page.
#[component]
pub fn NotFoundPage(message: Option<String>, route: Vec<String>) -> Element {
    let route: String = route.join(" / ");

    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto",
                h2 {
                    class: "text-3xl font-bold",
                    "Page Not Found"
                }

                p { "The page you requested doesn't exist." }

                p {
                    "Requested route: {route}"
                }
                if let Some(message) = message {
                    p {
                        "Log: {message}"
                    }
                }

                p {
                    Link {
                        to: HomePage {},
                        class: "group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        "Go to Homepage"
                    }
                }
            }
        }
    }
}
