use reqwest::Client;

pub struct CerebrasClient {
    http: Client,
    api_key: String,
}

impl CerebrasClient {
    pub fn new(api_key: impl Into<String>) -> CerebrasClient {
        CerebrasClient {
            http: Client::builder().use_rustls_tls().build().unwrap(),
            api_key: api_key.into(),
        }
    }
}

pub mod chat {
    use crate::models::ChatModel;
    use crate::{
        client::CerebrasClient,
        models::{ChatRequest, Message},
    };
    use reqwest::{Error, Method, Response};
    const URL: &str = "https://api.cerebras.ai/v1/chat/completions";

    impl CerebrasClient {
        pub async fn completion(
            &self,
            content: String,
            model: ChatModel,
        ) -> Result<Response, Error> {
            let message = Message {
                content,
                role: "user".to_string(),
            };
            let request = ChatRequest {
                model: model.into(),
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
                .header("Authorization", format!("Bearer {}", &self.api_key))
                .json(&request)
                .send()
                .await?;

            Ok(response)
        }
    }
}
