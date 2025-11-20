use crate::{ChatRequest, ChatResponse};
use reqwest::{Client, Error};

const URL: &str = "https://api.cerebras.ai/v1/chat/completions";

pub struct CerebrasClient {
    http: Client,
    api_key: String,
}

impl CerebrasClient {
    /// Creates a new client
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cerebras_sdk::CerebrasClient;
    /// let client = CerebrasClient::new("your-api-key")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<CerebrasClient, reqwest::Error> {
        Ok(CerebrasClient {
            http: Client::builder().use_rustls_tls().build()?,
            api_key: api_key.into(),
        })
    }

    /// Chat completion
    ///
    /// Build a [`ChatRequest`] using [`ChatRequestBuilder`],
    /// then send it to this method to get a response
    ///
    /// [`ChatRequest`]: crate::ChatRequest
    /// [`ChatRequestBuilder`]: crate::ChatRequestBuilder
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, Error> {
        let response = self
            .http
            .post(URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }
}
