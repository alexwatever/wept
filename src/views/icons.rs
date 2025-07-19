use dioxus::prelude::*;

// Modules
use crate::{app::state::STATE, routes::Routes};

/// Cart icon component
#[component]
pub fn CartIcon() -> Element {
    let state = STATE.read();

    let cart_count = state
        .cart
        .items
        .iter()
        .map(|item| item.quantity.unwrap_or(0))
        .sum::<i64>();

    rsx! {
        Link {
            to: Routes::CartPage {},
            class: "relative",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "lucide lucide-shopping-cart",
                path { d: "M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" }
                path { d: "M3 6h18" }
                path { d: "M16 10a4 4 0 0 1-8 0" }
            },
            if cart_count > 0 {
                span {
                    class: "absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-4 w-4 flex items-center justify-center",
                    "{cart_count}"
                }
            }
        }
    }
}

/// Hamburger menu icon
pub fn _hamburger_menu() -> Element {
    rsx! {
        svg {
            height: "19",
            width: "25",
            fill: "none",
            view_box: "0 0 25 19",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                stroke_width: "1.5",
                d: "M1.25 17.25H23.75C24.3023 17.25 24.75 16.8023 24.75 16.25V2.75C24.75 2.19772 24.3023 1.75 23.75 1.75H1.25C0.697715 1.75 0.25 2.19772 0.25 2.75V16.25C0.25 16.8023 0.697715 17.25 1.25 17.25Z",
                stroke: "#141414",
                stroke_linejoin: "round",
                stroke_linecap: "round",
            }
            path {
                stroke_width: "1.5",
                d: "M5.75 5H19.25",
                stroke: "#141414",
                stroke_linejoin: "round",
                stroke_linecap: "round",
            }
            path {
                d: "M5.75 9.5H19.25",
                stroke: "#141414",
                stroke_linejoin: "round",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
            path {
                stroke_linecap: "round",
                stroke_width: "1.5",
                d: "M5.75 14H19.25",
                stroke: "#141414",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn Facebook() -> Element {
    rsx! {
        svg {
            class: "w-6 h-6",
            "aria-hidden": "true",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "currentColor",
            view_box: "0 0 8 19",
            path {
                "fill-rule": "evenodd",
                d: "M6.135 3H8V0H6.135a4.147 4.147 0 0 0-4.142 4.142V6H0v3h2v9.938h3V9h2.021l.592-3H5V3.591A.6.6 0 0 1 5.592 3h.543Z",
                "clip-rule": "evenodd"
            }
        }
    }
}

#[component]
pub fn Instagram() -> Element {
    rsx! {
        svg {
            class: "w-6 h-6",
            "aria-hidden": "true",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "currentColor",
            view_box: "0 0 16 16",
            path {
                d: "M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.917 3.917 0 0 0-1.417.923A3.927 3.927 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.916 3.916 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.926 3.926 0 0 0-.923-1.417A3.911 3.911 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0h.003zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599.28.28.453.546.598.92.11.282.24.705.275 1.485.039.843.047 1.096.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.47 2.47 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.478 2.478 0 0 1-.92-.598 2.48 2.48 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.231 0-2.136.008-2.388.046-3.231.036-.78.166-1.204.275-1.486.145-.373.319-.64.599-.92.28-.28.546-.453.92-.598.282-.11.705-.24 1.485-.276.843-.038 1.096-.047 3.232-.047h.001zm4.908 1.169a1.448 1.448 0 1 0 0 2.897 1.448 1.448 0 0 0 0-2.897zM8 3.881a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217zm0 1.442a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334z"
            }
        }
    }
}

#[component]
pub fn Twitter() -> Element {
    rsx! {
        svg {
            class: "w-6 h-6",
            "aria-hidden": "true",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path {
                d: "M13.795 10.533 20.68 2h-3.073l-5.255 6.517L6.952 2H2l7.304 10.513L2 22h3.074l5.643-6.905L16.471 22h4.952l-7.628-11.467Zm-2.38 2.95L9.94 11.432l-1.42-2.025 5.344-7.65h2.38l-5.324 7.632 1.53 2.186 1.42 2.025-5.556 8.01h-2.38l5.536-7.986Z"
            }
        }
    }
}
