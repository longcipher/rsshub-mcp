# RSSHub MCP

A Model Context Protocol (MCP) server for RSSHub, providing structured access to RSS feeds and content through the MCP protocol.

## Overview

This project is a complete RSSHub MCP solution implemented in Rust with a multi-crate workspace structure. It provides not only metadata discovery but also **actual RSS content retrieval**, making it a full-featured RSS workflow tool.

### Key Features

- ✅ **Complete RSS Workflow**: Discover feeds → Configure parameters → Retrieve actual RSS content
- ✅ **Smart Search**: Filter through 250+ namespaces efficiently with keyword search
- ✅ **Type Safety**: Full compile-time guarantees through Rust's type system
- ✅ **High Performance**: Async/await with tokio for efficient I/O operations
- ✅ **Robust Architecture**: Clean separation of concerns across 3 crates
- ✅ **Comprehensive Testing**: Independent test clients for validation

## Project Structure

```text
rsshub-mcp/
├── Cargo.toml          # Workspace configuration
├── rsshub-api/         # RSSHub API client library
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs      # Core API client implementation
├── rsshub-mcp/         # MCP server implementation
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs     # Server entry point
│       ├── service.rs  # MCP service implementation
│       ├── config.rs   # Configuration management
│       └── log.rs      # Logging configuration
└── mcp-client/         # Testing client
    ├── Cargo.toml
    └── src/
        ├── quick_test.rs   # Quick functionality test
        └── simple_test.rs  # Basic connection test
```

## Crates Description

### 1. rsshub-api (v0.1.0)

- **Purpose**: Standalone RSSHub API client library
- **Features**:
  - Complete implementation of core RSSHub API methods
  - RSS content retrieval functionality
  - Error handling using eyre
  - Async operation support
  - Can be used as an independent library

### 2. rsshub-mcp (v0.1.0)

- **Purpose**: MCP (Model Context Protocol) server
- **Features**:
  - Built on ultrafast-mcp framework
  - Depends on rsshub-api library
  - Provides 10 MCP tools (including RSS content retrieval)
  - HTTP transport protocol support

### 3. mcp-client (v0.1.0)

- **Purpose**: Testing and validation tools
- **Features**:
  - Quick functionality testing (quick_test)
  - Basic connection testing (simple_test)
  - Used for development and debugging

## Core Functionality

### MCP Tools

The server exposes tools for RSSHub interaction via MCP:

#### Core Discovery Tools

1. `get_all_namespaces` - Get all available namespaces
2. `get_namespace` - Get routes for a specific namespace
3. **`search_namespaces`** 🆕 - **Search namespaces by keyword (much more practical than listing all)**
4. `get_radar_rules` - Get all radar rules for automatic feed detection
5. `get_radar_rule` - Get a specific radar rule by name
6. `get_categories` - List known categories (informational; static guidance)
7. `get_category` - Get feeds for a specific category

8. **`search_routes`** 🆕 - Search routes by keyword across all namespaces or within a namespace (supports text/json output)
9. **`get_route_detail`** 🆕 - Get detailed information for a specific route within a namespace (supports text/json output)
10. **`suggest_route_keys`** 🆕 - Fuzzy-suggest closest route keys within a namespace for a partial path

#### Content Retrieval Tool

 1. **`get_feed`** 🆕 - **Fetch actual RSS content from RSSHub paths**


### Usage Examples

#### Get RSS Content

```json
{
  "tool": "get_feed",
  "arguments": {
  "path": "bilibili/user/video/2267573",
  "format": "json"
  }
}
```

#### Search Namespaces

```json
{
  "tool": "search_namespaces", 
  "arguments": {
    "query": "bili"
  }
}
```

#### List Tools (MCP protocol)

```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": "list-1"
}
```

#### Search Routes

```json
{
  "tool": "search_routes",
  "arguments": {
    "query": "live",
    "namespace": "bilibili",
    "limit": 10,
    "format": "json"
  }
}
```

#### Get Route Detail

