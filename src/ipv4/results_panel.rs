// src/ipv4/results_panel.rs
/*
use dioxus::prelude::*;
use crate::ipv4::{calculator::{LAST_N, LIMIT}, types::{CalculationResult, IpSubnetResult, Ipv4InputError, SubnetResult}};


fn get_tab_class(is_active: bool) -> &'static str {
    if is_active {
        "px-6 py-3 font-medium border-b-2 border-blue-600 text-blue-400"
    } else {
        "px-6 py-3 font-medium border-b-2 border-transparent text-white-600 hover:text-gray-400"
    }
}

#[component]
pub fn ResultsPanel(result: Option<Result<CalculationResult, Ipv4InputError>>) -> Element {
    let mut active_tab = use_signal(|| 0); // 0 = Summary, 1 = Subnets

    rsx! {
        div { class: "h-150 bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto",
            h2 { class: "text-xl font-bold mb-6 text-center", "Results" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => {
                    let has_subnets = !calc.subnets.is_empty();

                    // Compute classes outside rsx!
                    let summary_tab_class = get_tab_class(*active_tab.read() == 0);
                    let subnets_tab_class = get_tab_class(*active_tab.read() == 1);

                    rsx! {
                        // Tabs bar
                        div { class: "flex grid grid-cols-2 border-b border-gray-600 mb-6",
                            button {
                                class: "{summary_tab_class} transition-colors",
                                onclick: move |_| active_tab.set(0),
                                "Network Details"
                            }
                            if has_subnets {
                                button {
                                    class: "{subnets_tab_class} transition-colors",
                                    onclick: move |_| active_tab.set(1),
                                    "Subnets ({calc.total_subnets})"
                                }
                            }
                        }

                        // Tab content
                        if *active_tab.read() == 0 || !has_subnets {
                            SummaryTable { 
                                summary: calc.summary.clone(), 
                                new_prefix: calc.new_prefix, 
                                subnets: calc.subnets.clone() 
                            }

                        }
                        if *active_tab.read() == 1 && has_subnets {
                            SubnetTable { subnets: calc.subnets.clone(), base_prefix: calc.base_network.prefix_len(), total_subnets: calc.total_subnets }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PlaceholderMessage() -> Element {
    rsx! {
        p { class: "text-center text-gray-500 text-base py-20",
            "Enter details and calculate"
        }
    }
}


#[component]
fn ErrorMessage(err: Ipv4InputError) -> Element {
    let msg = match err {
        Ipv4InputError::ParseError(s) => s,
        Ipv4InputError::InvalidMask => "Invalid subnet mask".to_string(),
        Ipv4InputError::InvalidPrefix => "Invalid prefix length".to_string(),
    };
    rsx! {
        div { class: "bg-red-900/40 border font-roboto border-red-500 text-red-300 p-6 rounded-lg",
            strong { "Error: " } "{msg}"
        }
    }
}

#[component]
fn SummaryTable(
    summary: IpSubnetResult,
    new_prefix: Option<u8>,
    subnets: Vec<IpSubnetResult>
) -> Element {
    let is_subnetted = new_prefix.is_some() || subnets.len() > 1;

    let summary_v4 = match summary {
        IpSubnetResult::V4(s) => s,
        IpSubnetResult::V6(_) => unreachable!("IPv4 context"),
    };

    let display_v4 = if is_subnetted {
        subnets
            .first()
            .and_then(|s| match s {
                IpSubnetResult::V4(inner) => Some(inner.clone()),
                _ => None,
            })
            .unwrap_or_else(|| summary_v4.clone())
    } else {
        summary_v4.clone()
    };

    let base_prefix = summary_v4.network.prefix_len(); // OK: from original summary

    rsx! {
        div {
            class: "overflow-y-auto pr-2",
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    SummaryRow { label: "Network ID", value: "{display_v4.network.network()}/{display_v4.network.prefix_len()}" }
                    SummaryRow { label: "Netmask", value: "{display_v4.netmask}" }
                    SummaryRow { label: "Wildcard Mask", value: "{display_v4.wildcard}" }
                    SummaryRow { label: "First Host", value: display_v4.first_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Last Host", value: display_v4.last_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Broadcast", value: "{display_v4.broadcast}" }
                    SummaryRow { label: "Usable Hosts", value: "{display_v4.usable_hosts}" }
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

#[component]
fn SubnetTable(subnets:Vec<IpSubnetResult>, base_prefix: u8, total_subnets: u128) -> Element {
    let is_truncated = subnets.len() == LIMIT && total_subnets > LIMIT as u128;
    let first_k = if is_truncated { LIMIT - LAST_N } else { subnets.len() };
    rsx! {
        div { class: "mt-12 h-80",
            div { class: "overflow-x-auto",
                table { class: "w-full text-sm font-roboto text-left",
                    thead { class: "bg-gray-700",
                        tr {
                            th { class: "px-4 py-3 w-24", span {"ID" }}
                            th { class: "px-4 py-3", span {"Subnet" }}
                            th { class: "px-4 py-3", span {"Range" }}
                            th { class: "px-4 py-3", span {"Broadcast" }}
                        }
                    }
                    tbody {
                        {
                            // Precompute the rows as an iterator of RSX elements
                            subnets.iter().enumerate().map(|(i, sub)| {
                                let sub_v4 = match sub {
                                    IpSubnetResult::V4(s) => s,
                                    IpSubnetResult::V6(_) => unreachable!(),
                                };
                                // Compute the ID here (pure Rust, outside rsx!)
                                let id = if is_truncated && i < first_k {
                                    (i + 1) as u128
                                } else if is_truncated {
                                    // Assuming i >= first_k means we're in the last chunk
                                    total_subnets - (LAST_N as u128 - 1) + (i - first_k) as u128
                                } else {
                                    (i + 1) as u128
                                };

                                // Handle the truncation ellipsis row separately
                                if is_truncated && i == first_k {
                                    rsx! {
                                        tr { class: "border-t border-gray-700",
                                            td { colspan: "4",
                                                class: "px-4 py-3 text-center text-gray-500 italic",
                                                "..."
                                                p { class: "mt-4 text-center text-gray-500", "Showing only {LIMIT} subnets (first {first_k} subnets + last {LAST_N} subnets, too many to list all)" }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        tr { class: "border-t border-gray-700 hover:bg-gray-700/50",
                                            td { class: "px-4 py-3 ", span { "{id}" } }
                                            td { class: "px-4 py-3 ", span { "{sub_v4.network}" } }
                                            td { class: "px-4 py-3 ",
                                                span { "{sub_v4.first_host.as_deref().unwrap_or(\"-\")} → {sub_v4.last_host.as_deref().unwrap_or(\"-\")}" }
                                            }
                                            td { class: "px-4 py-3", span { "{sub_v4.broadcast}" } }
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
*/
// src/ipv4/results_panel.rs
use dioxus::prelude::*;
use crate::{
    components::subnet_table::SubnetsTable, 
    ipv4::{
        calculator::{LAST_N, LIMIT}, 
        types::{CalculationResult, IpSubnetResult, Ipv4InputError}
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
pub fn ResultsPanel(result: Option<Result<CalculationResult, Ipv4InputError>>) -> Element {
    let mut active_tab = use_signal(|| 0); 

    rsx! {
        div { class: "h-150 bg-gray-800 rounded-lg shadow-lg p-6 overflow-auto flex flex-col",
            h2 { class: "text-xl font-bold mb-6 text-center text-white", "IPv4 Results" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => {
                    let has_subnets = !calc.subnets.is_empty();

                    rsx! {
                        // Tab Headers
                        div { class: "flex grid grid-cols-2 border-b border-gray-600 mb-6",
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
                        }

                        // Tab Content
                        div { class: "flex-1",
                            match *active_tab.read() {
                                0 => rsx! { SummaryView { calc: calc.clone() } },
                                1 => rsx! { 
                                    SubnetsTable { 
                                        calc: calc.clone(), 
                                        limit: LIMIT, 
                                        last_n: LAST_N 
                                    } 
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
        IpSubnetResult::V4(v4) => v4,
        _ => return rsx! { "Invalid Result" }
    };

    rsx! {
        div { class: "overflow-hidden rounded-lg mt-2",
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    SummaryRow { label: "Network ID", value: "{summary.network}" }
                    SummaryRow { label: "Netmask", value: "{summary.netmask}" }
                    SummaryRow { label: "Wildcard Mask", value: "{summary.wildcard}" }
                    SummaryRow { label: "First Host", value: summary.first_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Last Host", value: summary.last_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Broadcast", value: "{summary.broadcast}" }
                    SummaryRow { label: "Total Subnets", value: "{calc.total_subnets}" }
                    
                    if let Some(p) = calc.new_prefix {
                        tr { class: "",
                            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", "New Prefix" }
                            td { class: "px-4 py-3", "/{calc.base_network.prefix_len()} → /{p}" }
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
        tr {class:"border-b border-gray-700", 
            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", "{label}" }
            td { class: "px-4 py-3 break-all", "{value}" }
        }
    }
}

#[component]
fn PlaceholderMessage() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center py-20 text-gray-500",
            p { "Enter details and calculate" }
        }
    }
}

#[component]
fn ErrorMessage(err: Ipv4InputError) -> Element {
    rsx! {
        div { class: "p-4 bg-red-900/20 border border-red-900/50 rounded text-red-400",
            h3 { class: "font-bold mb-1", "Calculation Error" }
            // FIX: Using {:?} debug formatter if Display is not implemented
            p { "{err:?}" }
        }
    }
}