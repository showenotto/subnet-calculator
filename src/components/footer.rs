// src/components/footer.rs
use dioxus::prelude::*;

pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-100 dark:bg-gray-800 text-center py-3 text-sm text-gray-600 dark:text-gray-400 mt-auto",
            div { class: "max-w-7xl mx-auto px-4",
                a {
                    href: "https://github.com/showenotto/subnet-calculator",
                    target: "_blank",
                    class: "hover:underline",
                    "GitHub"
                }
                span { class: "mx-4", "•" }
                span { "Made using Dioxus" }
            }
        }
    }
}