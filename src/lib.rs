//! # cerebras-sdk
//!
//! A library for accessing and interacting with Cerebra's Inference API
//!
//! ## Example usage
//!
//! ### Chat completion
//!
//! ```rust
//! use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role,
//! ChatModel};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = CerebrasClient::new("your-api-key")?;
//!
//!     let request = ChatRequestBuilder::builder()
//!         .model(ChatModel::Llama33_70B)
//!         .message("You are a helpful assistant. Explain quantum computing.", Role::System)
//!         .message("What is quantum entanglement?", Role::User)
//!         .temperature(0.7)
//!         .max_tokens(500)
//!         .top_p(0.9)
//!         .build();
//!
//!     let response = client.send(request).await?;
//!     let reply = &response.choices[0].message.content;
//!     println!("Assistant: {}", reply);
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod models;

pub use client::CerebrasClient;
pub use models::*;
