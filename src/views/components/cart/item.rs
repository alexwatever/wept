use dioxus::prelude::*;

use crate::{
    app::state::STATE,
    controllers::cart::CartController,
    graphql::models::cart::{cart_query, cart_query::CartQueryCartContentsNodesProductNode},
};

#[component]
pub fn CartItem(item: cart_query::CartQueryCartContentsNodes) -> Element {
    let product_name = if let Some(product) = &item.product {
        match &product.node {
            CartQueryCartContentsNodesProductNode::SimpleProduct(sp) => {
                sp.name.clone().unwrap_or_default()
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };

    let key_decrease = item.key.clone();
    let key_increase = item.key.clone();
    let key_remove = item.key.clone();
    let quantity = item.quantity.unwrap_or(0);
    let mut cart_controller = use_signal(CartController::new);

    rsx! {
        div { class: "flex items-center justify-between py-4 border-b",
            div { class: "flex items-center space-x-4",
                span { "{product_name}" }
            }
            div { class: "flex items-center space-x-4",
                div { class: "flex items-center space-x-2",
                    button {
                        class: "px-2 py-1 bg-gray-200 rounded",
                        onclick: move |_| {
                            let key_decrease = key_decrease.clone();
                            spawn(async move {
                                if let Err(e) = cart_controller.write().update_item_quantity(key_decrease, quantity - 1).await {
                                    tracing::error!("Error updating item quantity: {}", e);
                                }
                                if let Ok(Some(cart_data)) = cart_controller.write().get_cart().await {
                                    if let Some(cart) = cart_data.cart {
                                        let mut state = STATE.write();
                                        if let Some(contents) = cart.contents {
                                            state.cart.items = contents.nodes.into_iter().collect();
                                        }
                                        state.cart.total = cart.total.unwrap_or_default();
                                        state.cart.subtotal = cart.subtotal.unwrap_or_default();
                                        state.save_cart();
                                    }
                                }
                            });
                        },
                        "-"
                    }
                    span { "{quantity}" }
                    button {
                        class: "px-2 py-1 bg-gray-200 rounded",
                        onclick: move |_| {
                            let key_increase = key_increase.clone();
                            spawn(async move {
                                if let Err(e) = cart_controller.write().update_item_quantity(key_increase, quantity + 1).await {
                                    tracing::error!("Error updating item quantity: {}", e);
                                }
                                if let Ok(Some(cart_data)) = cart_controller.write().get_cart().await {
                                    if let Some(cart) = cart_data.cart {
                                        let mut state = STATE.write();
                                        if let Some(contents) = cart.contents {
                                            state.cart.items = contents.nodes.into_iter().collect();
                                        }
                                        state.cart.total = cart.total.unwrap_or_default();
                                        state.cart.subtotal = cart.subtotal.unwrap_or_default();
                                        state.save_cart();
                                    }
                                }
                            });
                        },
                        "+"
                    }
                }
                button {
                    class: "text-red-500",
                    onclick: move |_| {
                        let key_remove = key_remove.clone();
                        spawn(async move {
                            if let Err(e) = cart_controller.write().remove_item_from_cart(key_remove).await {
                                tracing::error!("Error removing item from cart: {}", e);
                            }
                            if let Ok(Some(cart_data)) = cart_controller.write().get_cart().await {
                                if let Some(cart) = cart_data.cart {
                                    let mut state = STATE.write();
                                    if let Some(contents) = cart.contents {
                                        state.cart.items = contents.nodes.into_iter().collect();
                                    }
                                    state.cart.total = cart.total.unwrap_or_default();
                                    state.cart.subtotal = cart.subtotal.unwrap_or_default();
                                    state.save_cart();
                                }
                            }
                        });
                    },
                    "Remove"
                }
            }
        }
    }
}
