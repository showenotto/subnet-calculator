use std::thread::Scope;

// src/ipv4/input_panel.rs
use dioxus::prelude::*;

use crate::ipv4::types::{calculate_from_cidr, Ipv4InputError, Ipv4Result};

#[derive(Clone, PartialEq)]
pub enum SubnetMode {
    Basic,
    ByHosts,
    BySubnets,
    CustomSubnets,
}

/*
pub fn InputPanel(
    mut cidr_input: Signal<String>,
    mut result: Signal<Option<Result<Ipv4Result, Ipv4InputError>>>,
) -> Element {
    rsx! {
        div { class: "w-full lg:w-96 bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 h-fit",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Input Panel" }

            // Main CIDR Input
            div { class: "mb-6",
                label { class: "block text-sm font-medium mb-2", "IP Address / CIDR" }
                input {
                    class: "w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700",
                    r#type: "text",
                    placeholder: "e.g. 192.168.1.0/24",
                    value: "{cidr_input.read()}",
                    oninput: move |evt| {
                        cidr_input.set(evt.value().clone());
                        // Trigger calculation (will be debounced in parent)
                    }
                }
            }

            // Subnet Mode Selector
            div { class: "mb-6",
                label { class: "block text-sm font-medium mb-2", "Subnet Mode" }
                select {
                    class: "w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-700",
                    // We'll expand this later with oninput
                    option { value: "hosts", "Number of hosts" }
                    option { value: "subnets", "Number of subnets" }
                    option { value: "inspect", "Inspect given subnet" }
                }
            }

            // Placeholder for dynamic fields (hosts/subnets count)
            div { class: "mb-8", id: "dynamic-input" }

            // Calculate Button
            button {
                class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-lg transition text-lg",
                onclick: move |_| {
                    // Force immediate calculation
                    let input = cidr_input.read().clone();
                    let res = calculate_from_cidr(&input);
                    result.set(Some(res));
                },
                "Calculate"
            }
        }
    }
}
*/
#[component]
pub fn InputPanel(
    cidr_input: Signal<String>,
    result: Signal<Option<Result<Ipv4Result, Ipv4InputError>>>,
) -> Element {
    rsx! {
        div { class: "w-full bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 ",
            h2 { class: "text-2xl font-bold mb-2 text-center", "Enter Network Information" }

            // Main CIDR Input
            div { class: "mb-6",
                label { class: "block text-sm font-medium mb-2", "IP Address" }
                input {
                    class: "w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700",
                    r#type: "text",
                    placeholder: "e.g. 192.168.1.0/24",
                    value: "{cidr_input}",
                    oninput: move |evt| cidr_input.set(evt.value())
                }
            }

            // Subnet Mode Selector (placeholder for now)
            div { class: "mb-6",
                label { class: "block text-left text-sm font-medium mb-2", "Subnet Mode" }
                select {
                    //class: "block mx-0 w-64 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg dark:bg-gray-800",
                    class: "block w-60 mx-0 px-4 py-3 pr-10 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-100 dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none",
                    style: "background-image: url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e\"); background-position: right 0.75rem center; background-repeat: no-repeat; background-size: 1.5em;",
                    option { value: "hosts", "Number of hosts" }
                    option { value: "subnets", "Number of subnets" }
                    option { value: "inspect", "Inspect given subnet" }
                }
            }

            // Placeholder for dynamic fields (we'll add later)
            div { class: "mb-8", id: "dynamic-input" }

            // Calculate Button
            button {
                class: "w-90 bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-lg transition text-lg",
                onclick: move |_| {
                    let input = cidr_input();
                    let res = calculate_from_cidr(&input);
                    result.set(Some(res));
                },
                "Calculate"
            }
        }
    }
}