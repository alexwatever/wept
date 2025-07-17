use dioxus::prelude::*;

#[component]
pub fn Card(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "p-6 bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 ease-in-out",
            h2 {
                class: "text-2xl font-bold text-gray-800 mb-2",
                "{title}"
            }
            {children}
        }
    }
}
