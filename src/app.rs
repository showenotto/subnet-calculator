// src/app.rs
use dioxus::prelude::*;
use crate::{components::{ActiveTab, Footer, Header, Tabs}, ipv4::Ipv4Tab, ipv6::Ipv6Tab};

#[derive(Props, Clone, PartialEq)]
struct PlaceholderProps {
    name: &'static str,
}

pub fn App() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Ipv4);

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        div { class: "flex flex-col min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            Header {}
            Tabs {
                active_tab: *active_tab.read(),
                on_tab_change: move |tab| active_tab.set(tab)
            }
            main { class: "flex-1 mx-auto px-4 sm:px-6 lg:px-8 py-8 w-full font-roboto",
                // IPv4 Tab Content
                div { class: if *active_tab.read() == ActiveTab::Ipv4 { "" } else { "hidden" },
                    div { class: "text-center py-1",
                        h2 { class: "text-4xl font-bold mb-4", "IPv4 Subnet Calculator" }
                        Ipv4Tab {}
                    }
                }

                // IPv6 Tab Content
                div { class: if *active_tab.read() == ActiveTab::Ipv6 { "" } else { "hidden" },
                    div { class: "text-center py-1",
                        h2 { class: "text-4xl font-bold mb-4", "IPv6 Subnet Calculator"}
                        Ipv6Tab {}
                    }
                }

                // Converter Tab Content
                div { class: if *active_tab.read() == ActiveTab::Converter { "" } else { "hidden" },
                    div { class: "text-center py-16",
                        h2 { class: "text-4xl font-bold text-gray-400",
                            "Converter module coming soon..."
                        }
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn PlaceholderTab(props: PlaceholderProps) -> Element {
    rsx! {
        div { class: "text-center py-16",
            h2 { class: "text-4xl font-bold text-gray-500 dark:text-gray-400",
                "{props.name} module coming soon..."
            }
        }
    }
}