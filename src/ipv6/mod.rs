pub mod calculator;
pub mod types;
pub mod input_panel;
pub mod results_panel;

use dioxus::prelude::*;
use crate::ipv6::types::{CalculationResult, Ipv6InputError, HierarchyLevel, SubnetMode};
use crate::ipv6::input_panel::InputPanel;
use crate::ipv6::results_panel::ResultsPanel;

#[component]
pub fn Ipv6Tab() -> Element {
    // Signals for shared state between input and results
    let addr_input = use_signal(|| "2001:db8::".to_string()); // Default IPv6 address
    let prefix_input = use_signal(|| "/48".to_string()); // Default prefix
    let mode = use_signal(|| SubnetMode::Inspect);
    let count_input = use_signal(|| "".to_string()); // For BySubnets mode
    let child_prefix_input = use_signal(|| "".to_string()); // For ByPrefix mode
    let hierarchy_levels = use_signal(|| vec![] as Vec<HierarchyLevel>); // For ByHierarchy
    let result = use_signal(|| None::<Result<CalculationResult, Ipv6InputError>>);

    rsx! {
        div { class: "grid grid-cols-3 gap-4",
            InputPanel {
                addr_input,
                prefix_input,
                mode,
                count_input,
                child_prefix_input,
                hierarchy_levels,
                result
            }
            ResultsPanel { result: result.read().clone(), hierarchy_levels}
        }
    }
}