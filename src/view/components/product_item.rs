use dioxus::prelude::*;

// # Modules
use crate::model::products::Product;

#[component]
pub(crate) fn product_item(product: Product) -> Element {
    let Product {
        id,
        sku,
        slug,
        name,
        status,
        description,
        ..
    } = product;

    rsx! {
        section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
            // img {
            //     class: "object-scale-down w-1/6 h-full",
            //     src: "{image}",
            // }
            div { class: "pl-4 text-left text-ellipsis",
                a {
                    href: "/details/{id}",
                    class: "w-full text-center",
                    "{name:?}"
                }
                p {
                    class: "w-full",
                    "{slug:?}"
                }
                p {
                    class: "w-full",
                    "{description:?}"
                }
                p {
                    class: "w-full",
                    "{status:?}"
                }
                p {
                    class: "w-1/4",
                    "{sku:?}"
                }
            }
        }
    }
}