```json
{
  "tool": "get_route_detail",
  "arguments": {
    "namespace": "bilibili",
    "route_key": "/live/room/:roomID",
    "format": "text"
  }
}
```

#### Get Radar Rules (JSON)

```json
{
  "tool": "get_radar_rules",
  "arguments": {
    "format": "json"
  }
}
```

#### Get A Radar Rule (text)

```json
{
  "tool": "get_radar_rule",
  "arguments": {
    "rule_name": "github.com",
    "format": "text"
  }
}
```

#### Get Category (JSON)

```json
{
  "tool": "get_category",
  "arguments": {
    "category": "programming",
    "format": "json"
  }
}
```

#### Suggest Route Keys

```json
{
  "tool": "suggest_route_keys",
  "arguments": {
    "namespace": "bilibili",
    "partial": "live/ro",
    "limit": 5
  }
}
```

## Implementation Highlights

### Recent Improvements

Based on analysis of reference projects (`reonokiy/rsshub-mcp` and `RechardLLee/RSSHUB-MCP`), we've implemented the most valuable missing features:

1. **RSS Content Retrieval** - Users can now get actual RSS feed content, not just metadata
2. **Intelligent Search** - Efficiently filter through hundreds of namespaces
3. **Enhanced Data Structures** - Proper types for RSS content handling
4. **Superior Architecture** - Rust's advantages over Python implementations

### Quality Advantages

- **Type Safety**: Full compile-time guarantees through Rust's type system
- **Performance**: Async/await with tokio for efficient I/O
- **Memory Safety**: No runtime memory errors possible
- **Error Handling**: Comprehensive error propagation with proper types
- **Modularity**: Clean separation of concerns across crates

### Development & Testing

#### Development Workflow

```bash
# Format and lint all code
just lint

# Build all crates
cargo build --workspace

# Run all tests
just test
```

#### Testing the Implementation

```bash
# Start the MCP server
cargo run -p rsshub-mcp --bin rsshub-mcp

# Test all 10 tools (including new RSS content retrieval)
cargo run -p mcp-client --bin quick_test

# Note: start the server from the crate directory so it finds config.toml
# (cd rsshub-mcp && cargo run -p rsshub-mcp --bin rsshub-mcp)

# Basic connection test
cargo run -p mcp-client --bin simple_test
```

### Validation Status

Note: See quick_test for an end-to-end exercise of each tool. Build and run locally to validate in your environment.

## Advantages

1. **Complete RSS Workflow**: Unlike reference projects that only provide discovery, this implementation enables full RSS content retrieval
2. **Superior Architecture**: 3-crate workspace design with better modularity than Python alternatives
3. **Type Safety**: Rust's type system prevents runtime errors common in dynamic languages
4. **High Performance**: Async/await with tokio provides better I/O efficiency
5. **Comprehensive Testing**: Independent test client facilitates debugging and validation
6. **Code Quality**: Strict linting, formatting, and dependency management

## Future Enhancements

### High Priority

- **Enhanced RSS Parsing**: Parse RSS content into structured data instead of raw text
- **Caching Mechanism**: Add TTL caching for metadata (namespaces, radar rules)
- **Content Processing**: HTML cleaning, timezone normalization

### Medium Priority

- **URL Format Support**: Support multiple URL formats (rsshub://, standard URL, short paths)
- **Retry Logic**: Add simple retry mechanism for failed requests
- **Better Error Messages**: Provide more user-friendly error responses

### Low Priority

- **Resource Handler**: Waiting for MCP framework support
- **AI Assistant Features**: Prompt handlers for enhanced functionality

## Project Status

Note: Verified locally during development. Re-run quick_test after changes.

### Dependency Management

Uses workspace-level dependency management with shared versions defined in the root `Cargo.toml`:

```toml
[workspace.dependencies]
ultrafast-mcp = "202506018.1.0"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
# ... other dependencies
```

## API Documentation

- <https://rsshub.app/api/reference>

## References

- <https://github.com/glidea/zenfeed>
- <https://github.com/RechardLLee/RSSHUB-MCP>
