# Cerebras SDK

A Rust SDK for interacting with the Cerebras AI API, providing a clean and idiomatic interface for making chat completions requests.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cerebras-sdk = "0.1.0"
```

## Quick Start

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let client = CerebrasClient::new("your-api-key-here");

    // Build a chat request using the builder pattern
    let request = ChatRequestBuilder::builder()
        .message("What is the C programming language?", Role::User)
        .seed(42)
        .build();

    // Send the request and get the response
    let response = client.send(request).await?;
    let chat_response = response.json::<cerebras_sdk::ChatResponse>().await?;

    // Extract and print the response content
    let content = &chat_response.choices[0].message.content;
    println!("{}", content);

    Ok(())
}
```

## Usage

### Basic Chat Request

The SDK provides a fluent builder pattern for constructing chat requests:

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role};

let client = CerebrasClient::new("your-api-key");

let request = ChatRequestBuilder::builder()
    .message("Hello, how are you?", Role::User)
    .build();

let response = client.send(request).await?;
let chat_response = response.json::<cerebras_sdk::ChatResponse>().await?;
```

### Advanced Configuration

You can customize various parameters of your chat request:

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role, ChatModel};

let client = CerebrasClient::new("your-api-key");

let request = ChatRequestBuilder::builder()
    .model(ChatModel::Llama33_70B)  // Choose a different model
    .message("Explain quantum computing", Role::User)
    .temperature(0.7)              // Control randomness (0.0 to 1.0)
    .max_tokens(1000)             // Limit response length
    .top_p(0.9)                   // Nucleus sampling
    .seed(123)                    // For reproducible results
    .build();

let response = client.send(request).await?;
```

### Conversation History

Maintain conversation context by including message history:

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role};

let client = CerebrasClient::new("your-api-key");

let request = ChatRequestBuilder::builder()
    .message("You are a helpful assistant.", Role::System)
    .message("What is Rust?", Role::User)
    .message("Rust is a systems programming language focused on safety and performance.", Role::Assistant)
    .message("Can you give me a simple example?", Role::User)
    .build();

let response = client.send(request).await?;
```

## Available Models

The SDK supports the following Cerebras models:

- `ChatModel::Llama31_8B` - `llama3.1-8b`
- `ChatModel::Llama33_70B` - `llama-3.3-70b`
- `ChatModel::OpenAIGptOss` - `gpt-oss-120b`
- `ChatModel::Qwen3_32B` - `qwen-3-32b`

## API Reference

### Client

- `CerebrasClient::new(api_key)` - Create a new client instance
- `client.send(request)` - Send a chat request to the API

### ChatRequestBuilder

- `ChatRequestBuilder::builder()` - Create a new builder
- `.model(model)` - Set the AI model to use
- `.message(content, role)` - Add a message to the conversation
- `.temperature(temp)` - Set sampling temperature (0.0 to 1.0)
- `.max_tokens(max)` - Set maximum response tokens
- `.seed(seed)` - Set random seed for reproducibility
- `.top_p(value)` - Set nucleus sampling parameter
- `.build()` - Build the final ChatRequest

### Data Types

- `ChatResponse` - Complete API response including metadata
- `Message` - Individual chat messages with content and role
- `Role` - Enum: `User`, `System`, `Assistant`
- `Usage` - Token usage statistics
- `TimeInfo` - Performance timing information

## Error Handling

The SDK uses Rust's standard error handling with `Result` types. All methods return `Result<T, E>` where errors can occur:

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role};

match client.send(request).await {
    Ok(response) => {
        let chat_response = response.json::<cerebras_sdk::ChatResponse>().await?;
        println!("Success: {}", chat_response.choices[0].message.content);
    }
    Err(e) => {
        eprintln!("Request failed: {}", e);
    }
}
```

## Environment Variables

For local development, you can load your API key from environment variables:

```rust
use cerebras_sdk::{CerebrasClient, ChatRequestBuilder, Role};
use std::env;

// Load from .env file (requires the dotenv feature)
dotenv::from_path(".env").ok();

let api_key = env::var("CEREBRAS_API_KEY")
    .expect("CEREBRAS_API_KEY environment variable must be set");
let client = CerebrasClient::new(api_key);
```

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.