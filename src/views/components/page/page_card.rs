use dioxus::prelude::*;

use crate::{models::page::Page, routes::Routes};

#[component]
pub fn PageCard(page: Page) -> Element {
    let page_url = Routes::PagePage {
        slug: page.slug.clone().unwrap_or_default(),
    };

    rsx! {
        div {
            class: "p-6 bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 ease-in-out",
            h2 {
                class: "text-2xl font-bold text-gray-800 mb-2",
                "{page.title.as_deref().unwrap_or_default()}"
            }
            a {
                class: "text-blue-600 hover:text-blue-800 visited:text-purple-600 transition-colors duration-300 ease-in-out",
                href: "{page_url}",
                "Read more"
            }
        }
    }
}
