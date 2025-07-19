use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::{app::state::STATE, controllers::cart::CartController, models::product::Product};

#[component]
pub fn AddToCartForm(product: Product) -> Element {
    rsx! {
        div { class: "flex flex-wrap -mx-4 mb-14 items-center",
            div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
                button {
                    class: "block bg-orange-300 hover:bg-orange-400 text-center text-white font-bold font-heading py-5 px-8 rounded-md uppercase transition duration-200",
                    onclick: move |_| {
                        if let Some(product_id) = product.database_id {
                            let mut cart_controller = CartController::new();
                            spawn(async move {
                                match cart_controller.add_to_cart(product_id, 1).await {
                                    Ok(_) => {
                                        // Refetch cart after adding an item
                                        match cart_controller.get_cart().await {
                                            Ok(Some(response_data)) => {
                                                if let Some(cart) = response_data.cart {
                                                    let mut state = STATE.write();
                                                    if let Some(contents) = cart.contents {
                                                        state.cart.items =
                                                            contents.nodes.into_iter().collect();
                                                    }
                                                    state.cart.total = cart.total.unwrap_or_default();
                                                    state.cart.subtotal =
                                                        cart.subtotal.unwrap_or_default();

                                                    // Save cart to local storage
                                                    if let Err(e) =
                                                        LocalStorage::set("cart", state.cart.clone())
                                                    {
                                                        tracing::error!(
                                                            "Failed to save cart to local storage: {}",
                                                            e
                                                        );
                                                    }
                                                }
                                            }
                                            Ok(None) => {}
                                            Err(e) => {
                                                tracing::error!("Error refetching cart: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!("Error adding to cart: {}", e);
                                    }
                                }
                            });
                        }
                    },
                    "Add to cart"
                }
            }
        }
    }
}
