use dioxus::prelude::*;
use crate::components::utils::{CopyButton, ExportButton, SummaryRow};
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
                                0 => rsx! { SummaryView { key: "{calc.base_network.to_string()}-{calc.total_subnets}", calc: calc.clone() } },
                                1 => rsx! { 
                                    SubnetsTable { 
                                        key: "{calc.total_subnets}-{calc.subnets.len()}",
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
 let summary_ref = match &calc.summary {
        IpSubnetResult::V4(v4) => v4,
        _ => return rsx! { "Invalid Result" }
    };

    // 1. Updated labels and values for CSV Export
    let labels = vec![
        "Network", "Netmask", "Wildcard", "First Host", 
        "Last Host", "Broadcast", "Usable Hosts", "Total Subnets"
    ];
    let values = vec![
        summary_ref.network.to_string(),
        summary_ref.netmask.to_string(),
        summary_ref.wildcard.to_string(),
        summary_ref.first_host.clone().unwrap_or("-".into()),
        summary_ref.last_host.clone().unwrap_or("-".into()),
        summary_ref.broadcast.to_string(),
        calc.total_subnets.to_string(),
        summary_ref.usable_hosts.to_string(), 
    ];
    
    let csv_content = use_memo(move || {
        crate::components::utils::generate_summary_csv(&labels, &values)
    });

    // 2. Updated formatted text for Copy Button
    let summary_for_memo = summary_ref.clone();
    let total_subnets = calc.total_subnets;
    let get_text = use_memo(move || {
        let mut s = String::new();
        s.push_str(&format!("Network: {}\n", summary_for_memo.network));
        s.push_str(&format!("Netmask: {}\n", summary_for_memo.netmask));
        s.push_str(&format!("Wildcard: {}\n", summary_for_memo.wildcard));
        s.push_str(&format!("First Host: {}\n", summary_for_memo.first_host.clone().unwrap_or("-".into())));
        s.push_str(&format!("Last Host: {}\n", summary_for_memo.last_host.clone().unwrap_or("-".into())));
        s.push_str(&format!("Broadcast: {}\n", summary_for_memo.broadcast));
        // FIX: Pull from summary_for_memo
        s.push_str(&format!("Total Subnets: {}\n", total_subnets));
        s.push_str(&format!("Usable Hosts: {}\n", summary_for_memo.usable_hosts));
        s
    });

    rsx! {
        div { class: "overflow-hidden",
            div { class: "flex justify-end mb-2",
                ExportButton { 
                    default_filename: "ipv4_summary.csv",
                    mime: "text/csv",
                    get_content: csv_content
                }
                CopyButton { 
                    key: "{get_text.read()}", 
                    get_text: get_text.clone()
                }
            }
            table { class: "w-full text-sm text-left border-collapse",
                tbody {
                    // 2. Use 'summary_ref' here. Since it's a reference, 
                    // it wasn't moved into the closure.
                    SummaryRow { label: "Network ID", value: "{summary_ref.network}" }
                    SummaryRow { label: "Netmask", value: "{summary_ref.netmask}" }
                    SummaryRow { label: "Wildcard Mask", value: "{summary_ref.wildcard}" }
                    SummaryRow { label: "First Host", value: summary_ref.first_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Last Host", value: summary_ref.last_host.clone().unwrap_or("-".into()) }
                    SummaryRow { label: "Broadcast", value: "{summary_ref.broadcast}" }
                    SummaryRow { label: "Total Subnets", value: "{calc.total_subnets}" }
                    SummaryRow { label: "Usable Hosts", value: "{summary_ref.usable_hosts}" }
                    
                    if let Some(p) = calc.new_prefix {
                        tr { class: "border-b border-gray-700",
                            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", span {"New CIDR" }}
                            td { class: "px-4 py-3", span {"/{calc.base_network.prefix_len()} → /{p}" }}
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