use dioxus::prelude::*;
use crate::components::input_container::{InputContainer};
use crate::components::forms::{FormField, INPUT_CLASS, SELECT_STYLE};
use crate::ipv4::{calculator::calculate, types::{CIDR_OPTIONS, CalculationResult, Ipv4InputError, SubnetMode}};


#[component]
pub fn InputPanel(
    ip_input: Signal<String>,
    cidr_input: Signal<String>,
    mode: Signal<SubnetMode>,
    count_input: Signal<String>,
    result: Signal<Option<Result<CalculationResult, Ipv4InputError>>>,
) -> Element {
    // Logic for the Calculate button's disabled state
    let is_disabled = if *mode.read() == SubnetMode::Inspect {
        false
    } else {
        count_input.read().trim().is_empty() || count_input.read().parse::<u32>().is_err()
    };

    rsx! {
        InputContainer {
            title: "Enter IPv4 Network Information",
            is_disabled: is_disabled,
            on_calculate: move |_| {
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
            on_clear: move |_| {
                ip_input.set("192.168.1.0".to_string());
                cidr_input.set("/24".to_string()); // Ensure a default CIDR is reset
                mode.set(SubnetMode::Inspect);
                count_input.set("".to_string());
                result.set(None);
            },

            // IP Address Input using shared FormField and styling
            FormField { label: "IP Address",
                input {
                    class: INPUT_CLASS,
                    r#type: "text",
                    placeholder: "e.g. 192.168.1.0",
                    value: "{ip_input}",
                    oninput: move |evt| ip_input.set(evt.value())
                }
            }

            // Subnet Mask Selector using shared styling and specific IPv4 options
            FormField { label: "Subnet Mask",
                select {
                    class: "{INPUT_CLASS} pr-10 appearance-none",
                    style: SELECT_STYLE,
                    value: "{cidr_input}",
                    onchange: move |evt| cidr_input.set(evt.value()),

                    { CIDR_OPTIONS.iter().map(|&(_prefix, cidr, mask)| {
                        rsx! {
                            option {
                                key: "{cidr}",
                                value: "{cidr}",
                                selected: *cidr_input.read() == *cidr,
                                "{cidr} — {mask}"
                            }
                        }
                    })}
                }
            }

            // Subnet Mode Selector
            FormField { label: "Subnet Mode",
                select {
                    class: "block w-60 px-4 py-3 pr-10 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none bg-gray-700",
                    style: SELECT_STYLE,
                    onchange: move |e| {
                        mode.set(match e.value().as_str() {
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

            // Conditional field for Hosts or Subnets
            if *mode.read() != SubnetMode::Inspect {
                FormField { 
                    label: if *mode.read() == SubnetMode::ByHosts { "Number of Hosts Needed" } else { "Number of Subnets Needed" },
                    input {
                        r#type: "number",
                        min: "1",
                        class: "{INPUT_CLASS} hide-number-spinner",
                        placeholder: "e.g. 32",
                        value: "{count_input}",
                        oninput: move |e| count_input.set(e.value())
                    }
                }
            }
        }
    }
}