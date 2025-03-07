use dioxus::prelude::*;

// # Modules
use crate::{
    controller::products::fetch_products, model::pagination::PageSort,
    view::components::product_item::product_item,
};

#[allow(non_snake_case)]
pub(crate) fn HomePage() -> Element {
    let products = use_server_future(|| fetch_products(10, PageSort::Ascending))?;
    let products = products().unwrap()?;

    rsx! {
        section { class: "p-10",
            for product in products {
                product_item {
                    product
                }
            }
        }
    }
}
