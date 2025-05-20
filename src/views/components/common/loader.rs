use dioxus::prelude::*;

/// Loader Component
///
/// This component displays a loading indicator.
///
/// **Returns**  
///
/// * `Element` - The loading component
#[component]
pub fn LoaderComponent() -> Element {
    rsx! {
        div {
            class: "spinner",
        }
    }
}

/// Children Or Loading Component
///
/// This component displays a loading indicator while children are loading.
///
/// **Arguments**  
///
/// * `children` - The children to display while loading
///
/// **Returns**  
///
/// * `Element` - The children or loading component
#[component]
pub fn ChildrenOrLoading(children: Element) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/public/loading.css")
        }
        SuspenseBoundary {
            fallback: |context: SuspenseContext| {
                rsx! {
                    if let Some(placeholder) = context.suspense_placeholder() {
                        {placeholder}
                    } else {
                        LoaderComponent {}
                    }
                }
            },
            {children}
        }
    }
}
