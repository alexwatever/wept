use dioxus::prelude::*;
use dioxus_router::prelude::navigator;

// Modules
use crate::routes::Routes;

/// Search bar component
#[component]
pub fn SearchBar() -> Element {
    let mut search_query = use_signal(String::new);

    rsx! {
        form {
            onsubmit: move |event| {
                if !search_query.read().trim().is_empty() {
                    let navigator = navigator();
                    navigator.push(Routes::SearchPage { query: search_query.read().clone() });
                }
                event.stop_propagation();
            },

            div {
                class: "relative",
                input {
                    r#type: "search",
                    class: "block w-full p-3 ps-5 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500",
                    placeholder: "Search products...",
                    required: true,
                    value: "{search_query}",
                    oninput: move |event| search_query.set(event.value().clone()),
                }
                button {
                    r#type: "submit",
                    class: "text-white absolute end-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-1",
                    "Search"
                }
            }
        }
    }
}
