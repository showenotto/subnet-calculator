// src/ipv6/results_panel.rs
use dioxus::prelude::*;
use crate::ipv6::types::{CalculationResult, HierarchyNode, Ipv6InputError, SubnetResult};
use crate::ipv6::calculator::{LAST_N, LIMIT};

fn get_tab_class(is_active: bool) -> &'static str {
    if is_active {
        "px-6 py-3 font-medium border-b-2 border-blue-600 text-blue-600 dark:text-blue-400"
    } else {
        "px-6 py-3 font-medium border-b-2 border-transparent text-gray-600 hover:text-gray-900 dark:hover:text-gray-400"
    }
}

#[component]
pub fn ResultsPanel(result: Option<Result<CalculationResult, Ipv6InputError>>) -> Element {
    let mut active_tab = use_signal(|| 0); 

    rsx! {
        // Changed h-full to a fixed height or min-height to match IPv4 style if needed
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto",
            h2 { class: "text-xl font-bold mb-6 text-center", "Results" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => {
                    let has_subnets = !calc.subnets.is_empty() || calc.hierarchy.is_some();
                    let summary_tab_class = get_tab_class(*active_tab.read() == 0);
                    let subnets_tab_class = get_tab_class(*active_tab.read() == 1);

                    rsx! {
                        div { class: "flex border-b border-gray-300 dark:border-gray-600 mb-6",
                            button {
                                class: "{summary_tab_class}",
                                onclick: move |_| active_tab.set(0),
                                "Network Details"
                            }
                            if has_subnets {
                                button {
                                    class: "{subnets_tab_class}",
                                    onclick: move |_| active_tab.set(1),
                                    if calc.hierarchy.is_some() { "Hierarchy Tree" } else { "Subnets ({calc.total_subnets})" }
                                }
                            }
                        }

                        if *active_tab.read() == 0 || !has_subnets {
                            SummaryTable { summary: calc.summary.clone() }
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
fn SummaryTable(summary: SubnetResult) -> Element {
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
                }
            }
        }
    }
}

#[component]
fn SummaryRow(label: &'static str, value: String) -> Element {
    rsx! {
        tr { class: "border-b dark:border-gray-700",
            th { class: "px-4 py-3 font-medium text-gray-700 dark:text-gray-300 w-1/3", "{label}" }
            td { class: "px-4 py-3 font-mono break-all", "{value}" }
        }
    }
}

// ... PlaceholderMessage, ErrorMessage, SubnetTable, and HierarchyTree remain largely same 
// but ensure font-sizes use text-sm for consistency with IPv4.

#[component]
fn PlaceholderMessage() -> Element {
    rsx! { p { class: "text-center text-gray-500 py-20 text-sm", "Enter details and calculate" } }
}

#[component]
fn ErrorMessage(err: Ipv6InputError) -> Element {
    let msg = match err {
        Ipv6InputError::ParseError(s) => s,
        Ipv6InputError::InvalidPrefix => "Invalid prefix".to_string(),
        Ipv6InputError::InsufficientBits => "Insufficient bits for hierarchy".to_string(),
    };
    rsx! { div { class: "bg-red-100 dark:bg-red-900/40 p-4 rounded text-sm text-red-700 dark:text-red-300", strong { "Error: " } "{msg}" } }
}

#[component]
fn SubnetTable(subnets: Vec<SubnetResult>, total_subnets: u128) -> Element {
    let is_truncated = subnets.len() == LIMIT && total_subnets > LIMIT as u128;
    let first_k = if is_truncated { LIMIT - LAST_N } else { subnets.len() };

    rsx! {
        div { class: "overflow-x-auto",
            table { class: "w-full text-sm text-left",
                thead { class: "bg-gray-100 dark:bg-gray-700",
                    tr {
                        th { class: "px-4 py-2", "ID" }
                        th { class: "px-4 py-2", "Subnet" }
                        th { class: "px-4 py-2", "Range" }
                    }
                }
                tbody {
                    for (i, sub) in subnets.iter().enumerate() {
                        if is_truncated && i == first_k {
                            tr { td { colspan: "3", class: "text-center italic py-4 text-gray-500", "..." } }
                        } else {
                            tr { class: "border-t dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/50",
                                td { class: "px-4 py-2 font-roboto", "{(i + 1)}" }
                                td { class: "px-4 py-2 font-roboto", "{sub.network}" }
                                td { class: "px-4 py-2 font-roboto", "{sub.first_host} → {sub.last_host}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HierarchyTree(nodes: Vec<HierarchyNode>) -> Element {
    rsx! {
        ul { class: "list-disc pl-6 text-sm",
            for node in nodes {
                li { class: "py-1", "{node.label}: {node.prefix}" }
                if !node.children.is_empty() {
                    HierarchyTree { nodes: node.children }
                }
            }
        }
    }
}