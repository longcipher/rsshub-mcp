# MCP Client

A testing and validation client for the RSSHub MCP (Model Context Protocol) server, providing tools for development, debugging, and integration testing.

## Overview

This crate contains client applications designed to test and validate the functionality of the RSSHub MCP server. It includes both quick functionality tests and comprehensive integration tests to ensure the MCP server is working correctly.

## Features

- **Quick Testing**: Fast functionality validation with `quick_test`
- **Simple Testing**: Basic connection and communication testing with `simple_test`
- **MCP Protocol**: Full Model Context Protocol implementation for testing
- **HTTP Transport**: Tests HTTP-based MCP communication
- **Async Support**: Built on `tokio` for high-performance async operations
- **JSON-RPC**: Proper JSON-RPC 2.0 protocol implementation

## Binaries

### 1. `quick_test`

A comprehensive test client that validates all major MCP tools and functionality.

#### Key Features

- Tests MCP server initialization
- Validates all 8 RSSHub MCP tools (including new RSS content retrieval)
- Provides detailed output and error reporting
- Session management with unique session IDs
- Tests both discovery and content retrieval functionality

#### Usage

```bash
# Run quick functionality test
cargo run -p mcp-client --bin quick_test
```

#### Test Coverage

- `get_categories` - Validate category retrieval
- `get_namespace` - Test namespace functionality (using "bilibili")
- `get_category` - Test specific category retrieval (using "programming")
- **`search_namespaces`** 🆕 - Test namespace search functionality (searching for "bili")
- **`get_feed`** 🆕 - **Test actual RSS content retrieval (most important feature)**
- Connection initialization and session management
- Error handling and response validation

### 2. `simple_test`

A basic connection test client for fundamental MCP communication validation.

#### Core Features

- Basic MCP connection testing
- Protocol initialization validation
- Simple tool invocation testing
- Connection health checks

#### Usage

```bash
# Run basic connection test
cargo run -p mcp-client --bin simple_test
```

### 3. `generic`

A flexible CLI to call any MCP tool with JSON arguments.

```bash
# Usage
cargo run -p mcp-client --bin generic -- <tool_name> '[json_arguments]' [--url http://host:port/mcp]

# Examples
cargo run -p mcp-client --bin generic -- get_radar_rule '{"domain":"github.com"}'
RSSHUB_MCP_URL=http://127.0.0.1:8000/mcp \
    cargo run -p mcp-client --bin generic -- get_namespace '{"namespace":"bilibili"}'
```

## MCP Protocol Implementation

### JSON-RPC 2.0

The client implements proper JSON-RPC 2.0 protocol for MCP communication:

```json
{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
        "capabilities": {},
        "clientInfo": {
            "name": "quick-test-client",
            "version": "1.0.0"
        },
        "protocolVersion": "2024-11-05"
    },
    "id": "init-1"
}
```

### Session Management

Each test session uses a unique session ID for proper isolation:

```rust
let session_id = Uuid::new_v4().to_string();
```

### Tool Invocation

Example tool call structures:

#### Namespace Search

```json
{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "search_namespaces",
        "arguments": {
            "query": "bili"
        }
    },
    "id": "tool-search"
}
```

#### RSS Content Retrieval

```json
{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "get_feed",
        "arguments": {
            "path": "bilibili/user/video/2267573"
        }
    },
    "id": "tool-feed"
}
```

#### Namespace Information

```json
{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "get_namespace",
        "arguments": {
            "namespace": "bilibili"
        }
    },
    "id": "tool-namespace"
}
```

## Configuration

### Default Server Settings

- **Host**: `127.0.0.1`
- **Port**: `8000`
- **Endpoint**: `/mcp`
- **Protocol**: HTTP

### Custom Configuration

You can modify the server endpoint via environment variable or in the source code:

```rust
// Prefer environment variable when running
// RSSHUB_MCP_URL=http://127.0.0.1:8000/mcp
let url = std::env::var("RSSHUB_MCP_URL").unwrap_or_else(|_| "http://127.0.0.1:8000/mcp".to_string());
```

## Development and Testing

### Running Tests

```bash
# Run quick functionality test
cargo run -p mcp-client --bin quick_test

# Run simple connection test
cargo run -p mcp-client --bin simple_test

# Build both binaries
cargo build -p mcp-client
```

### Test Output

The tests provide detailed output showing:

