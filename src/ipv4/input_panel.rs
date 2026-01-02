// src/ipv4/input_panel.rs
use dioxus::prelude::*;

use crate::ipv4::{calculator::calculate, types::{CIDR_OPTIONS, CalculationResult, Ipv4InputError, SubnetMode}};



#[component]
pub fn InputPanel(
    ip_input: Signal<String>,
    cidr_input: Signal<String>,
    mode: Signal<SubnetMode>,
    count_input: Signal<String>,
    result: Signal<Option<Result<CalculationResult, Ipv4InputError>>>,
) -> Element {
    let current_mode = mode.read().clone();
    let count_is_valid = count_input.read().trim().parse::<u32>().is_ok_and(|n| n >= 1);
    let show_error = matches!(current_mode, SubnetMode::ByHosts | SubnetMode::BySubnets) && !count_is_valid;
    
    let is_disabled = if *mode.read() == SubnetMode::Inspect {
        false
    } else {
        count_input.read().trim().is_empty() || count_input.read().parse::<u32>().is_err()
    };

    let button_classes = if is_disabled {
        "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold  py-4 rounded-lg text-lg transition opacity-50 cursor-not-allowed col-span-2"
    } else {
        "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold  py-4 rounded-lg text-lg transition col-span-2"
    };

    rsx! {
        div { class: "w-full h-150  bg-gray-800 rounded-lg shadow-lg p-6 flex flex-col",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Enter IPv4 Network Information" }

            // IP Input
            div { class: "mb-6",
                label { class: "block text-left text-sm font-medium  mb-2", "IP Address" }
                input {
                    class: "w-full px-4 py-3 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                    r#type: "text",
                    placeholder: "e.g. 192.168.1.0",
                    value: "{ip_input}",
                    oninput: move |evt| ip_input.set(evt.value())
                }
            }

            // CIDR or Subnet Mask Input
            div { class: "mb-6",
                label { class: "block text-left text-sm font-medium  mb-2", "Subnet Mask" }
                select {
                    class: "w-full px-4 py-3 pr-10  border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",

                    // Control the select with the current signal value
                    value: "{cidr_input}",

                    onchange: move |evt| cidr_input.set(evt.value()),

                    { CIDR_OPTIONS.iter().map(|&(_prefix, cidr, mask)| {
                        let is_selected = *cidr_input.read() == *cidr;
                        rsx! {
                            option {
                                value: "{cidr}",
                                selected: is_selected,
                                "{cidr} — {mask}"
                            }
                        }
                    })}
                }
            }
            // Subnet Mode Selector (placeholder for now)
            div { class: "mb-6",
                label { class: "block text-left text-sm font-medium  mb-2", "Subnet Mode" }
                select {
                    class: "block w-60 mx-0 px-4 py-3 pr-10 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",
                    onchange: move |e| {
                        let val = e.value();
                        mode.set(match val.as_str() {
                            "hosts" => SubnetMode::ByHosts,
                            "subnets" => SubnetMode::BySubnets,
                            _ => SubnetMode::Inspect,
                        });
                    },

                    option { value: "inspect", selected: *mode.read() == SubnetMode::Inspect, "Inspect given subnet" }
                    option { value: "hosts", selected: *mode.read() == SubnetMode::ByHosts, "Number of hosts" }
                    option { value: "subnets", selected: *mode.read() == SubnetMode::BySubnets, "Number of subnets" }
                }
            }

            // Number of Hosts or Subnets field
            if *mode.read() != SubnetMode::Inspect{
                div { class: "mb-4",
                    label { class: "block text-sm font-medium  mb-2",
                        if *mode.read() == SubnetMode::ByHosts { "Number of Hosts Needed" } else { "Number of Subnets Needed" }
                    }
                    input {
                        r#type: "number",
                        min: "1",
                        class: "w-full px-4 py-3 border  rounded-lg bg-gray-700 hide-number-spinner",
                        placeholder: "e.g. 32",
                        value: "{count_input}",
                        oninput: move |e| count_input.set(e.value())
                    }
                }
            }

            //Padding to push calculate button down
            div { class: "flex-1" }
            div {
                class: "grid grid-cols-3",
                button {
                    class: "{button_classes}",
                    disabled: is_disabled,
                    onclick: move |_| {
                        if is_disabled { return; }
                        let hosts = if *mode.read() == SubnetMode::ByHosts {
                            count_input.read().parse().ok()
                        } else { None };
                        let subnets = if *mode.read() == SubnetMode::BySubnets {
                            count_input.read().parse().ok()
                        } else { None };

                        let res = calculate(&ip_input(), &cidr_input(), hosts, subnets);
                        result.set(Some(res));
                    },
                    "Calculate"
                }
                button {
                    class: "w-full ml-2 bg-red-500 hover:bg-red-500 text-white font-bold  py-4 rounded-lg text-lg transition",
                    onclick: move |_| {
                        ip_input.set("192.168.1.0".to_string());
                        mode.set(SubnetMode::Inspect);
                        count_input.set("".to_string());
                        result.set(None);
                    },
                    "Clear"
                }
            }
            
        }
    }
}