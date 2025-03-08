use dioxus::prelude::*;

/// # Loader Component
///
/// This component displays a loading indicator.  
///
/// **Returns**  
///
/// The LoaderComponent element.
#[component]
pub fn LoaderComponent() -> Element {
    rsx! {
        div {
            class: "spinner",
        }
    }
}

#[component]
pub(crate) fn ChildrenOrLoading(children: Element) -> Element {
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
