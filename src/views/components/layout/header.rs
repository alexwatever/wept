use dioxus::prelude::*;

// Modules
use crate::{
    controllers::settings::SettingsController,
    views::{
        components::{common::loader::LoaderComponent, layout::nav::Nav},
        icons::CartIcon,
    },
};

/// Header component
#[component]
pub fn Header() -> Element {
    let settings = SettingsController::new();
    let settings_resource = use_resource(move || {
        let settings = settings.clone();
        async move { settings.get().await }
    });

    rsx! {
        header {
            class: "bg-white shadow",
            div {
                class: "container mx-auto px-4",
                div {
                    class: "flex justify-between items-center py-4",
                    div {
                        match &*settings_resource.value().read_unchecked() {
                            Some(Ok(Some(data))) => {
                                if let Some(settings) = data.page.as_ref().and_then(|p| p.wept_settings.as_ref()) {
                                    rsx! {
                                        // Site Logo
                                        if let Some(logo) = &settings.site_logo {
                                            a { href: "/",
                                                img {
                                                    class: "h-10",
                                                    src: "{logo.source_url.as_deref().unwrap_or_default()}",
                                                    alt: "{logo.alt_text.as_deref().unwrap_or_default()}"
                                                }
                                            }
                                        }

                                        // Site Name
                                        if let Some(site_name) = &settings.site_name {
                                             if !site_name.is_empty() {
                                                a { href: "/",
                                                    h1 {
                                                        class: "text-2xl font-bold",
                                                        "{site_name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                     rsx! { div { "Settings not found." } }
                                }
                            }
                            Some(Ok(None)) => rsx! { div { "No settings data available." } },
                            Some(Err(e)) => rsx! { div { "Error: {e}" } },
                            None => rsx! { LoaderComponent {} }
                        }
                    }

                    // Cart
                    div {
                        CartIcon {}
                    }

                    // Navigation
                    div {
                        class: "flex items-center space-x-4",
                        Nav {},
                    }
                }
            }
        }
    }
}
