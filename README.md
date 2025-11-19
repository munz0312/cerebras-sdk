# cerebras-sdk

A Rust SDK for the Cerebras AI API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cerebras-sdk = "0.1.0"
```

## Usage

Set your API key as an environment variable:

```bash
export API_KEY=your_api_key_here
```

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, ChatResponse, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path(".env").ok();
    let api_key = env::var("API_KEY").expect("env variable API_KEY must be set");

    let client = CerebrasClient::new(api_key)?;

    let request = ChatRequestBuilder::builder()
        .message("What is the C programming language", Role::User)
        .seed(42)
        .build();

    let response = client.send(request).await?;
    let content = &response.choices[0].message.content;
    println!("{}", content);

    Ok(())
}
```

## Features

- Type-safe client for the Cerebras AI API
- Builder pattern for chat requests
- Support for multiple AI models
- Async/await support

## Supported Models

- `Llama31_8B`
- `Llama33_70B`
- `OpenAIGptOss`
- `Qwen3_32B`

## License

This project is licensed under the MIT License.