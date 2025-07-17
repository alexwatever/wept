use dioxus::prelude::*;

// Modules
use crate::{
    models::{category::ProductCategory, page::Page, post::Post, product::Product},
    views::components::{
        category::category_card::ProductCategoryCard, page::page_card::PageCard,
        post::post_card::PostCard, product::product_card::ProductCard,
    },
};

/// Entity List component
///
/// A reusable component for displaying lists of entities that implement EntityDisplay.
///
/// **Arguments**  
///
/// * `entities` - The list of entities to render
///
/// **Returns**
///
/// * `Element` - The entity list component
#[component]
pub fn EntityDisplayListComponent<T: EntityDisplay>(entities: Vec<T>) -> Element {
    if !entities.is_empty() {
        rsx! {
            div { class: "p-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for item in entities.iter() {
                    {item.render()}
                }
            }
        }
    } else {
        rsx! {}
    }
}

/// Entity Display trait
///
/// This trait should be implemented by any component that wants to be displayed in a list.
///
/// **Returns**
///
/// * `Element` - The entity component
pub trait EntityDisplay: Clone + PartialEq + 'static {
    /// Render the entity
    fn render(&self) -> Element;
}

/// Implementation of EntityDisplay for Post
///
/// **Arguments**
///
/// * `self` - The Post to render
///
/// **Returns**
///
/// * `Element` - A Post component in list format
impl EntityDisplay for Post {
    fn render(&self) -> Element {
        rsx! {
            PostCard { post: self.clone() }
        }
    }
}

/// Implementation of EntityDisplay for Page
///
/// **Arguments**
///
/// * `self` - The Page to render
///
/// **Returns**
///
/// * `Element` - A Page component in list format
impl EntityDisplay for Page {
    fn render(&self) -> Element {
        rsx! {
            PageCard { page: self.clone() }
        }
    }
}

/// Implementation of EntityDisplay for Product
///
/// **Arguments**
///
/// * `self` - The Product to render
///
/// **Returns**
///
/// * `Element` - A Product component in list format
impl EntityDisplay for Product {
    fn render(&self) -> Element {
        rsx! {
            ProductCard { product: self.clone() }
        }
    }
}

/// Implementation of EntityDisplay for ProductCategory
///
/// **Arguments**
///
/// * `self` - The ProductCategory to render
///
/// **Returns**
///
/// * `Element` - A ProductCategory component in list format
impl EntityDisplay for ProductCategory {
    fn render(&self) -> Element {
        rsx! {
            ProductCategoryCard { category: self.clone() }
        }
    }
}
