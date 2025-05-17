use dioxus::prelude::*;

// # Modules
use crate::view::components::loader::LoaderComponent;

/// Entity Display trait
///
/// This trait should be implemented by any component that wants
/// to be displayed in a list.
pub trait EntityDisplay: Clone + PartialEq + 'static {
    /// Render the entity
    fn render(&self) -> Element;
}

/// # Generic Entity List Component
///
/// A reusable component for displaying lists of entities.
///
/// **Arguments**  
///
/// * `entities` - A signal containing a list of entities
/// * `render_item` - A function that renders each entity
///
/// **Returns**  
///
/// The EntityListComponent element
#[component]
#[allow(non_snake_case)]
pub(crate) fn EntityListComponent<
    T: Clone + 'static + PartialEq,
    F: Fn(T) -> Element + 'static + PartialEq,
>(
    entities: Signal<Vec<T>>,
    render_item: F,
) -> Element {
    let items = entities.read();

    if !items.is_empty() {
        rsx! {
            section { class: "p-10",
                for item in items.iter() {
                    {render_item(item.clone())}
                }
            }
        }
    } else {
        rsx! {
            section { class: "p-10",
                section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center",
                    div { class: "pl-4 text-left text-ellipsis",
                        LoaderComponent {}
                    }
                }
            }
        }
    }
}

/// # Entity List Component with Default Rendering
///
/// A reusable component for displaying lists of entities that implement EntityDisplay.
///
/// **Arguments**  
///
/// * `entities` - A signal containing a list of entities that implement EntityDisplay
///
/// **Returns**  
///
/// The EntityDisplayListComponent element
#[component]
#[allow(non_snake_case)]
pub(crate) fn EntityDisplayListComponent<T: EntityDisplay + PartialEq>(
    entities: Signal<Vec<T>>,
) -> Element {
    rsx! {
        div {
            // Use a separate component to avoid closure comparison issues
            EntityListRenderer {
                entities: entities
            }
        }
    }
}

// Helper component to avoid closure comparison issues
#[component]
fn EntityListRenderer<T: EntityDisplay + PartialEq>(entities: Signal<Vec<T>>) -> Element {
    let items = entities.read();

    if !items.is_empty() {
        rsx! {
            section { class: "p-10",
                for item in items.iter() {
                    {item.render()}
                }
            }
        }
    } else {
        rsx! {
            section { class: "p-10",
                section { class: "p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center",
                    div { class: "pl-4 text-left text-ellipsis",
                        LoaderComponent {}
                    }
                }
            }
        }
    }
}
