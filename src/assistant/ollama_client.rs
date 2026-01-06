use ollama_rs::Ollama;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponseStream};
use crate::assistant::types::Message;

pub struct OllamaClient {
    client: Ollama,
    model: String,
}

impl OllamaClient {
    pub fn new(model: Option<String>) -> Self {
        Self {
            client: Ollama::default(),
            model: model.unwrap_or_else(|| "gemma3:1b".to_string()),
        }
    }

    /// Sends a chat request and returns a stream of responses
    pub async fn send_chat(&self, user_query: String) -> ChatMessageResponseStream {
        // Here we can inject the "System Prompt" to make it a networking expert
        let system_msg = ChatMessage::system(
            "You are a Subnetting Expert. Provide precise networking calculations. \
             Use CIDR notation and binary where helpful.".to_string()
        );
        let user_msg = ChatMessage::user(user_query);

        self.client
            .send_chat_messages_stream(ChatMessageRequest::new(
                self.model.clone(),
                vec![system_msg, user_msg],
            ))
            .await
            .expect("Failed to connect to Ollama. Is it running?")
    }
}