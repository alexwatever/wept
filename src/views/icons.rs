use dioxus::prelude::*;

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
            view_box: "0 0 24 24",
            path {
                d: "M13.795 10.533 20.68 2h-3.073l-5.255 6.517L6.952 2H2l7.304 10.513L2 22h3.074l5.643-6.905L16.471 22h4.952l-7.628-11.467Zm-2.38 2.95L9.94 11.432l-1.42-2.025 5.344-7.65h2.38l-5.324 7.632 1.53 2.186 1.42 2.025-5.556 8.01h-2.38l5.536-7.986Z"
            }
        }
    }
}
