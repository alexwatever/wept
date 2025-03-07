use dioxus::{launch, prelude::*};

// # Modules
mod controller;
mod model;
mod routes;
mod view;
use routes::Routes;
use view::components::{loader::ChildrenOrLoading, nav::Nav};

fn main() {
    launch(|| {
        rsx! {
            document::Link {
                rel: "stylesheet",
                href: asset!("/public/tailwind.css")
            }

            Nav {}

            ChildrenOrLoading {
                Router::<Routes> {}
            }
        }
    });
}
