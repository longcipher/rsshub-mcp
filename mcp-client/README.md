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
- Validates all 6 RSSHub MCP tools
- Provides detailed output and error reporting
- Session management with unique session IDs

#### Usage

```bash
# Run quick functionality test
cargo run -p mcp-client --bin quick_test
```

#### Test Coverage

- `get_categories` - Validate category retrieval
- `get_namespace` - Test namespace functionality (using "bilibili")
- `get_category` - Test specific category retrieval (using "programming")
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

Example tool call structure:

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

You can modify the server endpoint in the source code:

```rust
let url = "http://127.0.0.1:8000/mcp";  // Change as needed
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
=== RSSHub MCP工具快速测试 ===

1. 初始化连接...
   ✅ 初始化成功!

2. 测试 get_categories...
   ✅ 成功! 获取到分类数据，前100字符: {"categories":{"programming":{"name":"Programming","feeds"...

3. 测试 get_namespace (bilibili)...
   ✅ 成功! bilibili命名空间数据，前100字符: {"routes":{"user":{"path":"/bilibili/user/:uid"...

4. 测试 get_category (programming)...
   ✅ 成功! programming分类数据，前100字符: {"feeds":{"github":{"name":"GitHub","url":"https://...

=== 快速测试完成 ===
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

- Validate new tool implementations
- Test protocol compliance
- Debug communication issues
- Performance testing

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
- Verifying tool functionality

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
println!("\n5. 测试 new_tool...");
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
