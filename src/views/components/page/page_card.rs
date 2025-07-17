use dioxus::prelude::*;

use crate::{models::page::Page, routes::Routes, views::components::common::card::Card};

#[component]
pub fn PageCard(page: Page) -> Element {
    let page_url = Routes::PagePage {
        slug: page.slug.clone().unwrap_or_default(),
    };

    rsx! {
        Card {
            title: page.title.unwrap_or_default(),
            a {
                class: "text-blue-600 hover:text-blue-800 visited:text-purple-600 transition-colors duration-300 ease-in-out",
                href: "{page_url}",
                "Read more"
            }
        }
    }
}
