use dioxus::prelude::*;
use crate::ipv6::types::{CalculationResult, HierarchyLevel, Ipv6InputError, PREFIX_OPTIONS, SubnetMode};
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
    let is_by_subnets = current_mode == SubnetMode::BySubnets;
    let is_by_prefix = current_mode == SubnetMode::ByPrefix;
    let is_by_hierarchy = current_mode == SubnetMode::ByHierarchy;
    let is_inspect = current_mode == SubnetMode::Inspect;

    // New: Signal for parsed base prefix (from prefix_input, e.g., "/48" -> 48)
    let mut parsed_base_prefix = use_signal(|| 48u8); // Default to 48 if parsing fails
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

    // New: Signals for current (single) hierarchy level inputs - always visible in hierarchy mode
    let mut current_label = use_signal(|| String::new());
    let mut current_bits = use_signal(|| 0u8); // 0 means no selection

    // New: Compute sum of bits in existing levels
    let mut sum_previous_bits: u32 = hierarchy_levels.read().iter().map(|l| l.bits as u32).sum();

    // New: Compute remaining bits (up to /64 for subnetting, leaving 64 for IIDs)
    /*
    let mut max_available_bits = if *parsed_base_prefix.read() <= 64 {
        64u32 - *parsed_base_prefix.read() as u32 - sum_previous_bits
    } else {
        0
    };
    */
    let max_available_bits = if *parsed_base_prefix.read() > 64 {
        0
    } else {
        let total_available = 64u32 - *parsed_base_prefix.read() as u32;
        total_available.saturating_sub(sum_previous_bits)
    };

    // NEW: Compute total usable subnets from hierarchy (product of all level.num)
    let total_usable_subnets = if is_by_hierarchy && !hierarchy_levels.read().is_empty() {
        hierarchy_levels.read().iter().fold(1u128, |acc, l| acc * l.num as u128)
    } else {
        0
    };

    //Disable calculation button
    let is_disabled = match current_mode {
        SubnetMode::BySubnets => count_input.with(|input| input.trim().parse::<u32>().map_or(true, |n| n < 1)),
        SubnetMode::ByPrefix => child_prefix_input.with(|input| input.trim().parse::<u8>().map_or(true, |p| p > 64)),
        SubnetMode::ByHierarchy => {
            let levels = hierarchy_levels.read();
            levels.is_empty() || levels.iter().any(|l| l.num < 1 || l.bits < 1)
        }
        _ => false,
    };

    let button_classes = if is_disabled {
        "w-full bg-blue-600 hover:bg-blue-700  font-bold py-4 rounded-lg text-lg transition opacity-50 cursor-not-allowed col-span-2"
    } else {
        "w-full bg-blue-600 hover:bg-blue-700  font-bold py-4 rounded-lg text-lg transition col-span-2"
    };
    rsx! {
        div { class: "w-full h-150 overflow-y-auto  bg-gray-800 rounded-lg shadow-lg p-6 flex flex-col",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Enter IPv6 Network Information" }

            // IPv6 Address Input
            div { class: "mb-6",
                label { class: "block text-sm text-left font-medium mb-2", "IPv6 Address" }
                input {
                    class: "w-full px-4 py-3 text-base border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                    r#type: "text",
                    placeholder: "e.g. 2001:db8::",
                    value: "{addr_input}",
                    oninput: move |e| addr_input.set(e.value())
                }
            }

            // Prefix length (using the provided code snippet, adapted)
            div { class: "mb-6",
                label { class: "block text-left text-sm font-medium mb-2", "Prefix Length" }
                select {
                    class: "w-full px-4 py-3 pr-10 text-base border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",

                    // Control the select with the current signal value
                    value: "{prefix_input}",

                    onchange: move |evt| prefix_input.set(evt.value()),

                    { PREFIX_OPTIONS.iter().map(|&(prefix, prefix_length)| {
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

            // Mode Selector
            div { class: "mb-6",
                label { class: "block text-sm text-left font-medium mb-2", "Subnet Mode" }
                select {
                    class: "block w-60 px-4 py-3 pr-10 text-base border border-gray-600 rounded-lg  focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",
                    onchange: move |e| {
                        mode.set(match e.value().as_str() {
                            "subnets" => SubnetMode::BySubnets,
                            "prefix" => SubnetMode::ByPrefix,
                            "hierarchy" => SubnetMode::ByHierarchy,
                            _ => SubnetMode::Inspect,
                        });
                    },
                    option { value: "inspect", selected: is_inspect, "Inspect given subnet" }
                    option { value: "subnets", selected: is_by_subnets, "Number of subnets" }
                    option { value: "prefix", selected: is_by_prefix, "By prefix" }
                    option { value: "hierarchy", selected: is_by_hierarchy, "By hierarchy" }
                }
            }

            // Dynamic Fields based on Mode
            if is_by_subnets {
                div { class: "mb-6",
                    label { class: "block text-sm font-medium mb-2", "Number of Subnets Needed" }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full px-4  text-base py-3 border rounded-lg bg-gray-700",
                        placeholder: "e.g. 100",
                        value: "{count_input}",
                        oninput: move |e| count_input.set(e.value())
                    }
                }
            }
            if is_by_prefix {
                div { class: "mb-6",
                    label { class: "block text-sm font-medium mb-2", "Child Prefix Length" }
                    input {
                        r#type: "number",
                        min: "1",
                        max: "64",
                        class: "w-full text-base  px-4 py-3 border rounded-lg bg-gray-700",
                        placeholder: "e.g. 64",
                        value: "{child_prefix_input}",
                        oninput: move |e| child_prefix_input.set(e.value())
                    }
                }
            }
            if is_by_hierarchy {
                         // NEW: Remaining bits indicator
                        div { class: "mb-2 text-xs text-left text-gray-400",
                            "Remaining bits: "
                            strong { "{max_available_bits}" }
                            " (out of {64 - *parsed_base_prefix.read()})"
                        } 
                        div { class: "mb-2",
                            label { class: "block text-xs text-left font-medium mb-2", "Level Label" }
                            input {
                                r#type: "text",
                                class: "flex px-4 py-2 text-sm text-left border rounded-lg bg-gray-700",
                                placeholder: "e.g. Region",
                                value: "{current_label}",
                                oninput: move |e| current_label.set(e.value())
                            }
                        }
                       
                        div { class: "mb-6",
                            label { class: "block text-sm text-left font-medium mb-2", "Number of Subnets" }
                            select {
                                class: "flex px-4 py-2 pr-10 text-sm border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                                style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",
                                value: "{current_bits}",
                                oninput: move |e| current_bits.set(e.value().parse::<u8>().unwrap_or(0)),
                                option { value: "0", "Select..." } // Default/no selection
                                // Dynamically generate options based on remaining bits (up to 16 for perf)
                                for bits in 1..=max_available_bits.min(16) {
                                    option {
                                        value: "{bits}",
                                        "{1u32 << bits} subnets ({bits} bits)"
                                    }
                                }
                            }
                        }

                        // New: Buttons for add/remove
                        div { class: "mb-4",
                            button {
                                //class: "mt-2 text-base bg-green-500 px-4 py-2 rounded {if max_available_bits == 0 || *current_bits.read() == 0 || current_label.read().is_empty() { \"opacity-50 cursor-not-allowed\" } else { \"\" }}",
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
                                //class: "mt-2 ml-2 text-base bg-red-500 px-4 py-2 rounded {if hierarchy_levels.read().is_empty() { \"opacity-50 cursor-not-allowed\" } else { \"\" }}",
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

                        // New: Display message if no more bits available
                        if max_available_bits == 0 {
                            p { class: "mt-2 mb-2 text-center text-sm text-gray-500", "All subnet bits allocated (64 bits left for IIDs)" }
                        }

                        // New: Table displaying added levels
                        if !hierarchy_levels.read().is_empty() {
                            div { class: "mb-4",
                                table { class: "w-full text-sm text-left border-collapse",
                                    thead {
                                        tr {
                                            th { span{"Level" }}
                                            th { span{"Label" }}
                                            th { span{"# Subnets"} }
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
                    }
            // Spacer to push buttons to bottom
            div { class: "flex-1" }

            // Buttons
            div { class: "grid grid-cols-3",
                button {
                    class: "{button_classes}",
                    disabled: is_disabled,
                    onclick: move |_| if !is_disabled {
                        let needed_subnets = if is_by_subnets { count_input.read().parse().ok() } else { None };
                        let child_prefix = if is_by_prefix { child_prefix_input.read().parse().ok() } else { None };
                        let levels = hierarchy_levels.read().clone();
                        let res = calculate(&addr_input(), &prefix_input(), current_mode.clone(), needed_subnets, child_prefix, levels);
                        result.set(Some(res));
                    },
                    "Calculate"
                }
                button {
                    class: "w-full ml-2 bg-red-500 hover:bg-red-500  font-bold py-4 rounded-lg text-lg transition",
                    onclick: move |_| {
                        addr_input.set("2001:db8::".to_string());
                        prefix_input.set("/48".to_string());
                        mode.set(SubnetMode::Inspect);
                        count_input.set("".to_string());
                        child_prefix_input.set("".to_string());
                        hierarchy_levels.set(vec![]);
                        result.set(None);
                    },
                    "Clear"
                }
            }
        }
    }
}