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

#[derive(Debug, Clone)]
pub struct ChatRequestBuilder {
    pub model: ChatModel,
    // pub stream: bool,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub seed: u32,
    pub top_p: f32,
}

impl ChatRequestBuilder {
    pub fn builder() -> ChatRequestBuilder {
        ChatRequestBuilder {
            model: ChatModel::Llama31_8B,
            // stream: false,
            messages: Vec::new(),
            temperature: 0.0,
            max_tokens: -1,
            seed: 0,
            top_p: 1.0,
        }
    }

    pub fn model(mut self, model: ChatModel) -> Self {
        self.model = model;
        self
    }

    pub fn message(mut self, content: &str, role: Role) -> Self {
        self.messages.push(Message {
            content: content.to_string(),
            role,
        });
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn max_tokens(mut self, max: i32) -> Self {
        self.max_tokens = max;
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest {
            model: self.model.into(),
            stream: false,
            messages: self.messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            seed: self.seed,
            top_p: self.top_p,
        }
    }
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
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
}

#[derive(Debug, Clone)]
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
