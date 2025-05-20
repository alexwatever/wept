use dioxus::prelude::*;

// Modules
use crate::{routes::Routes, views::icons};

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
                                to: Routes::NotFoundPage { route: vec!["category".to_string()], message: None },
                                "Category"
                            }
                        }
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::NotFoundPage { route: vec!["collection".to_string()], message: None },
                                "Collection"
                            }
                        }
                        li { class: "mr-12",
                            Link { class: "hover:text-gray-600",
                                to: Routes::NotFoundPage { route: vec!["story".to_string()], message: None },
                                "Story"
                            }
                        }
                    }

                    // Logo
                    Link { class: "flex-shrink-0 xl:mx-auto text-3xl font-bold font-heading",
                        to: Routes::HomePage {},
                        img { class: "h-9",
                            width: "auto",
                            alt: "",
                            src: "https://shuffle.dev/yofte-assets/logos/yofte-logo.svg",
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

                // Burger menu
                a { class: "navbar-burger self-center mr-12 xl:hidden",
                    href: "#",
                    icons::hamburger {}
                }
            }
        }
    }
}
