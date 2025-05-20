use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{app::error::AppErrorKind, routes::Routes::HomePage};

/// Generic error page
///
/// **Arguments**  
///
/// * `kind` - The kind of error that occurred.
/// * `message` - The user-friendly message to display.
/// * `route` - The route segments to pass to the error page.
///
/// **Returns**  
///
/// * `Element` - The rendered error page.
#[component]
pub fn GenericErrorPage(
    kind: AppErrorKind,
    message: String,
    route: Option<Vec<String>>,
) -> Element {
    let kind: String = kind.to_string();
    let route: Option<String> = route.as_ref().map(|r: &Vec<String>| r.join(" / "));

    rsx! {
        section {
            class: "flex items-center justify-center min-h-screen bg-gray-100 py-12 px-4 sm:px-6 lg:px-8",
            div {
                class: "max-w-md w-full space-y-8 p-10 bg-white shadow-xl rounded-lg text-center",
                div {
                    div {
                        class: "mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-red-100",
                        svg {
                            class: "h-6 w-6 text-red-600",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
                            }
                        }
                    }
                    h2 {
                        class: "mt-6 text-center text-3xl font-extrabold text-gray-900",
                        "Oops! {kind}"
                    }
                    p {
                        class: "mt-2 text-center text-sm text-gray-600",
                        "{message}"
                    }
                    if let Some(route) = route {
                        p {
                            class: "mt-2 text-xs text-gray-500",
                            "Affected route: /{route}"
                        }
                    }
                }
                div {
                    class: "mt-8",
                    div {
                        class: "mt-6",
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
}
