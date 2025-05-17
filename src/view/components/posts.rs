use dioxus::prelude::*;

// # Modules
use crate::{
    model::posts::{Post, Posts},
    view::components::entity_list::EntityDisplayListComponent,
};

/// # Posts Component
///
/// This component displays a list of posts.  
///
/// **Arguments**  
///
/// * `posts` - A signal containing a list of posts.
///
/// **Returns**  
///
/// The PostsComponent element.
#[component]
#[allow(non_snake_case)]
pub(crate) fn PostsComponent(posts: Signal<Posts>) -> Element {
    let items: Signal<Vec<Post>> = Signal::new(posts.read().0.clone());

    rsx! {
        EntityDisplayListComponent {
            entities: items
        }
    }
}
