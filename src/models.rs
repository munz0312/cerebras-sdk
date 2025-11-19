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

pub struct ChatResponse {
    id: String,
    choices: Vec<Choice>,
    created: i64,
    model: String,
    system_fingerprint: String,
    usage: Usage,
    time_info: TimeInfo,
}

pub struct Choice {
    finish_reason: String,
    index: u32,
    message: Message,
}

pub struct Usage {
    total_tokens: u32,
    completion_tokens: u32,
    prompt_tokens: u32,
}

pub struct TimeInfo {
    queue_time: f64,
    prompt_time: f64,
    completion_time: f64,
    total_time: f64,
    created: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

pub enum ChatModel {
    Llama31_8B,
    Llama33_70B,
    OpenAIGptOss,
    Qwen3_32B,
}

impl From<ChatModel> for String {
    fn from(model: ChatModel) -> String {
        model.as_str().to_string()
    }
}

impl ChatModel {
    fn as_str(&self) -> &str {
        match self {
            ChatModel::Llama31_8B => "llama3.1-8b",
            ChatModel::Llama33_70B => "llama-3.3-70b",
            ChatModel::OpenAIGptOss => "gpt-oss-120b",
            ChatModel::Qwen3_32B => "qwen-3-32b",
        }
    }
}
