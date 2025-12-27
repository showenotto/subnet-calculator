// src/ipv4/results_panel.rs
use dioxus::prelude::*;
use crate::ipv4::types::{CalculationResult, Ipv4InputError};

#[component]
pub fn ResultsPanel(result: Option<Result<CalculationResult, Ipv4InputError>>) -> Element {
    rsx! {
        div { class: "flex-1 bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 overflow-auto",
            h2 { class: "text-3xl font-bold mb-8 text-center", "Network Summary" }

            match result {
                None => rsx! { PlaceholderMessage {} },
                Some(Err(err)) => rsx! { ErrorMessage { err } },
                Some(Ok(calc)) => rsx! {
                    SummaryGrid { summary: calc.summary, new_prefix: calc.new_prefix }

                    if !calc.subnets.is_empty() && calc.subnets.len() > 1 {
                        SubnetTable { subnets: calc.subnets.clone(), base_prefix: calc.base_network.prefix_len() }
                    }
                }
            }
        }
    }
}

#[component]
fn PlaceholderMessage() -> Element {
    rsx! {
        p { class: "text-center text-gray-500 dark:text-gray-400 py-20 text-lg",
            "Enter network details and click Calculate"
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
        div { class: "bg-red-100 dark:bg-red-900/40 border border-red-500 text-red-700 dark:text-red-300 p-6 rounded-lg",
            strong { "Error: " } "{msg}"
        }
    }
}

#[component]
fn SummaryGrid(summary: crate::ipv4::types::SubnetResult, new_prefix: Option<u8>) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mb-8",
            InfoRow { label: "Network", value: format!("{}/{}", summary.network.network(), summary.network.prefix_len()) }
            InfoRow { label: "Netmask", value: summary.netmask.clone() }
            InfoRow { label: "Wildcard", value: summary.wildcard.clone() }
            InfoRow { label: "Broadcast", value: summary.broadcast.clone() }
            InfoRow { label: "First Host", value: summary.first_host.clone().unwrap_or("-".into()) }
            InfoRow { label: "Last Host", value: summary.last_host.clone().unwrap_or("-".into()) }
            InfoRow { label: "Usable Hosts", value: summary.usable_hosts.to_string() }

            if let Some(p) = new_prefix {
                InfoRow { label: "New Subnet Prefix", value: format!("/{}", p) }
            }
        }
    }
}

#[component]
fn SubnetTable(subnets:Vec<crate::ipv4::types::SubnetResult>, base_prefix: u8) -> Element {
    rsx! {
        div { class: "mt-12",
            h3 { class: "text-2xl font-semibold mb-6 text-blue-600 dark:text-blue-400", "Resulting Subnets" }
            div { class: "overflow-x-auto",
                table { class: "w-full text-sm text-left",
                    thead { class: "bg-gray-100 dark:bg-gray-700",
                        tr {
                            th { class: "px-4 py-3", "Subnet" }
                            th { class: "px-4 py-3", "Range" }
                            th { class: "px-4 py-3", "Usable Hosts" }
                            th { class: "px-4 py-3", "Broadcast" }
                        }
                    }
                    tbody {
                        for (i , sub) in subnets.iter().enumerate() {
                            tr { class: "border-t dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/50",
                                td { class: "px-4 py-3 font-mono", "{sub.network}" }
                                td { class: "px-4 py-3 font-mono",
                                    "{sub.first_host.as_deref().unwrap_or(\"-\")} → {sub.last_host.as_deref().unwrap_or(\"-\")}"
                                }
                                td { class: "px-4 py-3 text-center", "{sub.usable_hosts}" }
                                td { class: "px-4 py-3 font-mono", "{sub.broadcast}" }
                            }
                        }
                    }
                }
            }
            if subnets.len() >= 64 {
                p { class: "mt-4 text-center text-gray-500", "Showing first 64 subnets (too many to list all)" }
            }
        }
    }
}

#[component]
fn InfoRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "bg-gray-50 dark:bg-gray-700/50 rounded-lg p-5",
            div { class: "text-sm text-gray-600 dark:text-gray-400", "{label}" }
            div { class: "text-lg font-mono font-semibold mt-1 break-all", "{value}" }
            button {
                class: "mt-3 text-xs px-3 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded hover:bg-blue-200 dark:hover:bg-blue-800",
                onclick: move |_| {
                    #[cfg(web)]
                    {
                        let _ = web_sys::window()
                            .and_then(|w| w.navigator())
                            .and_then(|n| n.clipboard())
                            .and_then(|c| c.write_text(&value));
                    }
                },
                "Copy"
            }
        }
    }
}