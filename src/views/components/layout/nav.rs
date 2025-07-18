use dioxus::prelude::*;
use dioxus_router::prelude::Link;

// Modules
use crate::{routes::Routes, views::components::search::search_bar::SearchBar};

/// Navigation component
#[component]
pub fn Nav() -> Element {
    rsx! {
        section { class: "relative",
            nav { class: "flex justify-between border-b",
                div { class: "px-12 py-8 flex w-full items-center",
                    // Navigation links
                    ul { class: "hidden xl:flex font-semibold font-heading",
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::HomePage {},
                                "Home"
                            }
                        }
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::CategoriesPage {},
                                "Categories"
                            }
                        }
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::PagesListPage {},
                                "Pages"
                            }
                        }
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::PostsPage {},
                                "Posts"
                            }
                        }
                    }

                    // Search
                    div { class: "hidden xl:inline-block w-full max-w-xs mr-14",
                        SearchBar {}
                    }
                }
            }
        }
    }
}
