use reqwest::Client;

pub struct CerebrasClient {
    http: Client,
    api_key: String,
    model: String,
}

impl CerebrasClient {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> CerebrasClient {
        CerebrasClient {
            http: Client::builder().use_rustls_tls().build().unwrap(),
            api_key: api_key.into(),
            model: model.into(),
        }
    }
}

pub mod chat {
    use crate::error::APIError;
    use crate::{
        client::CerebrasClient,
        models::{ChatRequest, Message},
    };
    use reqwest::{Error, Method, Response};
    const URL: &str = "https://api.cerebras.ai/v1/chat/completions";

    impl CerebrasClient {
        pub async fn completion(&self, content: String) -> Result<Response, Error> {
            let message = Message {
                content,
                role: "user".to_string(),
            };
            let request = ChatRequest {
                model: self.model.clone(),
                stream: false,
                messages: vec![message],
                temperature: 0.0,
                max_tokens: -1,
                seed: 0,
                top_p: 1.0,
            };
            let response = self
                .http
                .request(Method::POST, URL)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key.clone()))
                .json(&request)
                .send()
                .await?;

            Ok(response)
        }
    }
}
