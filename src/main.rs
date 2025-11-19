use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, ChatResponse, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let api_key = env::var("API_KEY").expect("env variable API_KEY must be set");
    let client = CerebrasClient::new(api_key);
    let request = ChatRequestBuilder::builder()
        .message("What is the C programming language", Role::User)
        .seed(42)
        .build();
    let response = client.send(request).await?;
    let response: ChatResponse = response.json().await?;
    let content = &response.choices[0].message.content;
    println!("{}", content);
    Ok(())
}
