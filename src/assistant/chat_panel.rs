use dioxus::prelude::*;
use ollama_rs::Ollama;
use futures_util::StreamExt;
use crate::assistant::types::Message;
use crate::assistant::OllamaClient;
use dioxus_markdown::Markdown;
use pulldown_cmark::{Parser, Options, html};

#[component]
pub fn AssistantTab() -> Element {
    let mut messages = use_signal(Vec::<Message>::new);
    let mut input_text = use_signal(String::new);

    let chat_task = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        let ollama = OllamaClient::new(None);
        
        while let Some(user_query) = rx.next().await {
            messages.write().push(Message { role: "user".into(), content: user_query.clone() });
            messages.write().push(Message { role: "assistant".into(), content: "".into() });

            let mut stream = ollama.send_chat(user_query).await;
            let mut current_response = String::new();
            while let Some(Ok(res)) = stream.next().await {
                current_response.push_str(&res.message.content);
                if let Some(last) = messages.write().last_mut() {
                    last.content = current_response.clone();
                }
            }
        }
    });

    rsx! {
        div { class: "w-full h-150 overflow-y-auto bg-gray-800 rounded-lg shadow-lg p-6 flex flex-col",
            // Message Display Area
            for msg in messages.read().iter() {
                ChatMessage { msg: msg.clone() }
            }
            div { class: "p-4 bg-white dark:bg-gray-800 flex gap-2",
                input {
                    class: "flex-1 p-3 rounded-lg border dark:border-gray-600 dark:bg-gray-900 focus:ring-2 focus:ring-blue-500 outline-none transition-all",
                    placeholder: "Ask about CIDR calculation, wildcards, or VLANs...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !input_text.read().is_empty() {
                            chat_task.send(input_text.read().clone());
                            input_text.set("".to_string());
                        }
                    }
                }
            }
        }
    }
}





#[component]
fn ChatMessage(msg: Message) -> Element {
    let is_user = msg.role == "user";
    // We change the alignment to use flex-col and items-start/end
    // to ensure the entire bubble block is positioned correctly.
    let alignment = if is_user { "items-end" } else { "items-start" };
    let bubble_bg = if is_user { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-100 border border-gray-700" };

    // Parse Markdown to HTML string (Your existing logic)
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&msg.content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    rsx! {
        // Use 'flex' with 'alignment' to position the bubble on the left or right
        div { class: "flex {alignment} w-full px-4 py-2",
            div { 
                // REMOVED: 'w-fit' -> This allows the bubble to expand
                // ADDED: 'w-full' or simply omitting width will make it take available space
                class: "max-w-[85%] p-4 rounded {bubble_bg} text-left break-words ",
                
                div { 
                    // REMOVED: 'w-fit', 'prose-p:inline'
                    // Keeping standard prose layout ensures the block elements fill the bubble width
                    class: "prose prose-sm dark:prose-invert max-w-none text-left",
                    dangerous_inner_html: "{html_output}" 
                }
            }
        }
    }
}