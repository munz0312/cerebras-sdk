use cerebras_sdk::{CerebrasClient, ChatModel};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let api_key = env::var("API_KEY").expect("env variable API_KEY must be set");
    let client = CerebrasClient::new(api_key);
    let response = client
        .completion(
            "Explain what the Rust Programming Language is".to_string(),
            ChatModel::Llama31_8B,
        )
        .await?;
    println!("Status code: {}", response.status());
    println!("Response text: {}", response.text().await?);
    Ok(())
}
