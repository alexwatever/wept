use dioxus::prelude::*;

// # Modules
use crate::{
    model::products::{Product, Products},
    view::components::entity_list::EntityDisplayListComponent,
};

/// # Products Component
///
/// This component displays a list of products.  
///
/// **Arguments**  
///
/// * `products` - A signal containing a list of products.
///
/// **Returns**  
///
/// The ProductsComponent element.
#[component]
#[allow(non_snake_case)]
pub(crate) fn ProductsComponent(products: Signal<Products>) -> Element {
    let items: Signal<Vec<Product>> = Signal::new(products.read().0.clone());

    rsx! {
        EntityDisplayListComponent {
            entities: items
        }
    }
}
