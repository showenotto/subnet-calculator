// src/components/results_container.rs
use dioxus::prelude::*;

#[component]
pub fn ResultsContainer(
    title: &'static str,
    active_tab: Signal<usize>,
    tabs: Vec<&'static str>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "h-150 bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto col-span-2",
            h2 { class: "text-xl font-bold mb-6 text-center text-gray-100", "{title}" }

            // Tab Navigation
            div { class: "flex border-b border-gray-700 mb-6 justify-center",
                for (i, tab_name) in tabs.iter().enumerate() {
                    button {
                        key: "{tab_name}",
                        class: if *active_tab.read() == i {
                            "px-6 py-3 font-medium border-b-2 border-blue-600 text-blue-400 transition-colors"
                        } else {
                            "px-6 py-3 font-medium border-b-2 border-transparent text-gray-400 hover:text-gray-200 transition-colors"
                        },
                        onclick: move |_| active_tab.set(i),
                        "{tab_name}"
                    }
                }
            }

            // Content Area
            div { class: "mt-4", {children} }
        }
    }
}