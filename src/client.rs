use reqwest::Client;

pub struct CerebrasClient {
    http: Client,
    api_key: String,
}

impl CerebrasClient {
    pub fn new(api_key: impl Into<String>) -> Result<CerebrasClient, reqwest::Error> {
        Ok(CerebrasClient {
            http: Client::builder().use_rustls_tls().build()?,
            api_key: api_key.into(),
        })
    }
}

pub mod chat {
    use crate::{client::CerebrasClient, models::ChatRequest};
    use reqwest::{Error, Method, Response};
    const URL: &str = "https://api.cerebras.ai/v1/chat/completions";

    impl CerebrasClient {
        pub async fn send(&self, request: ChatRequest) -> Result<Response, Error> {
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
