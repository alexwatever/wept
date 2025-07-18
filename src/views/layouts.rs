use dioxus::prelude::*;

// Modules
use crate::{
    routes::Routes,
    views::components::{
        common::loader::ChildrenOrLoading,
        layout::{footer::Footer, header::Header},
    },
};

/// Main layout component
#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen",

            // Header component
            Header {}

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
