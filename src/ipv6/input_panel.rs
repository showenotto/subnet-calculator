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

    let is_disabled = match current_mode {
    SubnetMode::BySubnets => count_input.with(|input| {
        input.trim().parse::<u32>().map_or(true, |n| n < 1)
    }),
    SubnetMode::ByPrefix => child_prefix_input.with(|input| {
        input.trim().parse::<u8>().map_or(true, |p| p > 64)
    }),
    SubnetMode::ByHierarchy => {
        let levels = hierarchy_levels.read();
        levels.is_empty() || levels.iter().any(|l| l.num < 1 || l.bits < 1)
    }
    _ => false,
    };

    let button_classes = if is_disabled {
        "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-lg text-lg transition opacity-50 cursor-not-allowed col-span-2"
    } else {
        "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-lg text-lg transition col-span-2"
    };
    rsx! {
        div { class: "w-full h-full bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 flex flex-col",
            h2 { class: "text-2xl text-white font-bold mb-6 text-center", "Enter IPv6 Network Information" }

            // IPv6 Address Input
            div { class: "mb-6",
                label { class: "block text-sm text-white text-left font-medium mb-2", "IPv6 Address" }
                input {
                    class: "w-full px-4 py-3 text-base text-white text-base border rounded-lg dark:bg-gray-700",
                    r#type: "text",
                    placeholder: "e.g. 2001:db8::",
                    value: "{addr_input}",
                    oninput: move |e| addr_input.set(e.value())
                }
            }

            // Prefix length (using the provided code snippet, adapted)
            div { class: "mb-6",
                label { class: "block text-left text-sm text-white font-medium mb-2", "Prefix Length" }
                select {
                    class: "w-full px-4 py-3 pr-10 text-base text-white border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",

                    // Control the select with the current signal value
                    value: "{prefix_input}",

                    onchange: move |evt| prefix_input.set(evt.value()),

                    { PREFIX_OPTIONS.iter().map(|&(prefix, prefix_length)| {
                        let is_selected = *prefix_input.read() == *prefix_length;
                        rsx! {
                            option {
                                class: "text-white, text-base",
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
                label { class: "block text-sm text-left text-white font-medium mb-2", "Subnet Mode" }
                select {
                    class: "w-full px-4 py-3 pr-10 text-base text-white border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
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
                        class: "w-full px-4 text-white text-base py-3 border rounded-lg dark:bg-gray-700",
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
                        class: "w-full text-base text-white px-4 py-3 border rounded-lg dark:bg-gray-700",
                        placeholder: "e.g. 64",
                        value: "{child_prefix_input}",
                        oninput: move |e| child_prefix_input.set(e.value())
                    }
                }
            }
            if is_by_hierarchy {
                div { class: "mb-6",
                    //h3 { class: "text-base font-semibold mb-4", "Hierarchy Levels" }
                    button {
                        class: "mb-4 bg-green-500 text-white text-base px-4 py-2 rounded",
                        onclick: move |_| hierarchy_levels.write().push(HierarchyLevel { name: "".to_string(), num: 1, bits: 4 }),
                        "Add Level"
                    }
                    for (i, level) in hierarchy_levels.read().iter().enumerate() {
                        div { class: "mb-4 border p-4 rounded",
                            input {
                                class: "w-full text-sm text-white mb-2 px-4 py-2 border",
                                placeholder: "Level Name (e.g. Region)",
                                value: "{level.name}",
                                oninput: move |e| hierarchy_levels.write()[i].name = e.value()
                            }
                            input {
                                r#type: "number",
                                min: "1",
                                class: "w-full text-sm text-white mb-2 px-4 py-2 border",
                                placeholder: "Number of Subdivisions",
                                value: "{level.num}",
                                oninput: move |e| hierarchy_levels.write()[i].num = e.value().parse().unwrap_or(1)
                            }
                        }
                    }
                    div {} //Line break beween buttons
                    button {
                        class: "mt-2 text-base bg-red-500 text-white px-4 py-2 rounded",
                        onclick: move |_| { hierarchy_levels.write().pop(); },
                        "Remove Last Level"
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
                    class: "w-full ml-2 bg-red-500 hover:bg-red-500 text-white font-bold py-4 rounded-lg text-lg transition",
                    onclick: move |_| {
                        addr_input.set("".to_string());
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