use dioxus::prelude::*;

// Modules
use crate::{
    controllers::settings::SettingsController,
    views::{
        components::common::loader::LoaderComponent,
        icons::{Facebook, Instagram, Twitter},
    },
};

/// Footer component
#[component]
pub fn Footer() -> Element {
    // Get settings data
    let settings = SettingsController::new();
    let settings = use_resource(move || {
        let settings = settings.clone();
        async move { settings.get().await }
    });

    rsx! {
        footer {
            class: "bg-gray-800 text-white p-8",
            div {
                class: "container mx-auto",
                match &*settings.value().read_unchecked() {
                    Some(Ok(Some(data))) => {
                        let settings = data.page.as_ref().and_then(|p| p.wept_settings.as_ref());
                        if let Some(settings) = settings {
                            rsx! {
                                div {
                                    class: "flex flex-col md:flex-row justify-between items-center",
                                    div {
                                        class: "mb-4 md:mb-0",
                                        if let Some(copyright) = &settings.copyright {
                                            p { class: "text-sm", "{copyright}" }
                                        }
                                        if let Some(email) = &settings.email {
                                            p { class: "text-sm",
                                                a { href: "mailto:{email}", class: "hover:underline", "{email}" }
                                            }
                                        }
                                        if let Some(phone) = &settings.phone {
                                            p { class: "text-sm",
                                                a { href: "tel:{phone}", class: "hover:underline", "{phone}" }
                                            }
                                        }
                                        if let Some(address) = &settings.address {
                                            p { class: "text-sm", "{address}" }
                                        }
                                    }
                                    div {
                                        class: "flex space-x-4",
                                        if let Some(facebook) = &settings.facebook {
                                            if !facebook.is_empty() {
                                                a { href: "{facebook}", target: "_blank", rel: "noopener noreferrer", class: "hover:text-gray-400", Facebook {} }
                                            }
                                        }
                                        if let Some(instagram) = &settings.instagram {
                                            if !instagram.is_empty() {
                                                a { href: "{instagram}", target: "_blank", rel: "noopener noreferrer", class: "hover:text-gray-400", Instagram {} }
                                            }
                                        }
                                        if let Some(twitter) = &settings.twitter {
                                            if !twitter.is_empty() {
                                                a { href: "{twitter}", target: "_blank", rel: "noopener noreferrer", class: "hover:text-gray-400", Twitter {} }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! { div { "Footer settings not found." } }
                        }
                    }
                    Some(Ok(None)) => rsx! { div { "No footer data available." } },
                    Some(Err(e)) => rsx! { div { "Error loading footer: {e}" } },
                    None => rsx! { LoaderComponent {} },
                }
            }
        }
    }
}
