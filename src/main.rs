use dioxus::launch;
use dioxus::prelude::*;

// Modules
mod app;
mod controllers;
mod graphql;
mod models;
mod routes;
mod views;
use routes::Routes;

fn main() {
    // Initialize tracing subscriber for debug builds
    #[cfg(debug_assertions)]
    {
        #[cfg(target_arch = "wasm32")]
        tracing_wasm::set_as_global_default();

        #[cfg(not(target_arch = "wasm32"))]
        tracing_subscriber::fmt::init();
    }

    launch(move || {
        rsx! {
            // Tailwind stylesheet
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }

            // The Router component
            Router::<Routes> {}
        }
    });
}
