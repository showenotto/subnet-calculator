pub mod calculator;
pub mod types;
pub mod input_panel;
pub mod results_panel;

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use crate::ipv6::types::{CalculationResult, Ipv6InputError, HierarchyLevel, SubnetMode};
use crate::ipv6::input_panel::InputPanel;
use crate::ipv6::results_panel::ResultsPanel;
use crate::ipv6::calculator::calculate;

#[component]
pub fn Ipv6Tab() -> Element {
    // Signals for shared state between input and results
    let addr_input = use_signal(|| "2001:db8::".to_string()); // Default IPv6 address
    let prefix_input = use_signal(|| "/48".to_string()); // Default prefix
    let mode = use_signal(|| SubnetMode::Inspect);
    let count_input = use_signal(|| "".to_string()); // For BySubnets mode
    let child_prefix_input = use_signal(|| "".to_string()); // For ByPrefix mode
    let mut hierarchy_levels = use_signal(|| vec![] as Vec<HierarchyLevel>); // For ByHierarchy
    let mut result = use_signal(|| None::<Result<CalculationResult, Ipv6InputError>>);

    // Live calculation with 300ms debounce on input changes
    use_effect(move || {
        let addr = addr_input.read().clone();
        let prefix = prefix_input.read().clone();
        let count_str = count_input.read().clone();
        let child_prefix_str = child_prefix_input.read().clone();
        let levels = hierarchy_levels.read().clone();

        let needed_subnets = if *mode.read() == SubnetMode::BySubnets {
            count_str.parse().ok()
        } else { None };
        let child_prefix = if *mode.read() == SubnetMode::ByPrefix {
            child_prefix_str.parse().ok()
        } else { None };

        let res = calculate(&addr, &prefix, *mode.read(), needed_subnets, child_prefix, levels);
        result.set(Some(res));
    });

    rsx! {
        div { class: "grid grid-cols-3 gap-8",
            InputPanel {
                addr_input,
                prefix_input,
                mode,
                count_input,
                child_prefix_input,
                hierarchy_levels,
                result
            }
            div { class: "col-span-2",
                ResultsPanel { result: result.read().clone() }
            }
        }
    }
}