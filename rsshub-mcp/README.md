# RSSHub MCP Server

A [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server implementation for RSSHub, providing structured access to RSS feeds and content discovery through the MCP protocol.

## Overview

This MCP server bridges RSSHub's powerful RSS aggregation capabilities with the Model Context Protocol, enabling AI models and applications to discover, explore, and access RSS feeds in a structured way. Built on the `ultrafast-mcp` framework for high performance and reliability.

## Features

- **MCP Protocol Compliance**: Full implementation of the Model Context Protocol specification
- **High Performance**: Built on `ultrafast-mcp` framework for optimal speed
- **RSSHub Integration**: Direct integration with RSSHub API through the `rsshub-api` crate
- **Tool-Based Interface**: Provides 8 specialized tools for content discovery and RSS retrieval
- **RSS Content Retrieval**: Fetch actual RSS feed content, not just metadata
- **Smart Search**: Efficiently filter through hundreds of namespaces with keyword search
- **HTTP Transport**: Supports HTTP-based MCP communication
- **Configurable**: Flexible configuration options for different deployment scenarios
- **Logging**: Comprehensive logging with configurable levels

## MCP Tools

The server provides 8 tools that expose RSSHub functionality:

### Core Discovery Tools

### 1. `get_all_namespaces`

- **Description**: Retrieve all available namespaces in RSSHub
- **Parameters**: None
- **Returns**: Complete list of namespaces with their routes

### 2. `get_namespace`

- **Description**: Get detailed routes for a specific namespace
- **Parameters**:
  - `namespace` (string): The namespace identifier (e.g., "bilibili", "github")
- **Returns**: All routes available within the specified namespace

### 3. `search_namespaces` 🆕

- **Description**: Search and filter namespaces by keyword (much more practical than listing all)
- **Parameters**:
  - `query` (string, optional): Search keyword to filter namespaces
- **Returns**: Filtered list of namespaces matching the search query
- **Example**: Search for "bili" returns "bilibili", "sustainabilitymag"

### 4. `get_radar_rules`

- **Description**: Get all radar rules for automatic feed detection
- **Parameters**: None
- **Returns**: Complete radar rules database for feed discovery

### 5. `get_radar_rule`

- **Description**: Get radar rule for a specific domain
- **Parameters**:
  - `rule_name` (string): The domain/rule name (e.g., "github.com", "youtube.com")
- **Returns**: Radar configuration for the specified domain

### 6. `get_categories`

- **Description**: Get all available content categories
- **Parameters**: None
- **Returns**: Complete list of content categories

### 7. `get_category`

- **Description**: Get feeds for a specific category
- **Parameters**:
  - `category` (string): The category identifier (e.g., "programming", "news")
- **Returns**: All feeds within the specified category

### Content Retrieval Tool

### 8. `get_feed` 🆕

- **Description**: **Fetch actual RSS content from RSSHub paths (most important feature)**
- **Parameters**:
  - `path` (string): The RSSHub path (e.g., "bilibili/user/video/2267573", "github/issue/DIYgod/RSSHub")
- **Returns**: Actual RSS feed content including title, description, and feed items
- **Note**: This enables complete RSS workflow - from discovery to content retrieval

## Installation and Usage

### Building from Source

```bash
# Build the server
cargo build --release -p rsshub-mcp

# Or build in development mode
cargo build -p rsshub-mcp
```

### Running the Server

```bash
# Run with default configuration
cargo run -p rsshub-mcp --bin rsshub-mcp

# Run with custom config file
cargo run -p rsshub-mcp --bin rsshub-mcp -- --config custom-config.toml

# Check version
cargo run -p rsshub-mcp --bin rsshub-mcp -- --version
```

### Command Line Options

```text
Usage: rsshub-mcp [OPTIONS]

Options:
  -c, --config <CONFIG>  Configuration file path [default: config.toml]
  -v, --version         Show version information
  -h, --help            Print help
```

## Configuration

The server uses a TOML configuration file (`config.toml` by default):

```toml
# Server configuration
sse_server_addr = "127.0.0.1:8000"

# RSSHub API configuration
[rsshub]
host = "https://rsshub.akjong.com"
timeout = 120

# Logging configuration
[logging]
level = "info"
```

### Configuration Options

- **`sse_server_addr`**: Server bind address and port (default: "127.0.0.1:8000")
- **`rsshub.host`**: RSSHub instance URL
- **`rsshub.timeout`**: Request timeout in seconds
- **`logging.level`**: Log level (trace, debug, info, warn, error)

## MCP Client Integration

### Example Client Usage

```javascript
// Example using MCP SDK
const client = new MCPClient({
    serverEndpoint: "http://127.0.0.1:8000/mcp"
});

// Initialize connection
await client.initialize();

// Use discovery tools
const namespaces = await client.callTool("get_all_namespaces", {});
const bilibiliRoutes = await client.callTool("get_namespace", { 
    namespace: "bilibili" 
});
const programmingFeeds = await client.callTool("get_category", { 
    category: "programming" 
});

// Use new search functionality
const searchResults = await client.callTool("search_namespaces", { 
    query: "bili" 
});

// Fetch actual RSS content
const rssFeed = await client.callTool("get_feed", { 
    path: "bilibili/user/video/2267573" 
});
```

### HTTP API Endpoint

The server exposes the MCP protocol over HTTP at:

- **Endpoint**: `http://{host}:{port}/mcp`
- **Method**: POST
- **Content-Type**: `application/json`
- **Headers**: `mcp-session-id` (required for session management)

## Architecture

```text
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   MCP Client    │───▶│  RSSHub MCP     │───▶│   RSSHub API    │
│                 │    │    Server       │    │                 │
│ (AI Model/App)  │◀───│                 │◀───│  (rsshub-api)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Components

1. **Main Server** (`main.rs`): Entry point and server initialization
2. **Service Layer** (`service.rs`): MCP tool implementations and business logic
3. **Configuration** (`config.rs`): Configuration management and CLI parsing
4. **Logging** (`log.rs`): Structured logging setup

## Recent Improvements

Based on analysis of reference implementations, the server now includes enhanced functionality:

### Key Enhancements

1. **RSS Content Retrieval** - The `get_feed` tool enables users to fetch actual RSS content, not just metadata discovery
2. **Smart Namespace Search** - The `search_namespaces` tool efficiently filters through 250+ namespaces using keywords
3. **Enhanced Data Structures** - Proper type definitions for RSS content handling
4. **Complete RSS Workflow** - From discovery → configuration → content retrieval

### Quality Advantages

- **Type Safety**: Full compile-time guarantees through Rust's type system
- **Performance**: Async/await with tokio for efficient I/O operations
- **Memory Safety**: No runtime memory errors possible
- **Error Handling**: Comprehensive error propagation with proper types
- **Modularity**: Clean separation of concerns

## Development

### Project Structure

```text
rsshub-mcp/
├── src/
│   ├── main.rs      # Server entry point
│   ├── service.rs   # MCP service implementation
│   ├── config.rs    # Configuration management
│   └── log.rs       # Logging configuration
├── build.rs         # Build script for version info
├── config.toml      # Default configuration
└── Cargo.toml       # Package configuration
```

### Dependencies

#### Runtime Dependencies

- **`rsshub-api`** - RSSHub API client (local dependency)
- **`ultrafast-mcp`** - High-performance MCP framework
- **`tokio`** - Async runtime
- **`clap`** - Command line argument parsing
- **`config`** - Configuration management
- **`tracing`** - Structured logging
- **`serde`** - Serialization

#### Build Dependencies

- **`shadow-rs`** - Build-time information embedding

### Testing

```bash
# Run unit tests
cargo test -p rsshub-mcp

# Run integration tests with mock server
cargo test -p rsshub-mcp --test integration
```

## Monitoring and Observability

### Logging

The server provides structured logging with configurable levels:

```bash
# Set log level via environment
RUST_LOG=debug cargo run -p rsshub-mcp

# Or configure in config.toml
[logging]
level = "debug"
```

### Health Checks

The server automatically logs startup information and connection status.

## Deployment

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p rsshub-mcp

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rsshub-mcp /usr/local/bin/
COPY config.toml /etc/rsshub-mcp/
EXPOSE 8000
CMD ["rsshub-mcp", "--config", "/etc/rsshub-mcp/config.toml"]
```

### Systemd Service

```ini
[Unit]
Description=RSSHub MCP Server
After=network.target

[Service]
Type=simple
User=rsshub-mcp
ExecStart=/usr/local/bin/rsshub-mcp --config /etc/rsshub-mcp/config.toml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Performance

- Built on `ultrafast-mcp` for optimal performance
- Async I/O throughout the stack
- Minimal memory footprint
- Configurable timeouts and connection pooling

## Security

- No authentication required (design for trusted environments)
- Input validation on all tool parameters
- Secure error handling without information leakage
- Configurable CORS support

## Troubleshooting

### Common Issues

1. **Connection Refused**: Check if server address/port is correct
2. **Timeout Errors**: Increase timeout in configuration
3. **Tool Errors**: Verify RSSHub instance is accessible

### Debug Mode

```bash
RUST_LOG=debug cargo run -p rsshub-mcp
```

## License

MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure:

- Code follows Rust conventions
- Tests pass (`cargo test`)
- Documentation is updated
- Commit messages are descriptive

## Links

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [RSSHub Documentation](https://rsshub.app/)
- [UltraFast MCP Framework](https://github.com/ezsystems/ultrafast-mcp)
- [GitHub Repository](https://github.com/akjong/rsshub-mcp)