```text
=== RSSHub MCP Tool Quick Test ===

1. Initializing connection...
   ✅ Initialization successful!

2. Testing get_categories...
   ✅ Success! Retrieved category data, first 100 chars: Available categories: blog, news, programming...

3. Testing get_namespace (bilibili)...
   ✅ Success! bilibili namespace data, first 100 chars: RoutesMap { routes: Some({ "/live/room/:roomID"...

4. Testing get_category (programming)...
   ✅ Success! programming category data, first 100 chars: CategoryItems({ "deeplearning": CategoryInfo...

5. Testing search_namespaces (searching for 'bili')...
   ✅ Success! Search results: Namespaces matching 'bili':
   bilibili
   sustainabilitymag

6. Testing get_feed (retrieving RSS content)...
   ✅ Success! RSS content, first 200 chars: RSS Feed: RSSHub Feed...

=== Quick test completed ===
```

### Error Handling

The client includes comprehensive error handling:

```rust
if response.status().is_success() {
    // Process successful response
    let result: Value = response.json().await?;
    // ... handle result
} else {
    println!("   ❌ 失败: {}", response.status());
}
```

## Dependencies

- **`reqwest`** - HTTP client for MCP communication
- **`serde_json`** - JSON serialization/deserialization
- **`tokio`** - Async runtime
- **`uuid`** - Session ID generation

## Use Cases

### Development Testing

Use the client during RSSHub MCP server development to:

- Validate new tool implementations (including RSS content retrieval)
- Test protocol compliance
- Debug communication issues
- Performance testing
- **Validate complete RSS workflow** from discovery to content fetching

### Integration Testing

Incorporate the client into CI/CD pipelines:

```bash
# In your CI script
cargo run -p mcp-client --bin quick_test || exit 1
cargo run -p mcp-client --bin simple_test || exit 1
```

### Manual Validation

Use for manual testing when:

- Deploying to new environments
- Validating configuration changes
- Troubleshooting connection issues
- **Verifying RSS content retrieval functionality**
- Testing namespace search efficiency

## Troubleshooting

### Common Issues

1. **Connection Refused**

   ```text
   Error: Connection refused
   ```

   - Ensure RSSHub MCP server is running
   - Check server address and port
   - Verify firewall settings

2. **Tool Errors**

   ```text
   ❌ 失败: 500 Internal Server Error
   ```

   - Check server logs for errors
   - Validate tool parameters
   - Ensure RSSHub backend is accessible

3. **Session Issues**

   ```text
   Error: Invalid session
   ```

   - Restart both client and server
   - Check session ID generation

### Debug Mode

Enable debug logging for detailed troubleshooting:

```bash
RUST_LOG=debug cargo run -p mcp-client --bin quick_test
```

## Extending the Client

### Adding New Tests

To add tests for new MCP tools:

```rust
// Add new tool test
println!("\n7. Testing new_tool...");
let tool_request = json!({
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
        "name": "new_tool",
        "arguments": {
            "param": "value"
        }
    },
    "id": "tool-new"
});

// Send request and handle response
let response = client
    .post(url)
    .header("mcp-session-id", &session_id)
    .header("Content-Type", "application/json")
    .json(&tool_request)
    .send()
    .await?;

if response.status().is_success() {
    let result: Value = response.json().await?;
    if let Some(content) = result
        .get("result")
        .and_then(|r| r.get("content"))
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|item| item.get("text"))
        .and_then(|text| text.as_str())
    {
        println!("   ✅ Success! New tool result: {content}");
    }
} else {
    println!("   ❌ Failed: {}", response.status());
}
```

### Custom Test Scenarios

Create specialized test scenarios by:

1. Copying existing test files
2. Modifying tool calls and parameters
3. Adding custom validation logic
4. Building new binary targets

## Performance Testing

The client can be extended for performance testing:

```rust
// Example: Concurrent tool calls
let futures = (0..10).map(|i| {
    client.post(url)
        .header("mcp-session-id", &session_id)
        .json(&tool_request)
        .send()
});

let results = futures::future::join_all(futures).await;
```

## License

MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! When adding new test scenarios:

- Follow existing code patterns
- Add comprehensive error handling
- Include detailed output messages
- Test with various server configurations

## Links

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [RSSHub Documentation](https://rsshub.app/)
- [GitHub Repository](https://github.com/akjong/rsshub-mcp)
