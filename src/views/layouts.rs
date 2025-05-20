use dioxus::prelude::*;

// Modules
use crate::routes::Routes;
use crate::views::components::{common::loader::ChildrenOrLoading, layout::nav::Nav};

/// Main layout component
#[component]
pub fn MainLayout() -> Element {
    rsx! {
        // Navigation component
        Nav {}

        // Outlet for the routes
        ChildrenOrLoading {
            Outlet::<Routes> {}
        }
    }
}
