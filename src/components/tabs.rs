use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Ipv4,
    Ipv6,
    Converter,
}

#[component]
pub fn Tabs(active_tab: ActiveTab, on_tab_change: EventHandler<ActiveTab>) -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4",
            div { class: "flex space-x-1 border-b border-gray-300 dark:border-gray-700 -mb-px",
                TabButton {
                    label: "IPv4",
                    active: active_tab == ActiveTab::Ipv4,
                    onclick: move |_| on_tab_change.call(ActiveTab::Ipv4)
                }
                TabButton {
                    label: "IPv6",
                    active: active_tab == ActiveTab::Ipv6,
                    onclick: move |_| on_tab_change.call(ActiveTab::Ipv6)
                }
                TabButton {
                    label: "Converter",
                    active: active_tab == ActiveTab::Converter,
                    onclick: move |_| on_tab_change.call(ActiveTab::Converter)
                }
            }
        }
    }
}

#[component]
fn TabButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let base = "px-6 py-3 font-medium transition-colors rounded-t-lg";
    let active_class = if active {
        "border-b-4 border-blue-500 text-blue-600 dark:text-blue-400  dark:bg-gray-900 shadow-sm"
    } else {
        "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800"
    };

    rsx! {
        button {
            class: "{base} {active_class} -mb-px",
            onclick: move |evt| {
                evt.stop_propagation();
                onclick.call(evt);
            },
            {label}
        }
    }
}