// src/components/header.rs
use dioxus::prelude::*;

pub fn Header() -> Element {

    rsx! {
        header { class: "bg-blue-600 dark:bg-blue-800 text-white shadow-lg",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-2 flex justify-between items-center",
                div { class: "flex items-center space-x-4",
                    // Placeholder logo
                    h1 { class: "text-2xl font-bold", "Subnet Calculator" }
                }
            }
        }
    }
}