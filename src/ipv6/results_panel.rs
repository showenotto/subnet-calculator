// src/ipv6/results_panel.rs
use dioxus::prelude::*;
use crate::{
    components::subnet_table::SubnetsTable,
    ipv6::{
        calculator::{LAST_N, LIMIT},
        types::{CalculationResult, HierarchyLevel, HierarchyNode, IpSubnetResult, Ipv6InputError}
    }
};

fn get_tab_class(is_active: bool) -> &'static str {
    if is_active {
        "px-6 py-3 font-medium border-b-2 border-blue-600 text-blue-400"
    } else {
        "px-6 py-3 font-medium border-b-2 border-transparent text-gray-300 hover:text-gray-400"
    }
}

#[component]
pub fn ResultsPanel(
    result: Option<Result<CalculationResult, Ipv6InputError>>, 
    hierarchy_levels: Signal<Vec<HierarchyLevel>>
) -> Element {
    let mut active_tab = use_signal(|| 0); 
    let total_usable_subnets = if !hierarchy_levels.read().is_empty() {
        hierarchy_levels.read().iter().fold(1u128, |acc, l| acc * l.num as u128)
    } else {
        0
    };

    rsx! {
        div { class: "h-150 bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto flex flex-col col-span-2",
            h2 { class: "text-xl font-bold mb-6 text-center text-white", "IPv6 Results" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => {
                    let has_subnets = !calc.subnets.is_empty();
                    let has_hierarchy = calc.hierarchy.is_some();

                    rsx! {
                        // Tab Headers
                        div { 
                            class: "flex grid grid-cols-2 border-b border-gray-600 mb-6",
                            button { 
                                class: get_tab_class(*active_tab.read() == 0),
                                onclick: move |_| active_tab.set(0),
                                "Network Details"
                            }
                            if has_subnets {
                                button { 
                                    class: get_tab_class(*active_tab.read() == 1),
                                    onclick: move |_| active_tab.set(1),
                                    "Subnets ({calc.total_subnets})"
                                }
                            }
                            if has_hierarchy {
                                button { 
                                    class: get_tab_class(*active_tab.read() == 2),
                                    onclick: move |_| active_tab.set(2),
                                    "Hierarchy Tree ({total_usable_subnets.to_string()})"
                                }
                            }
                        }

                        // Tab Content
                        div { class: "flex-1",
                            match *active_tab.read() {
                                0 => rsx! { SummaryView { calc: calc.clone() } },
                                1 if has_subnets => rsx! { 
                                    SubnetsTable { 
                                        calc: calc.clone(), 
                                        limit: LIMIT, 
                                        last_n: LAST_N 
                                    } 
                                },
                                2 if has_hierarchy => rsx! {
                                    HierarchyTree { nodes: calc.hierarchy.as_ref().unwrap().tree.clone() }
                                },
                                _ => rsx! { div {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryView(calc: CalculationResult) -> Element {
    let summary = match &calc.summary {
        IpSubnetResult::V6(v6) => v6,
        _ => return rsx! { "Invalid Result" }
    };

    rsx! {
        div { class: "overflow-hidden rounded-lg mt-2",
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    SummaryRow { label: "Compressed", value: "{summary.compressed}" }
                    SummaryRow { label: "Expanded", value: "{summary.expanded}" }
                    // FIX: Access prefix_len via the network field
                    SummaryRow { label: "Prefix Length", value: "/{summary.network.prefix_len()}" }
                    SummaryRow { label: "Address Type", value: "{summary.addr_type}" }
                    SummaryRow { label: "First Address", value: "{summary.first_host}" }
                    SummaryRow { label: "Last Address", value: "{summary.last_host}" }
                    SummaryRow { label: "Total Subnets", value: "{calc.total_subnets}" }
                }
            }
        }
    }
}

#[component]
fn SummaryRow(label: &'static str, value: String) -> Element {
    rsx! {
        tr { class: "border-b border-gray-700", 
            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", "{label}" }
            td { class: "px-4 py-3 break-all", "{value}" }
        }
    }
}

#[component]
fn HierarchyTree(nodes: Vec<HierarchyNode>) -> Element {
    rsx! {
        div { class: "p-2",
            ul { class: "list-none pl-0 text-left",
                for node in nodes {
                    HierarchyNodeItem { key: "{node.prefix}", node }
                }
            }
        }
    }
}

#[component]
fn HierarchyNodeItem(node: HierarchyNode) -> Element {
    let mut is_expanded = use_signal(|| false);
    let children = &node.children;
    let has_children = !children.is_empty();
    let total_children = children.len();

    rsx! {
        li { class: "mb-1",
            // Node Header
            div { 
                class: "flex items-center gap-2 p-2 hover:bg-gray-700/50 rounded cursor-pointer transition-colors",
                onclick: move |_| is_expanded.toggle(),
                
                if has_children {
                    span { 
                        class: "text-blue-500 w-4 text-center font-bold", 
                        if is_expanded() { "−" } else { "+" } 
                    }
                } else {
                    span { class: "w-4" } 
                }

                span { class: "text-gray-200 text-base", "{node.label}:" }
                span { class: "text-blue-300 text-base", "{node.prefix}" }
            }

            // Recursive Children with Truncation Logic
            if has_children && is_expanded() {
                ul { class: "list-none ml-6 border-l border-gray-600 pl-10 mt-1 transition-all",
                    {
                        if total_children > LIMIT {
                            let first_k = LIMIT - LAST_N;
                            rsx! {
                                // 1. Render first K children
                                {children.iter().take(first_k).map(|child| rsx! {
                                    HierarchyNodeItem { key: "{child.prefix}", node: child.clone() }
                                })}

                                li { class: "py-1 text-gray-500 text-center text-sm italic ",
                                    "Showing only {LIMIT} nodes (first {LIMIT - LAST_N} + last {LAST_N}). Too many to show all."
                                }

                                // 3. Render last N children
                                {children.iter().skip(total_children - LAST_N).map(|child| rsx! {
                                    HierarchyNodeItem { key: "{child.prefix}", node: child.clone() }
                                })}
                            }
                        } else {
                            // Render all children normally if below LIMIT
                            rsx! {
                                {children.iter().map(|child| rsx! {
                                    HierarchyNodeItem { key: "{child.prefix}", node: child.clone() }
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}
//span { class: "text-blue-500 w-4 text-xs", if is_expanded() { "▼" } else { "▶" } }

#[component]
fn PlaceholderMessage() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center py-20 text-gray-500",
            p { "Enter details and calculate" }
        }
    }
}

#[component]
fn ErrorMessage(err: Ipv6InputError) -> Element {
    rsx! {
        div { class: "p-4 bg-red-900/20 border border-red-900/50 rounded text-red-400",
            h3 { class: "font-bold mb-1", "Calculation Error" }
            p { "{err:?}" }
        }
    }
}