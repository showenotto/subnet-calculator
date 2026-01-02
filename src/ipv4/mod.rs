// src/ipv4/mod.rs
pub mod calculator;
pub mod types;
pub mod input_panel;
pub mod results_panel;

use dioxus::prelude::*;
use crate::ipv4::types::{CalculationResult, Ipv4InputError, SubnetMode};
use crate::ipv4::input_panel::{InputPanel};
use crate::ipv4::results_panel::ResultsPanel;

#[component]
pub fn Ipv4Tab() -> Element {
    let ip_input = use_signal(|| "192.168.1.0".to_string());
    let cidr_input = use_signal(|| "/24".to_string());
    let mode = use_signal(|| SubnetMode::Inspect);
    let count_input = use_signal(|| "".to_string());
    let result = use_signal(|| None::<Result<CalculationResult, Ipv4InputError>>);


    rsx! {
        //div { class: "max-w-7xl mx-auto px-4 py-8",
        div {class: "grid grid-cols-3 gap-4",
                InputPanel {
                    ip_input,
                    cidr_input,
                    mode,
                    count_input,
                    result
                }
            
            div {class: "col-span-2",
                ResultsPanel { result: result.read().clone() }
            }
        }
    }
}