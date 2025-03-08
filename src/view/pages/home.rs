use dioxus::prelude::*;
use tracing::error;

// # Modules
use crate::{
    model::{pagination::PageSort, product::Products},
    view::components::products::ProductsComponent,
};

#[allow(non_snake_case)]
pub(crate) fn HomePage() -> Element {
    let mut products: Signal<Products> = use_signal(|| Products(vec![]));

    // Get products asyncronously
    let get_products: Resource<()> = use_resource(move || async move {
        // Fetch products
        let update: Products = Products::get(10, PageSort::Ascending)
            .await
            .unwrap_or_else(|err| {
                error!("Error fetching products: {err}");
                Products(vec![])
            });

        // Update the `products` signal
        products.set(update);
    });
    get_products();

    // # Render page
    rsx! {
        ProductsComponent { products }
    }
}
