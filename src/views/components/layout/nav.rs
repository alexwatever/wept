use dioxus::prelude::*;

// Modules
use crate::routes::Routes;

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
                    }

                    // Search
                    div { class: "hidden xl:inline-block mr-14",
                        input { class: "py-5 px-8 w-full placeholder-gray-400 text-xs uppercase font-semibold font-heading bg-gray-50 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
                            placeholder: "Search",
                            r#type: "text",
                        }
                    }
                }
            }
        }
    }
}
