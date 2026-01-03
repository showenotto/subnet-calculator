// src/ipv4/results_panel.rs
use dioxus::prelude::*;
use crate::ipv4::{calculator::{LAST_N, LIMIT}, types::{CalculationResult, Ipv4InputError, SubnetResult}};


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
fn SummaryTable(summary: SubnetResult, new_prefix: Option<u8>, subnets: Vec<SubnetResult>) -> Element {
    let is_subnetted = new_prefix.is_some() || subnets.len() > 1;

    // Use the first new subnet's details if subnetted; otherwise use the original summary
    let display = if is_subnetted {
        subnets
            .first()
            .cloned()
            .unwrap_or(summary.clone()) // fallback (should never happen)
    } else {
        summary.clone()
    };

    let base_prefix = summary.network.prefix_len();
    rsx! {
        div {
            class: "overflow-y-auto pr-2 ",  // ← This makes it scrollable
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    SummaryRow { label: "Network ID", value: "{display.network.network()}/{display.network.prefix_len()}"}
                    SummaryRow { label: "Netmask", value: display.netmask }
                    SummaryRow { label: "Wildcard Mask", value: display.wildcard}
                    SummaryRow { label: "First Host", value: display.first_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Last Host", value: display.last_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Broadcast", value: display.broadcast }
                    SummaryRow { label: "Usable Hosts", value: display.usable_hosts }
                    if is_subnetted {
                        tr { class: "border-b border-gray-700",
                            th { class: "px-4 py-3 font-medium font-roboto text-gray-300", 
                                span {"New Prefix" }
                            }
                            td { class: "px-4 py-3 font-roboto ", 
                                span {"/{base_prefix} → /{new_prefix.unwrap()}"}
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
fn SubnetTable(subnets:Vec<crate::ipv4::types::SubnetResult>, base_prefix: u8, total_subnets: u64) -> Element {
    let is_truncated = subnets.len() == LIMIT && total_subnets > LIMIT as u64;
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
                                // Compute the ID here (pure Rust, outside rsx!)
                                let id = if is_truncated && i < first_k {
                                    (i + 1) as u64
                                } else if is_truncated {
                                    // Assuming i >= first_k means we're in the last chunk
                                    total_subnets - (LAST_N as u64 - 1) + (i - first_k) as u64
                                } else {
                                    (i + 1) as u64
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
                                            td { class: "px-4 py-3 ", span { "{sub.network}" } }
                                            td { class: "px-4 py-3 ",
                                                span { "{sub.first_host.as_deref().unwrap_or(\"-\")} → {sub.last_host.as_deref().unwrap_or(\"-\")}" }
                                            }
                                            td { class: "px-4 py-3", span { "{sub.broadcast}" } }
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
