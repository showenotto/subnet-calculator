use dioxus::prelude::*;
use ollama_rs::Ollama;
use futures_util::StreamExt;
use crate::assistant::types::Message;
use crate::assistant::OllamaClient;
use pulldown_cmark::{Parser, Options, html};
use crate::components::forms::{SELECT_STYLE};



#[component]
pub fn AssistantTab() -> Element {
    let mut messages = use_signal(Vec::<Message>::new);
    let mut input_text = use_signal(String::new);
    let mut is_generating = use_signal(|| false);
    
    let mut ollama_installed = use_signal(|| true);
    let mut available_models = use_signal(Vec::<String>::new);
    let mut selected_model = use_signal(|| "gemma3:1b".to_string());
    let mut show_settings = use_signal(|| false);
    let current_model_name = selected_model.read().clone();

    use_future(move || async move {
        if let Ok(models) = Ollama::default().list_local_models().await {
            let names: Vec<String> = models.into_iter().map(|m| m.name).collect();
            available_models.set(names.clone());
            
            // Critical fix: Only set the selected model if models actually exist
            if !names.is_empty() {
                if !names.contains(&selected_model.read()) {
                    selected_model.set(names[0].clone());
                }
            }
        } else {
            ollama_installed.set(false);
        }
    });

    let chat_task = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
    let client = OllamaClient::new(selected_model.read().clone());
    
    while let Some(user_query) = rx.next().await {
        is_generating.set(true);
        messages.write().push(Message { role: "user".into(), content: user_query.clone() });
        messages.write().push(Message { role: "assistant".into(), content: "".into() });

        // 1. Get the Option<Stream>
        let stream_option = client.send_chat(user_query).await;
        
        // 2. Safely unwrap the stream
        if let Some(mut stream) = stream_option {
            let mut current_response = String::new();
            
            // 3. Now you can call .next() on the actual stream
            while let Some(Ok(res)) = stream.next().await {
                if !is_generating.read().clone() {
                    break;
                }

                current_response.push_str(&res.message.content);
                if let Some(last) = messages.write().last_mut() {
                    last.content = current_response.clone();
                }
            }
        } else {
            // Optional: Handle the case where the stream failed to initialize
            if let Some(last) = messages.write().last_mut() {
                last.content = "Error: Could not connect to the AI model. Please check if Ollama is running.".to_string();
            }
        }
        
        is_generating.set(false);
    }
});

    //Checks if ollama is installed
    if !ollama_installed.read().clone() {
        return rsx! {
            div { class: "p-8 text-center bg-red-900/20 border border-red-500 rounded-xl",
                h2 { class: "text-red-500 font-bold text-xl", "Ollama Not Detected" }
                p { class: "text-gray-300 mt-2", "Please ensure Ollama is installed and running on your system." }
                a { class: "text-blue-400 underline mt-4 block", href: "https://ollama.com", "Download Ollama" }
                p { class: "text-gray-300 mt-2", "After installing ollama I recommend installing models like gemma3:1b, gemma3:4b or any that is compatible with your hardware."}
            }
        };
    }
    //Checks whether models are availale.
    if available_models.read().is_empty() {
        return rsx! {
            div { class: "p-8 text-center bg-orange-900/20 border border-orange-500 rounded-xl",
                h2 { class: "text-orange-500 font-bold text-xl", "No Models Found" }
                p { class: "text-gray-300 mt-2", "Ollama is running, but you haven't downloaded any models yet." }
                p { class: "text-xs text-gray-400 mt-4 font-mono bg-black p-2 rounded", "ollama pull gemma3:1b" }
                p { class: "text-gray-400 mt-2 text-sm", "Run the command above in your terminal to get started." }
            }
        };
    }

    rsx! {
        div { class: "flex flex-col h-150 border rounded-xl bg-gray-800 shadow-lg overflow-hidden",
            // Header with Model Info and Settings Toggle
            div { class: "p-4 border-b border-gray-700 flex justify-between items-center bg-gray-900",
                div { class: "flex flex-col",
                    span { class: "text-xs text-gray-400", "Model: {selected_model}" }
                }
                div { class: "flex gap-2",
                    button { 
                        class: "text-xs bg-gray-700 hover:bg-gray-600 text-white px-3 py-1 rounded transition-all",
                        onclick: move |_| {
                            let current = *show_settings.read(); // Read and copy the value immediately
                            show_settings.set(!current);         // The borrow from .read() is gone here
                        },
                        "Settings"
                    }
                    button { 
                        class: "text-xs bg-red-900/30 hover:bg-red-900/50 text-red-400 px-3 py-1 rounded border border-red-800 transition-all",
                        onclick: move |_| messages.set(Vec::new()),
                        "Clear Chat"
                    }
                }
            }

            // Settings Dropdown
            if *show_settings.read() {
                // Capture current model to avoid repeated reads in the loop

                div { class: "p-4 bg-gray-900 border-b border-gray-700 flex flex-col gap-2",
                    label { class: "text-xs text-gray-400 font-semibold", "Select AI Model:" }
                    div { class: "relative", // Wrapper for custom arrow positioning
                        select { 
                            class: "w-full p-2 rounded border border-gray-600 outline-none focus:ring-2 focus:ring-blue-500 appearance-none bg-gray-700 text-white cursor-pointer",
                            style: SELECT_STYLE,
                            onchange: move |e| {
                                selected_model.set(e.value());
                            },
                            // Loop through models and check against the LIVE signal value
                            for model in available_models.read().iter() {
                                option { 
                                    value: "{model}", 
                                    // FIXED: This now checks the actual selected signal
                                    selected: model == &current_model_name, 
                                    "{model}" 
                                }
                            }
                        }
                    }
                }
            }

            // Message Display Area
            div { class: "flex-1 overflow-y-auto p-4 flex flex-col gap-2",
                for msg in messages.read().iter() {
                    ChatMessage { msg: msg.clone() }
                }
            }
            
            // Input Bar
            div { class: "p-4 bg-white dark:bg-gray-800 border-t dark:border-gray-700 flex gap-2",
                input {
                    class: "flex-1 p-3 rounded-lg border dark:border-gray-600 dark:bg-gray-900 focus:ring-2 focus:ring-blue-500 outline-none transition-all",
                    placeholder: "Ask about CIDR calculation...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !input_text.read().is_empty() && !is_generating.read().clone() {
                            chat_task.send(input_text.read().clone());
                            input_text.set("".to_string());
                        }
                    }
                }

                // Dynamic Stop/Send Button
                if is_generating.read().clone() {
                    button { 
                        class: "px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-all font-bold",
                        onclick: move |_| is_generating.set(false),
                        "Stop"
                    }
                } else {
                    button { 
                        class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-all font-bold",
                        onclick: move |_| {
                            if !input_text.read().is_empty() {
                                chat_task.send(input_text.read().clone());
                                input_text.set("".to_string());
                            }
                        },
                        "Send"
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