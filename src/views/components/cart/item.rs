use dioxus::prelude::*;

// Modules
use crate::{
    app::state::STATE,
    controllers::cart::CartController,
    graphql::models::cart::{cart_query, cart_query::CartQueryCartContentsNodesProductNode},
};

/// # Cart Item Component
///
/// A component that displays a single item in the cart.
///
/// **Arguments**
///
/// * `item` - The item to display.
///
/// **Returns**
///
/// * `Element` - The rendered component.
#[component]
pub fn CartItem(item: cart_query::CartQueryCartContentsNodes) -> Element {
    // Get the product name
    let product_name = &item
        .product
        .map(|product| match product.node {
            CartQueryCartContentsNodesProductNode::SimpleProduct(sp) => sp.name.unwrap_or_default(),
            _ => String::new(),
        })
        .unwrap_or_default();

    // Get the quantity of the item
    let quantity = item.quantity.unwrap_or(0);

    // Get the cart controller
    let mut cart_controller = use_signal(CartController::new);

    // Decrease quantity method
    let key_decrease = item.key.clone();
    let decrease_fn = move |_| -> () {
        // Decrease the quantity of the item.
        let key_decrease = key_decrease.clone();
        spawn(async move {
            // Update the quantity of the item.
            if let Err(e) = cart_controller
                .write()
                .update_item_quantity(key_decrease, quantity - 1)
                .await
            {
                tracing::error!("Error updating item quantity: {}", e);
            }

            // Update the cart state.
            if let Ok(Some(cart_data)) = cart_controller.write().get_cart().await {
                if let Some(cart) = cart_data.cart {
                    // Update the cart state.
                    let mut state = STATE.write();

                    // Update the cart items.
                    if let Some(contents) = cart.contents {
                        state.cart.items = contents.nodes.into_iter().collect();
                    }

                    // Update the cart total.
                    state.cart.total = cart.total.unwrap_or_default();

                    // Update the cart subtotal.
                    state.cart.subtotal = cart.subtotal.unwrap_or_default();

                    // Save the cart state.
                    state.save_cart();
                }
            }
        });
    };

    // Increase quantity method
    let key_increase = item.key.clone();
    let increase_fn = move |_| -> () {
        // Increase the quantity of the item.
        let key_increase = key_increase.clone();
        spawn(async move {
            if let Err(e) = cart_controller
                .write()
                .update_item_quantity(key_increase, quantity + 1)
                .await
            {
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
    };

    // Remove item method
    let key_remove = item.key.clone();
    let remove_fn = move |_| -> () {
        // Remove the item from the cart.
        let key_remove = key_remove.clone();
        spawn(async move {
            if let Err(e) = cart_controller
                .write()
                .remove_item_from_cart(key_remove)
                .await
            {
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
    };

    // Render the cart item.
    rsx! {
        div { class: "flex items-center justify-between py-4 border-b",
            div { class: "flex items-center space-x-4",
                span { "{product_name}" }
            }
            div { class: "flex items-center space-x-4",
                div { class: "flex items-center space-x-2",
                    button {
                        class: "px-2 py-1 bg-gray-200 rounded",
                        onclick: decrease_fn,
                        "-"
                    }
                    span { "{quantity}" }
                    button {
                        class: "px-2 py-1 bg-gray-200 rounded",
                        onclick: increase_fn,
                        "+"
                    }
                }
                button {
                    class: "text-red-500",
                    onclick: remove_fn,
                    "Remove"
                }
            }
        }
    }
}
