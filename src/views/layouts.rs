use dioxus::prelude::*;

// Modules
use crate::{
    routes::Routes,
    views::components::{
        common::loader::ChildrenOrLoading,
        layout::{footer::Footer, nav::Nav},
    },
};

/// Main layout component
#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen",

            // Navigation component
            Nav {}

            // Main content
            main {
                class: "flex-grow",
                ChildrenOrLoading {
                    Outlet::<Routes> {}
                }
            }

            // Footer component
            Footer {}
        }
    }
}
