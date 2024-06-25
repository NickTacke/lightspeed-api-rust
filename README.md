# Lightspeed API Rust Crate

This crate provides a Rust interface for the Lightspeed eCommerce (C-Series) API. It allows you to easily integrate Lightspeed's powerful e-commerce features into your Rust applications.

## Features

- Simple and intuitive API client
- Asynchronous operations using Tokio
- Comprehensive error handling
- Support for all major Lightspeed API endpoints
- Easily extensible for custom use cases

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lightspeed_api = { git = "https://github.com/NickTacke/lightspeed-api-rust.git" }
```

## Quick Start

Here's a simple example to get you started:

```rust
use lightspeed_api::{LightspeedClient, AccountResource};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = LightspeedClient::new("live", "your_api_key", "your_api_secret", "en");
    let account = AccountResource::new(&client);
    
    let account_info = account.get().await?;
    println!("Account info: {:?}", account_info);

    Ok(())
}
```

## Usage

### Initializing the Client

First, create an instance of `LightspeedClient` with your Lightspeed API credentials:

```rust
let client = LightspeedClient::new("live", "your_api_key", "your_api_secret", "en");
```

### Working with Resources

Each API resource (e.g., products, orders, customers) has its own struct. Here's an example using the `ProductResource`:

```rust
use lightspeed_api::{LightspeedClient, ProductResource};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = LightspeedClient::new("live", "your_api_key", "your_api_secret", "en");
    let products = ProductResource::new(&client);

    // Create a new product
    let new_product = json!({
        "title": "New Product",
        "description": "This is a new product",
        "price": 19.99
    });
    let created_product = products.create(&new_product).await?;

    // Get a product
    let product = products.get("product_id").await?;

    // Update a product
    let update_data = json!({"price": 24.99});
    let updated_product = products.update("product_id", &update_data).await?;

    // Delete a product
    let deleted_response = products.delete("product_id").await?;

    Ok(())
}
```

## Available Resources

- `AccountResource`
- `ProductResource`
- `OrderResource`
- `CustomerResource`
- More coming soon!

## Error Handling

This crate uses a custom `LightspeedApiError` for API-related errors. You can catch and handle these errors in your code:

```rust
match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(e) => {
        if let Some(api_error) = e.downcast_ref::<LightspeedApiError>() {
            println!("API Error: {} (Status: {})", api_error.message, api_error.status_code);
        } else {
            println!("Other error: {}", e);
        }
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This crate is not officially associated with or endorsed by Lightspeed. Use at your own risk.
