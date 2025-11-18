use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub seed: u32,
    pub top_p: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}
