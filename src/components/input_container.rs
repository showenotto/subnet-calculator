// src/common/components/input_container.rs
use dioxus::prelude::*;

#[component]
pub fn InputContainer(
    title: String,
    is_disabled: bool,
    on_calculate: EventHandler<()>,
    on_clear: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full h-150 overflow-y-auto bg-gray-800 rounded-lg shadow-lg p-6 flex flex-col",
            h2 { class: "text-2xl font-bold mb-6 text-center", "{title}" }
            
            // This renders the specific inputs (IP, Prefix, Mode etc.)
            {children}

            div { class: "flex-1" } // Spacer

            div { class: "grid grid-cols-3 gap-2",
                button {
                    class: if is_disabled {
                        "w-full bg-blue-600 font-bold py-4 rounded-lg text-lg transition opacity-50 cursor-not-allowed col-span-2"
                    } else {
                        "w-full bg-blue-600 hover:bg-blue-700 font-bold py-4 rounded-lg text-lg transition col-span-2"
                    },
                    disabled: is_disabled,
                    onclick: move |_| on_calculate.call(()),
                    "Calculate"
                }
                button {
                    class: "w-full bg-red-500 hover:bg-red-600 font-bold py-4 rounded-lg text-lg transition",
                    onclick: move |_| on_clear.call(()),
                    "Clear"
                }
            }
        }
    }
}