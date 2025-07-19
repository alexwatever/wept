// Modules
use dioxus::prelude::*;

use crate::app::state::STATE;

#[component]
pub fn CartIcon() -> Element {
    let state = STATE.read();

    let cart_icon = rsx! {
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
        }
    };

    let cart_count = state
        .cart
        .items
        .iter()
        .map(|item| item.quantity.unwrap_or(0))
        .sum::<i64>();

    rsx! {
        a {
            href: "/cart",
            class: "relative",
            {cart_icon},
            if cart_count > 0 {
                span {
                    class: "absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-4 w-4 flex items-center justify-center",
                    "{cart_count}"
                }
            }
        }
    }
}
