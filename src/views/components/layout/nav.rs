use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{
    controllers::navigation::NavigationController,
    routes::Routes,
    views::components::{common::loader::LoaderComponent, search::search_bar::SearchBar},
};

/// Navigation component
#[component]
pub fn Nav() -> Element {
    let navigation_controller = NavigationController::new();
    let menu_items = use_resource(move || {
        let navigation_controller = navigation_controller.clone();
        async move { navigation_controller.get_menu("Header Menu").await }
    });

    rsx! {
        section {
            class: "relative",
            nav {
                class: "flex justify-between border-b",
                div {
                    class: "px-12 py-8 flex w-full items-center",
                    // Navigation links
                    ul {
                        class: "hidden xl:flex font-semibold font-heading",
                        match &*menu_items.value().read_unchecked() {
                            Some(Ok(Some(data))) => {
                                if let Some(menu) = &data.menu {
                                    if let Some(nodes) = &menu.menu_items {
                                        rsx! {
                                            for item in &nodes.nodes {
                                                if let Some(item_path) = item.path.as_ref() {
                                                    li {
                                                        class: "mr-12",
                                                        Link {
                                                            class: "hover:text-gray-600",
                                                            to: Routes::from_path(item_path),
                                                            "{item.label.as_ref().unwrap_or(&\"\".to_string())}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! { "No menu items found" }
                                    }
                                } else {
                                    rsx! { "Menu not found" }
                                }
                            }
                            Some(Ok(None)) => rsx! { "No data" },
                            Some(Err(e)) => rsx! { "Error: {e}" },
                            None => rsx! { LoaderComponent {} },
                        }
                    }

                    // Search
                    div {
                        class: "hidden xl:inline-block w-full max-w-xs mr-14",
                        SearchBar {}
                    }
                }
            }
        }
    }
}
