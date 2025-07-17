use crate::{
    models::{category::ProductCategory, pagination::Pagination},
    routes::Routes,
    views::components::common::loader::LoaderComponent,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryListProps {
    categories: Vec<ProductCategory>,
    page_info: Option<Pagination>,
    is_loading: bool,
    on_load_more: EventHandler<()>,
}

#[component]
pub fn CategoryList(props: CategoryListProps) -> Element {
    let categories_to_display = &props.categories;
    let show_load_more_button = props.page_info.as_ref().is_some_and(|pi| pi.has_next_page);
    let is_loading_more = props.is_loading;

    rsx! {
        if categories_to_display.is_empty() && is_loading_more {
            LoaderComponent {}
        } else if categories_to_display.is_empty() {
            div { class: "text-center py-8",
                p { class: "text-lg text-gray-500", "No categories found." }
            }
        } else {
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-8",
                for category_item in categories_to_display.iter() {
                    div { class: "bg-white rounded-lg shadow-md overflow-hidden transform hover:scale-105 transition-transform duration-300",
                        Link {
                            to: Routes::CategoryPage { slug: category_item.slug.as_ref().cloned().unwrap_or_default() },
                            if let Some(img_src) = category_item.image.as_ref().and_then(|img| img.source_url.as_ref()) {
                                img { class: "w-full h-40 object-cover", src: "{img_src}", alt: "{category_item.name.as_deref().unwrap_or(\"Category Image\")}" }
                            } else {
                                div { class: "w-full h-40 bg-gray-200" }
                            }
                            div { class: "p-4",
                                h3 { class: "text-lg font-semibold text-gray-800 truncate", title: "{category_item.name.as_deref().unwrap_or_default()}", "{category_item.name.as_deref().unwrap_or_default()}" }
                                p { class: "text-sm text-gray-600", "Products: {category_item.count.map(|c| c.to_string()).as_deref().unwrap_or(\"N/A\")}" }
                            }
                        }
                    }
                }
            }
            if is_loading_more {
                div { class: "flex justify-center py-8", LoaderComponent {} }
            }
            if show_load_more_button && !is_loading_more {
                div { class: "text-center mt-8",
                    button {
                        class: "px-6 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300",
                        onclick: move |_| props.on_load_more.call(()),
                        "Load More Categories"
                    }
                }
            }
        }
    }
}
