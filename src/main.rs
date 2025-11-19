use cerebras_sdk::{CerebrasClient, ChatModel, ChatRequestBuilder};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let api_key = env::var("API_KEY").expect("env variable API_KEY must be set");
    let client = CerebrasClient::new(api_key);
    let request = ChatRequestBuilder::new()
        .message("What is the C programming language")
        .seed(42)
        .build();
    let response = client.send(request).await?;
    println!("Status code: {}", response.status());
    println!("Response text: {}", response.text().await?);
    Ok(())
}
