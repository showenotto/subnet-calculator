// src/ipv6/results_panel.rs
use dioxus::prelude::*;
use crate::ipv6::types::{CalculationResult, HierarchyLevel, HierarchyNode, Ipv6InputError, SubnetResult};
use crate::ipv6::calculator::{LAST_N,LIMIT};

fn get_tab_class(is_active: bool) -> &'static str {
    if is_active {
        "px-6 py-3 font-medium border-b-2 border-blue-600 text-blue-400"
    } else {
        "px-6 py-3 font-medium border-b-2 border-transparent text-white-600 hover:text-gray-400"
    }
}
#[component]
pub fn ResultsPanel(result: Option<Result<CalculationResult, Ipv6InputError>>, hierarchy_levels: Signal<Vec<HierarchyLevel>>) -> Element {
    let mut active_tab = use_signal(|| 0); 
      let total_usable_subnets = if !hierarchy_levels.read().is_empty() {
        hierarchy_levels.read().iter().fold(1u128, |acc, l| acc * l.num as u128)
    } else {
        0
    };

    rsx! {
        // Changed h-full to a fixed height or min-height to match IPv4 style if needed
        div { class: "h-150 bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto col-span-2",
            h2 { class: "text-xl font-bold mb-6 text-center", "Results" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => {
                    let is_subnetted = calc.new_prefix.is_some();  // ← Fixed: only when new_prefix exists
                    let has_subnets = !calc.subnets.is_empty() || calc.hierarchy.is_some();
                    let summary_tab_class = get_tab_class(*active_tab.read() == 0);
                    let subnets_tab_class = get_tab_class(*active_tab.read() == 1);

                    rsx! {
                        div { class: "flex grid grid-cols-2 border-b border-gray-600 mb-6",
                            button {
                                class: "{summary_tab_class}",
                                onclick: move |_| active_tab.set(0),
                                "Network Details"
                            }
                            if has_subnets {
                                button {
                                    class: "{subnets_tab_class}",
                                    onclick: move |_| active_tab.set(1),
                                    if calc.hierarchy.is_some() { "Hierarchy Tree ({total_usable_subnets.to_string()})" } else { "Subnets ({calc.total_subnets})" }
                                }
                            }
                        }

                        if *active_tab.read() == 0 || !has_subnets {
                            SummaryTable {
                                summary: calc.summary.clone(),
                                new_prefix: calc.new_prefix,
                                base_prefix: calc.base_network.prefix_len(),
                                is_subnetted: is_subnetted
                            } 
                        }
                        if *active_tab.read() == 1 && has_subnets {
                            if let Some(hier) = &calc.hierarchy {
                                HierarchyTree { nodes: hier.tree.clone() }
                            } else {
                                SubnetTable { subnets: calc.subnets.clone(), total_subnets: calc.total_subnets }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryTable(summary: SubnetResult, new_prefix: Option<u8>, base_prefix: u8, is_subnetted: bool) -> Element {
    rsx! {
        div { class: "overflow-y-auto pr-2",
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    SummaryRow { label: "Network", value: summary.network.to_string() }
                    SummaryRow { label: "Compressed", value: summary.compressed }
                    SummaryRow { label: "Expanded", value: summary.expanded }
                    SummaryRow { label: "Address Type", value: summary.addr_type }
                    SummaryRow { label: "First Host", value: summary.first_host }
                    SummaryRow { label: "Last Host", value: summary.last_host }
                    if is_subnetted {
                        tr { class: "border-b border-gray-700",
                            th { class: "px-4 py-3 font-medium font-roboto text-gray-300",
                                span { "New Prefix" }
                            }
                            td { class: "px-4 py-3 font-roboto",
                                span { "/{base_prefix} → /{new_prefix.unwrap()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryRow(label: &'static str, value: String) -> Element {
    rsx! {
        tr { class: "border-b border-gray-700",
            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", span {"{label}"} }
            td { class: "px-4 py-3 break-all", span {"{value}" }}
        }
    }
}

// ... PlaceholderMessage, ErrorMessage, SubnetTable, and HierarchyTree remain largely same 
// but ensure font-sizes use text-sm for consistency with IPv4.

#[component]
fn PlaceholderMessage() -> Element {
    rsx! { p { class: "text-center text-gray-500 py-20 text-base", "Enter details and calculate" } }
}

#[component]
fn ErrorMessage(err: Ipv6InputError) -> Element {
    let msg = match err {
        Ipv6InputError::ParseError(s) => s,
        Ipv6InputError::InvalidPrefix => "Invalid prefix. Child prefix must be bigger than the original prefix".to_string(),
        Ipv6InputError::InsufficientBits => "Insufficient bits for hierarchy".to_string(),
    };
    rsx! { div { class: "bg-red-900/40 p-4 rounded text-sm text-red-300", strong { "Error: " } "{msg}" } }
}

#[component]
fn SubnetTable(subnets: Vec<SubnetResult>, total_subnets: u128) -> Element {
    let is_truncated = subnets.len() == LIMIT && total_subnets > LIMIT as u128;
    let first_k = if is_truncated { LIMIT - LAST_N } else { subnets.len() };

    rsx! {
        div { class: "mt-12h-80",
            div { class: "overflow-x-auto",
                table { class: "w-full text-sm text-left",
                    thead { class: "bg-gray-700",
                        tr {
                            th { class: "px-4 py-2", "ID" }
                            th { class: "px-4 py-2", "Subnet" }
                            th { class: "px-4 py-2", "Range" }
                        }
                    }
                    tbody {
                            {
                            subnets.iter().enumerate().map(|(i, sub)| {
                                    // Compute correct ID
                                    let id = if is_truncated && i >= first_k {
                                        // Last N subnets: calculate real position
                                        let offset = i - first_k;
                                        total_subnets - (LAST_N as u128) + 1 + offset as u128
                                    } else {
                                        // Normal sequential numbering
                                        (i + 1) as u128
                                    };

                                    // Insert ellipsis row at the right position
                                    if is_truncated && i == first_k {
                                        rsx! {
                                            tr {
                                                td { colspan: "3", class: "text-center italic py-1 text-gray-500",
                                                    "..."
                                                }
                                            }
                                            tr {
                                                td { colspan: "3", class: "text-center text-gray-500 p-6 italic",
                                                    p { "Showing only {LIMIT} subnets (first {first_k} + last {LAST_N} subnets, too many to list all)" }
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            tr { class: "border-t border-gray-700 hover:bg-gray-700/50",
                                                td { class: "px-4 py-2 font-roboto", "{id}" }
                                                td { class: "px-4 py-2 font-roboto", "{sub.network}" }
                                                td { class: "px-4 py-2 font-roboto", "{sub.first_host} → {sub.last_host}" }
                                            }
                                        }
                                    }
                                })
                            }
                    }
                }
            }
        }
    }
}

#[component]
fn HierarchyNodeComponent(node: HierarchyNode) -> Element {
    let mut expanded = use_signal(|| false);  // Local state for this node's expansion

    rsx! {
        li { class: "py-1",
            span {
                class: "cursor-pointer",
                onclick: move |_| expanded.toggle(),
                if !node.children.is_empty() {
                    if expanded() { "- " } else { "+ " }
                }
                "{node.label}   →   {node.prefix}"
            }
            /*
            if expanded() && !node.children.is_empty() {
                ul { class: "pl-12 border-l border-gray-600 ml-2",
                    for child in node.children {
                        HierarchyNodeComponent { node: child }
                    }
                }
            }
            */
            if expanded() && !node.children.is_empty() {
                ul { class: "pl-12 border-l border-gray-600 ml-2",
                    {
                        let children = node.children;
                        let len = children.len();

                        if len > LIMIT {
                            // Truncated: first (LIMIT - LAST_N) + ellipsis + last LAST_N
                            let first_k = LIMIT - LAST_N;

                            rsx! {
                                // First K children
                                for child in children[..first_k].iter().cloned() {
                                    HierarchyNodeComponent { node: child }
                                }

                                li { class: "py-1 text-gray-500 text-center text-sm italic ",
                                    "Showing only {LIMIT} nodes (first {LIMIT - LAST_N} + last {LAST_N}). Too many to show all."
                                }

                                // Last N children
                                for child in children[len - LAST_N..].iter().cloned() {
                                    HierarchyNodeComponent { node: child }
                                }
                            }
                        } else {
                            // No truncation: all children
                            rsx! {
                                for child in children {
                                    HierarchyNodeComponent { node: child }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Updated: Now renders the tree using the recursive HierarchyNodeComponent
#[component]
fn HierarchyTree(nodes: Vec<HierarchyNode>) -> Element {
    rsx! {
        ul { class: "list-none pl-0 text-base text-left",  // Left-aligned, no bullets
            for node in nodes {
                HierarchyNodeComponent { node }
            }
        }
    }
}