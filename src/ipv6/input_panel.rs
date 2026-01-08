use dioxus::prelude::*;
use crate::components::input_container::InputContainer;
use crate::components::forms::{FormField, INPUT_CLASS, SELECT_STYLE};
use crate::ipv6::types::{CalculationResult, HierarchyLevel, Ipv6InputError, MAX_USABLE_SUBNETS, PREFIX_OPTIONS, SubnetMode};
use crate::ipv6::calculator::calculate;

#[component]
pub fn InputPanel(
    addr_input: Signal<String>,
    prefix_input: Signal<String>,
    mode: Signal<SubnetMode>,
    count_input: Signal<String>,
    child_prefix_input: Signal<String>,
    hierarchy_levels: Signal<Vec<HierarchyLevel>>,
    result: Signal<Option<Result<CalculationResult, Ipv6InputError>>>,
) -> Element {
    let current_mode = mode.read().clone();
    
    // State for Hierarchy Inputs
    let mut current_label = use_signal(|| String::new());
    let mut current_bits = use_signal(|| 0u8);
    let mut parsed_base_prefix = use_signal(|| 48u8);


    // Validation Logic
    let total_usable_subnets = if current_mode == SubnetMode::ByHierarchy && !hierarchy_levels.read().is_empty() {
        hierarchy_levels.read().iter().fold(1u128, |acc, l| acc * l.num as u128)
    } else {
        0
    };
    let too_many_subnets = total_usable_subnets > MAX_USABLE_SUBNETS;
    // New: Signal for parsed base prefix (from prefix_input, e.g., "/48" -> 48)
    use_effect(move || {
        let input = prefix_input.read();
        let prefix_str = input.trim().strip_prefix('/').unwrap_or(&input);

        if let Ok(prefix) = prefix_str.parse::<u8>() {
            parsed_base_prefix.set(prefix);
        }

        //Remove levels from table that no longer fit the network requirements
        let base = *parsed_base_prefix.read();
        if base > 64 {
            hierarchy_levels.set(vec![]);
            return;
        }

        let total_available = 64u32 - base as u32;
        let mut levels = hierarchy_levels.write();
        let mut used = 0u32;

        levels.retain(|level| {
            if used + level.bits as u32 <= total_available {
                used += level.bits as u32;
                true
            } else {
                false
            }
        });
    });

    let is_by_hierarchy = current_mode == SubnetMode::ByHierarchy;

    let hierarchy_error = too_many_subnets && is_by_hierarchy;
    // Calculate max available bits
    let sum_previous_bits: u32 = hierarchy_levels.read().iter().map(|l| l.bits as u32).sum();
    let max_available_bits = if *parsed_base_prefix.read() > 64 {
        0
    } else {
        let total_available = 64u32 - *parsed_base_prefix.read() as u32;
        total_available.saturating_sub(sum_previous_bits)
    };

    let is_disabled = match current_mode {
        SubnetMode::BySubnets => count_input.with(|input| input.trim().parse::<u32>().map_or(true, |n| n < 1)),
        SubnetMode::ByPrefix => child_prefix_input.with(|input| input.trim().parse::<u8>().map_or(true, |p| p > 128)),
        SubnetMode::ByHierarchy => //hierarchy_levels.read().is_empty() || too_many_subnets,
        hierarchy_levels.read().is_empty() || hierarchy_levels.read().iter().any(|l| l.num < 1 || l.bits < 1)|| too_many_subnets,
        _ => false,
    };

    rsx! {
        InputContainer {
            title: "Enter IPv6 Network Information",
            is_disabled: is_disabled,
            on_calculate: move |_| {
                if is_disabled { return; }
                let needed_subnets = if current_mode == SubnetMode::BySubnets { count_input.read().parse().ok() } else { None };
                let child_prefix = if current_mode == SubnetMode::ByPrefix { child_prefix_input.read().parse().ok() } else { None };
                let levels = hierarchy_levels.read().clone();
                let res = calculate(&addr_input(), &prefix_input(), current_mode.clone(), needed_subnets, child_prefix, levels);
                result.set(Some(res));
            },
            on_clear: move |_| {
                addr_input.set("2001:db8::".to_string());
                prefix_input.set("/48".to_string());
                mode.set(SubnetMode::Inspect);
                count_input.set("".to_string());
                child_prefix_input.set("".to_string());
                hierarchy_levels.set(vec![]);
                result.set(None);
            },

            FormField { label: "IPv6 Address",
                input {
                    class: INPUT_CLASS,
                    value: "{addr_input}",
                    oninput: move |e| addr_input.set(e.value())
                }
            }

            FormField { label: "Prefix Length",
                select {
                    class: "{INPUT_CLASS} pr-10 appearance-none",
                    style: SELECT_STYLE,
                    value: "{prefix_input}",
                    onchange: move |evt| prefix_input.set(evt.value()),
                    { PREFIX_OPTIONS.iter().map(|&(_prefix, prefix_length)| {
                        let is_selected = *prefix_input.read() == *prefix_length;
                        rsx! {
                            option {
                                class: "text-base",
                                value: "{prefix_length}",
                                selected: is_selected,
                                "{prefix_length} "
                            }
                        }
                    })}
                }
            }
            FormField { label: "Subnet Mode",
                select {
                    class: "block w-60 px-4 py-3 pr-10 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none bg-gray-700",
                    style: SELECT_STYLE,
                    onchange: move |e| {
                        mode.set(match e.value().as_str() {
                            "subnets" => SubnetMode::BySubnets,
                            "prefix" => SubnetMode::ByPrefix,
                            "hierarchy" => SubnetMode::ByHierarchy,
                            _ => SubnetMode::Inspect,
                        });
                    },
                    option { value: "inspect", selected: current_mode == SubnetMode::Inspect, "Inspect given subnet" }
                    option { value: "subnets", selected: current_mode == SubnetMode::BySubnets, "Number of subnets" }
                    option { value: "prefix", selected: current_mode == SubnetMode::ByPrefix, "By prefix" }
                    option { value: "hierarchy", selected: current_mode == SubnetMode::ByHierarchy, "By hierarchy" }
                }
            }

            // Merged Dynamic Fields
            {
                let m: SubnetMode = current_mode.clone();
                match m {
                    SubnetMode::BySubnets => rsx! {
                        FormField { label: "Number of Subnets Needed",
                            input { r#type: "number", class: INPUT_CLASS, value: "{count_input}", oninput: move |e| count_input.set(e.value()) }
                        }
                    },
                    SubnetMode::ByPrefix => rsx! {
                        FormField { label: "Child Prefix Length",
                            input { r#type: "number", max: "128", class: INPUT_CLASS, value: "{child_prefix_input}", oninput: move |e| child_prefix_input.set(e.value()) }
                        }
                    },
                    SubnetMode::ByHierarchy => rsx! {
                        div { class: "mb-2 text-xs text-gray-400", "Remaining bits: "
                            strong { "{max_available_bits}" }
                            " (out of {64 - *parsed_base_prefix.read()})" }
                        FormField { label: "Level Label",
                            input { 
                                class: "flex px-4 py-2 text-sm text-left border rounded-lg bg-gray-700", 
                                value: "{current_label}", 
                                placeholder: "e.g. Country",
                                oninput: move |e: Event<FormData>| current_label.set(e.value()) 
                            }
                        }
                        // Type-anchored Dropdown
                        {
                            let s: Signal<u8> = current_bits;
                            let max_bits: u8 = max_available_bits as u8;
                            rsx! {
                                SubnetBitsDropdown { 
                                    current_bits: s, 
                                    max_available_bits: max_bits 
                                }
                            }
                        }
                        
                         div { class: "mb-4",
                            button {
                                class: {
                                    let mut classes = vec![
                                        "mt-2",
                                        "text-base",
                                        "bg-green-500",
                                        "px-2",
                                        "py-1",
                                        "rounded",
                                    ];

                                    let is_disabled = max_available_bits == 0 
                                        || *current_bits.read() == 0 
                                        || current_label.read().is_empty();

                                    if is_disabled {
                                        classes.push("opacity-50");
                                        classes.push("cursor-not-allowed");
                                    }

                                    classes.join(" ")
                                },
                                disabled: max_available_bits == 0 || *current_bits.read() == 0 || current_label.read().is_empty(),
                                onclick: move |_| {
                                    if *current_bits.read() > 0 && !current_label.read().is_empty() {
                                        hierarchy_levels.write().push(HierarchyLevel {
                                            name: current_label.read().clone(),
                                            num: 1u32 << *current_bits.read(),
                                            bits: *current_bits.read(),
                                        });
                                        current_label.set("".to_string());
                                        current_bits.set(0);
                                    }
                                },
                                "Add Level"
                            }
                            
                            button {
                                class: {
                                    let mut classes = vec![
                                        "mt-2",
                                        "ml-2",
                                        "text-base",
                                        "bg-red-500",
                                        "px-2",
                                        "py-1",
                                        "rounded",
                                    ];

                                    if hierarchy_levels.read().is_empty() {
                                        classes.push("opacity-50");
                                        classes.push("cursor-not-allowed");
                                    }

                                    classes.join(" ")
                                },
                                disabled: hierarchy_levels.read().is_empty(),
                                onclick: move |_| { hierarchy_levels.write().pop(); },
                                "Remove Last Level"
                            }
                        }
                    },
                    _ => rsx! { div {} }
                }
            }

            // New: Table displaying added levels
            if current_mode == SubnetMode::ByHierarchy && !hierarchy_levels.read().is_empty() {
                div { class: "mb-4",
                    table { class: "w-full text-sm text-left border-collapse",
                        thead {
                            tr {
                                th { span{"Level" }}
                                th { span{"Label" }}
                                th { span{"# of Subnets"} }
                                th { span{"Bits" }}
                            }
                        }
                        tbody {
                            for (i, level) in hierarchy_levels.read().iter().enumerate() {
                                tr {
                                    td { span{"{i + 1}" }}
                                    td { span{"{level.name}" }}
                                    td { span{"{level.num}" }}
                                    td { span{"{level.bits}" }}
                                }
                            }
                        }
                    }
                }
            }
            // NEW: Display total usable subnets from hierarchy
            if total_usable_subnets > 0 {
                p { class: "mt-4 text-center text-sm text-gray-500",
                    "Total usable subnets: {total_usable_subnets}"
                }
            }
            if hierarchy_error {
                div { class: "mb-4 p-4 bg-red-900/50 border border-red-700 rounded-lg text-red-200 text-sm",
                    strong { "Error: " }
                    "The current hierarchy would generate {total_usable_subnets} subnets, which is too many to calculate efficiently. "
                    "Please reduce the number of subnets per level (maximum allowed: {MAX_USABLE_SUBNETS})."
                }
            }
        }
    }
}

#[component]
pub fn SubnetBitsDropdown(
    current_bits: Signal<u8>,
    max_available_bits: u8,
) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "relative mb-6",

            label { class: "block text-sm text-left font-medium mb-2", "Number of Subnets" }

            // Main toggle button
            button {
                class: "w-60 px-4 py-2 text-left text-sm text-white bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent flex justify-between items-center",
                onclick: move |evt| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    is_open.toggle();
                },

                {
                    let bits = *current_bits.read();
                    if bits == 0 {
                        rsx! { span { class: "text-gray-400", "Select..." } }
                    } else {
                        let num = 1u32 << bits;
                        let s = if num > 1 { "s" } else { "" };
                        let bit_s = if bits > 1 { "s" } else { "" };
                        rsx! { "{num} subnet{s} ({bits} bit{bit_s})" }
                    }
                }

                svg {
                    class: {
                        let base = "w-5 h-5 text-gray-400 ml-2 transition-transform duration-200";
                        let rotate = if is_open() { " rotate-180" } else { "" };
                        format!("{base}{rotate}")
                    },
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 20 20",
                    fill: "currentColor",
                    path { d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" }
                }
            }

            // Dropdown menu
            if is_open() {
                div {
                    class: "absolute z-50 mt-2 w-60 bg-gray-700 border border-gray-600 rounded-lg shadow-xl max-h-60 overflow-y-auto",
                    onclick: move |evt| evt.stop_propagation(), // Prevent closing when clicking inside

                    {(1..=max_available_bits.min(16u8)).map(|bits| {
                        let num = 1u32 << bits;
                        let s = if num > 1 { "s" } else { "" };
                        let bit_s = if bits > 1 { "s" } else { "" };
                        let is_selected = *current_bits.read() == bits;

                        let base_class = "w-full px-4 py-3 text-left text-xs text-white hover:bg-gray-600";
                        let selected_class = if is_selected { " bg-gray-600" } else { "" };

                        rsx! {
                            button {
                                key: "{bits}",
                                class: "{base_class}{selected_class}",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    current_bits.set(bits);
                                    is_open.set(false);
                                },
                                "{num} subnet{s} ({bits} bit{bit_s})"
                            }
                        }
                    })}
                }
            }
        }
    }
}