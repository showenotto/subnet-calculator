// src/app.rs
use dioxus::prelude::*;
use crate::{components::{ActiveTab, Footer, Header, Tabs}, ipv4::Ipv4Tab, ipv6::Ipv6Tab};
use crate::assistant::chat_panel::AssistantTab;

#[derive(Props, Clone, PartialEq)]
struct PlaceholderProps {
    name: &'static str,
}

pub fn app() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Ipv4);
    let mut is_loading = use_signal(|| true);

    let window = dioxus::desktop::use_window();
    use_future(move || {
        let value = window.clone();
        async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            
            // 2. Reveal the window
            value.set_visible(true);
            // Wait for 2 seconds
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            is_loading.set(false);
        }
    });

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }

        if *is_loading.read() {
            // This is your Splash Screen
            div { 
                class: "fixed inset-0 z-50 flex flex-col items-center justify-center bg-gray-900",
                
                // The Spiral/Spinner
                div { 
                    class: "relative w-20 h-20",
                    // Outer spinning ring
                    div { 
                        class: "absolute inset-0 border-4 border-blue-600/20 border-t-blue-600 rounded-full animate-spin"
                    }
                }
            }
        }
        else {
            div { class: "flex flex-col min-h-screen bg-gray-900 text-gray-100",
                Header {}
                Tabs {
                    active_tab: *active_tab.read(),
                    on_tab_change: move |tab| active_tab.set(tab)
                }
                main { class: "flex-1 mx-auto px-4 sm:px-6 lg:px-8 w-full font-roboto",
                    // IPv4 Tab Content
                    div { class: if *active_tab.read() == ActiveTab::Ipv4 { "" } else { "hidden" },
                        div { class: "text-center py-1",
                            h2 { class: "text-2xl font-bold mb-4", "IPv4 Subnet Calculator" }
                            Ipv4Tab {}
                        }
                    }

                    // IPv6 Tab Content
                    div { class: if *active_tab.read() == ActiveTab::Ipv6 { "" } else { "hidden" },
                        div { class: "text-center py-1",
                            h2 { class: "text-2xl font-bold mb-4", "IPv6 Subnet Calculator"}
                            Ipv6Tab {}
                        }
                    }

                    // AI Assistant Tab Content
                    div { class: if *active_tab.read() == ActiveTab::Assistant { "" } else { "hidden" },
                        div { class: "text-center py-1",
                            h2 { class: "text-2xl font-bold mb-4", "AI Assistant"}
                            AssistantTab {}
                        }
                    }
                }
                Footer {}
            }
        }
    }
}