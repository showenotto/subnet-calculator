use dioxus::prelude::*;
use crate::{common::types::{CalculationResult, IpSubnetResult}, components::utils::generate_subnets_text};
use crate::components::utils::CopyButton;
use crate::components::utils::ExportButton;

#[component]
pub fn SubnetsTable<T: Clone + PartialEq + 'static>(
    calc: CalculationResult<T>, 
    limit: usize, 
    last_n: usize
) -> Element {
    let subnets = &calc.subnets;
    let total_subnets = calc.total_subnets;
    let count = subnets.len();
    
    // Logic to determine if we are showing a truncated set
    let is_truncated = count >= limit && total_subnets > limit as u128;
    let first_k = if is_truncated { limit - last_n } else { count };

    // CLONE 1: For the Copy Button text logic
    let subnets_for_text = subnets.clone();
    let total_subnets_val = total_subnets;
    let truncated_val = is_truncated;
    let first_k_val = first_k;

    let get_text = use_memo(move || {
        generate_subnets_text(
            &subnets_for_text,
            total_subnets_val,
            truncated_val,
            first_k_val,
        )
    });

    // CLONE 2: For the CSV Export logic
    let subnets_for_csv = subnets.clone();
    let csv_text = use_memo(move || {
        crate::components::utils::generate_subnets_csv(
            &subnets_for_csv,
            total_subnets_val,
            truncated_val,
            first_k_val,
        )
    });

    rsx! {
        div { class: "overflow-hidden",
            div { class: "flex justify-end mb-2",
                ExportButton {
                    default_filename: "subnets.csv",
                    mime: "text/csv",
                    get_content: csv_text
                }
                CopyButton { 
                    key: "{get_text.read()}", 
                    get_text: get_text.clone() 
                }
                
            }
            table { class: "w-full text-sm text-left border-collapse",
                thead { class:"bg-gray-700",
                    tr { 
                        th { class: "px-4 py-2 font-medium w-20", span {"ID" }}
                        th { class: "px-4 py-2 font-medium", span {"Network" }}
                        th { class: "px-4 py-2 font-medium", span {"Host Range" }}
                        th { class: "px-4 py-2 font-medium", span {"Broadcast" }}
                    }
                }
                tbody {
                    {subnets.iter().enumerate().map(|(i, sub)| {
                        // Calculate ID based on truncation
                        let id = if is_truncated && i < first_k {
                            (i + 1) as u128
                        } else if is_truncated {
                            total_subnets - (last_n as u128 - 1) + (i - first_k) as u128
                        } else {
                            (i + 1) as u128
                        };

                        // Render Ellipsis Row
                        if is_truncated && i == first_k {
                            return rsx! {
                                tr { key: "divider-{id}", class: "",
                                    td { colspan: "4", class: "px-4 py-6 text-center text-gray-500 italic",
                                        div { "..." }
                                        div { class: "text-xs mt-2", 
                                            span {"Showing first {first_k} and last {last_n} of {total_subnets} subnets"} 
                                        }
                                    }
                                }
                            };
                        }

                        rsx! {
                            tr { 
                                key: "sub-{id}", 
                                class: "border-b border-gray-700 hover:bg-gray-700/30 transition-colors",
                                td { class: "px-4 py-3", span {"{id}" }}
                                match sub {
                                    IpSubnetResult::V4(v4) => rsx! {
                                        td { class: "px-4 py-3", span {"{v4.network}" }}
                                        td { class: "px-4 py-3", 
                                            span {"{v4.first_host.as_deref().unwrap_or(\"-\")} → {v4.last_host.as_deref().unwrap_or(\"-\")}"} 
                                        }
                                        td { class: "px-4 py-3", span {"{v4.broadcast}" }}
                                    },
                                    IpSubnetResult::V6(v6) => rsx! {
                                        td { class: "px-4 py-3", span {"{v6.compressed}" }}
                                        td { class: "px-4 py-3", span {"{v6.first_host} → {v6.last_host}" }}
                                        td { class: "px-4 py-3", span {"N/A" }}
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}