use dioxus::prelude::*;

use crate::{app::state::STATE, routes::Routes};

#[component]
fn Icon(cx: Scope) -> Element {
    let state = use_state(cx, || STATE.clone());
    let routes = use_state(cx, || Routes::new());

    let cart_icon = if state.cart.is_empty() {
        svg!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shopping-cart"><path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2H6z"/><path d="M3 6h18"/><path d="M16 10a4 4 0 1 1-8 0"/><circle cx="12" cy="10" r="4"/></svg>"#
        )
    } else {
        svg!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shopping-cart"><path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2H6z"/><path d="M3 6h18"/><path d="M16 10a4 4 0 1 1-8 0"/><circle cx="12" cy="10" r="4"/></svg>"#
        )
    };

    let cart_count = state.cart.len();

    let cart_link = rsx! {
        a {
            href: "/cart",
            class: "relative",
            cart_icon,
            span {
                class: "absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-4 w-4 flex items-center justify-center",
                cart_count
            }
        }
    };

    cart_link
}
