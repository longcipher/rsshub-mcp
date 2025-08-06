# RSSHub API Client

A Rust client library for interacting with [RSSHub](https://rsshub.app/) APIs, providing type-safe access to namespace information, radar rules, and category data.

## Overview

This crate provides a comprehensive Rust client for the RSSHub API, allowing developers to programmatically access RSS feed information, discover available routes, and retrieve metadata about various content sources.

## Features

- **Complete API Coverage**: Implements all 6 major RSSHub API endpoints
- **Type Safety**: Strongly typed responses with comprehensive data structures
- **Async Support**: Built on `tokio` and `reqwest` for high-performance async operations
- **Error Handling**: Uses `eyre` for ergonomic error handling
- **Configurable**: Customizable host and timeout settings
- **Well Tested**: Comprehensive test suite with mock server testing

## API Methods

### Core Methods

1. **`get_all_namespaces()`** - Retrieve all available namespaces
2. **`get_namespace(namespace)`** - Get routes for a specific namespace
3. **`get_all_radar_rules()`** - Get all radar rules for automatic feed detection
4. **`get_radar_rule(domain)`** - Get a specific radar rule by domain name
5. **`get_categories()`** - Get all available content categories
6. **`get_category(category)`** - Get feeds for a specific category

## Usage

### Basic Setup

```rust
use rsshub_api::{RsshubApiClient, RsshubClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = RsshubApiClient::new(RsshubClientConfig::default());
    
    // Or customize the configuration
    let config = RsshubClientConfig {
        host: Some("https://rsshub.example.com".to_string()),
        timeout: Some(60), // 60 seconds
    };
    let client = RsshubApiClient::new(config);
    
    Ok(())
}
```

### Getting Namespaces

```rust
// Get all available namespaces
let namespaces = client.get_all_namespaces().await?;
println!("Available namespaces: {:#?}", namespaces);

// Get specific namespace routes
let bilibili_routes = client.get_namespace("bilibili").await?;
println!("Bilibili routes: {:#?}", bilibili_routes);
```

### Working with Radar Rules

```rust
// Get all radar rules
let rules = client.get_all_radar_rules().await?;
println!("All radar rules: {:#?}", rules);

// Get specific domain rule
let domain_rule = client.get_radar_rule("github.com").await?;
println!("GitHub radar rule: {:#?}", domain_rule);
```

### Exploring Categories

```rust
// Get all categories
let categories = client.get_categories().await?;
println!("Available categories: {:#?}", categories);

// Get specific category feeds
let programming_feeds = client.get_category("programming").await?;
println!("Programming feeds: {:#?}", programming_feeds);
```

## Data Structures

### Core Types

- **`RsshubApiClient`** - Main client for API interactions
- **`RsshubClientConfig`** - Configuration for client initialization
- **`NamespaceResp`** - Response containing namespace information
- **`RulesResp`** - Response containing radar rules
- **`CategoryItems`** - Response containing category feeds

### Route Information

- **`RouteDetails`** - Detailed information about a specific route
- **`Features`** - Feature flags for routes (e.g., requires config, puppeteer)
- **`RadarItem`** - Individual radar rule configuration

## Configuration

### Default Settings

- **Host**: `https://rsshub.akjong.com`
- **Timeout**: 120 seconds

### Custom Configuration

```rust
let config = RsshubClientConfig {
    host: Some("https://your-rsshub-instance.com".to_string()),
    timeout: Some(30), // 30 seconds timeout
};
```

## Testing

The crate includes comprehensive tests with mock server support:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_get_all_namespaces
```

## Error Handling

The library uses `eyre` for error handling, providing rich error context:

```rust
match client.get_namespace("invalid").await {
    Ok(routes) => println!("Routes: {:#?}", routes),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Dependencies

- **`reqwest`** - HTTP client with JSON support
- **`serde`** - Serialization/deserialization
- **`tokio`** - Async runtime
- **`eyre`** - Error handling

## Development Dependencies

- **`mockito`** - HTTP mocking for tests
- **`tokio-test`** - Testing utilities

## License

MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [RSSHub Documentation](https://rsshub.app/)
- [RSSHub API Reference](https://rsshub.app/api/reference)
- [GitHub Repository](https://github.com/akjong/rsshub-mcp)
