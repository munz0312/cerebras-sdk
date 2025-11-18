use std::env;

use cerebras_sdk::CerebrasClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let api_key = env::var("API_KEY").unwrap();
    let model = "llama-3.3-70b";
    let client = CerebrasClient::new(api_key, model);
    let response = client
        .completion("Explain what the Rust Programming Language is".to_string())
        .await?;
    println!("Status code: {}", response.status());
    println!("Response text: {}", response.text().await?);
    Ok(())
}
