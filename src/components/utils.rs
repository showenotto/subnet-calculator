use dioxus::prelude::*;
use arboard::Clipboard;
use rfd::FileDialog;

use crate::ipv4::{calculator::LAST_N, types::IpSubnetResult};

#[component]
pub fn CopyButton(get_text: ReadSignal<String>) -> Element {
    let mut copied = use_signal(|| false);

    // This effect ensures that even if the key fails, 
    // the button resets when the text changes.
    use_effect(move || {
        let _ = get_text.read();
        copied.set(false);
    });

    rsx! {
        button {
            // Using copied() directly ensures the button re-renders 
            // the moment the signal changes.
            disabled: copied(),
            class: "ml-2 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition disabled:opacity-50",
            onclick: move |_| {
                let text_to_copy = get_text.read().clone();
                spawn(async move {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if clipboard.set_text(text_to_copy).is_ok() {
                            copied.set(true);
                            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                            copied.set(false);
                        }
                    }
                });
            },
            if copied() { "Copied!" } else { "Copy" }
        }
    }
}



#[component]
pub fn ExportButton(default_filename: String, mime: String, get_content: ReadSignal<String>, #[props(default = "Export CSV".to_string())] label: String) -> Element {
    rsx! {
        button {
            class: "ml-2 px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition",
            onclick: move |_| {
                let content = get_content();
                let bytes = content.as_bytes().to_vec();  // or use Vec<u8> directly if your content is binary

                if let Some(path) = FileDialog::new()
                    .set_file_name(&default_filename)
                    .save_file()
                {
                    let _ = std::fs::write(path, bytes);
                    // Optional: show success feedback in UI
                }
            },
            "{label}"
        }
    }
}

#[component]
pub fn SummaryRow(label: &'static str, value: String) -> Element {
    rsx! {
        tr { class: "border-b border-gray-700", 
            th { class: "px-4 py-3 font-medium text-gray-300 w-1/3", span {"{label}" }}
            td { class: "px-4 py-3 break-all", span {"{value}" }}
        }
    }
}

pub fn generate_subnets_text(
    subnets: &[IpSubnetResult],
    total_subnets: u128,
    is_truncated: bool,
    first_k: usize,
) -> String {
    let mut text = String::new();

    for (i, sub) in subnets.iter().enumerate() {
        let id = if is_truncated && i < first_k {
            (i + 1) as u128
        } else if is_truncated {
            total_subnets - (LAST_N as u128 - 1) + (i - first_k) as u128
        } else {
            (i + 1) as u128
        };

        if is_truncated && i == first_k {
            text.push_str("...\n");
            continue;
        }

        match sub {
            IpSubnetResult::V4(v4) => {
                text.push_str(&format!(
                    "{}\t{}\t{} → {}\t{}\n",
                    id,
                    v4.network,
                    v4.first_host.as_deref().unwrap_or("-"),
                    v4.last_host.as_deref().unwrap_or("-"),
                    v4.broadcast
                ));
            }
            IpSubnetResult::V6(v6) => {
                text.push_str(&format!(
                    "{}\t{}\t{} → {}\tN/A\n",
                    id,
                    v6.compressed,
                    v6.first_host,
                    v6.last_host
                ));
            }
        }
    }

    text
}

pub fn generate_summary_csv(labels: &[&str], values: &[String]) -> String {
    let mut csv = String::from("Label,Value\n");
    for (label, value) in labels.iter().zip(values.iter()) {
        // Simple escaping: wrap in quotes if contains comma
        let safe_value = if value.contains(',') { format!("\"{}\"", value) } else { value.clone() };
        csv.push_str(&format!("{},{}\n", label, safe_value));
    }
    csv
}

pub fn generate_subnets_csv(
    subnets: &[IpSubnetResult],
    total_subnets: u128,
    is_truncated: bool,
    first_k: usize,
) -> String {
    let mut csv = String::from("ID,Network,First Host,Last Host,Broadcast\n");
    
    for (i, sub) in subnets.iter().enumerate() {
        let id = if is_truncated && i < first_k {
            (i + 1) as u128
        } else if is_truncated {
            total_subnets - (LAST_N as u128 - 1) + (i - first_k) as u128
        } else {
            (i + 1) as u128
        };

        if is_truncated && i == first_k { continue; }

        match sub {
            IpSubnetResult::V4(v4) => {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    id, v4.network, 
                    v4.first_host.as_deref().unwrap_or("-"),
                    v4.last_host.as_deref().unwrap_or("-"),
                    v4.broadcast
                ));
            }
            IpSubnetResult::V6(v6) => {
                csv.push_str(&format!(
                    "{},{},{},{},N/A\n",
                    id, v6.compressed, v6.first_host, v6.last_host
                ));
            }
        }
    }
    csv
}