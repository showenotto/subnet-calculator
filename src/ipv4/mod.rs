// src/ipv4/mod.rs
pub mod calculator;
pub mod types;
pub mod input_panel;
pub mod results_panel;

use dioxus::prelude::*;
use crate::ipv4::types::{Ipv4InputError, Ipv4Result, calculate_from_cidr};
use crate::ipv4::input_panel::InputPanel;
//use crate::ipv4::results_panel::ResultsPanel;

#[component]
pub fn Ipv4Tab() -> Element {
    let cidr_input = use_signal(|| "192.168.1.0/24".to_string());
    let result = use_signal(|| None::<Result<Ipv4Result, Ipv4InputError>>);

    rsx! {
        //div { class: "max-w-7xl mx-auto px-4 py-8",
        div {class: "grid grid-cols-2",
                InputPanel {
                    cidr_input,
                    result
                }
                div { class: "w-full flex items-center justify-center",
                    p { class: "text-gray-500 dark:text-gray-400 text-xl",
                        "Results Panel coming next..."
                    }
                }
        }
    }
}