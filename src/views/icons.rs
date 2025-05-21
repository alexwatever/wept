use dioxus::prelude::*;

/// Hamburger menu icon
pub fn hamburger_menu() -> Element {
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
