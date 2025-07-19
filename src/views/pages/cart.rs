use dioxus::prelude::*;

// Modules
use crate::app::state::STATE;

/// Cart page component
#[component]
pub fn CartPage() -> Element {
    let state = STATE.read();
    let cart = &state.cart;

    rsx!(
        div {
            class: "container mx-auto p-4",
            h1 {
                class: "text-3xl font-bold mb-8",
                "Your Cart"
            }

            if cart.items.is_empty() {
                p { "Your cart is empty." }
            } else {
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    div {
                        class: "md:col-span-2",
                        for _item in &cart.items {
                            div {
                                class: "flex items-center border-b py-4",
                                "Item details here"
                            }
                        }
                    }
                    div {
                        class: "bg-gray-100 p-8",
                        h2 {
                            class: "text-2xl font-bold mb-4",
                            "Summary"
                        }
                        div {
                            class: "flex justify-between mb-2",
                            span { "Subtotal" }
                            span { "{cart.subtotal}" }
                        }
                        div {
                            class: "flex justify-between font-bold text-xl",
                            span { "Total" }
                            span { "{cart.total}" }
                        }
                    }
                }
            }
        }
    )
}
