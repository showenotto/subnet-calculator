// src/common/components/forms.rs
use dioxus::prelude::*;

#[component]
pub fn FormField(label: String, children: Element) -> Element {
    rsx! {
        div { class: "mb-6",
            label { class: "block text-sm text-left font-medium mb-2", "{label}" }
            {children}
        }
    }
}

// Shared style for text inputs
pub const INPUT_CLASS: &str = "w-full px-4 py-3 text-base border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-gray-700";

// Shared style for select menus (including the custom SVG arrow)
pub const SELECT_STYLE: &str = "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;";