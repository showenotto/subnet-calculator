use ollama_rs::Ollama;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponseStream};
use ollama_rs::models::LocalModel;

pub struct OllamaClient {
    client: Ollama,
    pub model: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self {
            client: Ollama::default(),
            model,
        }
    }

    /// Check if Ollama is reachable
    pub async fn is_available() -> bool {
        Ollama::default().list_local_models().await.is_ok()
    }

    /// Get a list of installed models
    pub async fn get_local_models() -> Vec<String> {
        match Ollama::default().list_local_models().await {
            Ok(models) => models.into_iter().map(|m| m.name).collect(),
            Err(_) => vec![],
        }
    }

    pub async fn send_chat(&self, user_query: String) -> Option<ChatMessageResponseStream> {
    let system_msg = ChatMessage::system(
        "You are a Subnetting Expert. Provide precise networking calculations.".to_string()
    );
    let user_msg = ChatMessage::user(user_query);

    // Use match instead of expect to avoid panicking the entire thread
    match self.client
        .send_chat_messages_stream(ChatMessageRequest::new(
            self.model.clone(),
            vec![system_msg, user_msg],
        ))
        .await {
            Ok(stream) => Some(stream),
            Err(e) => {
                eprintln!("Ollama Error: {}", e);
                None
            }
        }
    }
}