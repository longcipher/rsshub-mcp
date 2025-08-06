# RSSHub MCP

A Model Context Protocol (MCP) server for RSSHub, providing structured access to RSS feeds and content through the MCP protocol.

## Overview

This project has been successfully refactored into a multi-crate Cargo workspace structure, improving code organization and modularity. The workspace consists of three main crates that work together to provide a complete RSSHub MCP solution.

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
  - Complete implementation of 6 API methods
  - Error handling using eyre
  - Async operation support
  - Can be used as an independent library

### 2. rsshub-mcp (v0.1.0)

- **Purpose**: MCP (Model Context Protocol) server
- **Features**:
  - Built on ultrafast-mcp framework
  - Depends on rsshub-api library
  - Provides 6 MCP tools
  - HTTP transport protocol support

### 3. mcp-client (v0.1.0)

- **Purpose**: Testing and validation tools
- **Features**:
  - Quick functionality testing (quick_test)
  - Basic connection testing (simple_test)
  - Used for development and debugging

## Core Functionality

### MCP Tools

1. `get_all_namespaces` - Get all available namespaces
2. `get_namespace` - Get routes for a specific namespace
3. `get_radar_rules` - Get all radar rules for automatic feed detection
4. `get_radar_rule` - Get a specific radar rule by name
5. `get_categories` - Get all available categories
6. `get_category` - Get feeds for a specific category

## Build and Run

### Build the entire workspace

```bash
cargo build --workspace
```

### Check all crates

```bash
cargo check --workspace
```

### Run MCP server

```bash
cargo run -p rsshub-mcp --bin rsshub-mcp
```

### Run test clients

```bash
# Quick functionality test
cargo run -p mcp-client --bin quick_test

# Basic connection test
cargo run -p mcp-client --bin simple_test
```

## Dependency Management

Uses workspace-level dependency management with shared versions defined in the root `Cargo.toml`:

```toml
[workspace.dependencies]
ultrafast-mcp = "202506018.1.0"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
# ... other dependencies
```

## Validation Status

✅ **All crates compile successfully**  
✅ **MCP server starts normally**  
✅ **All 6 tools function correctly**  
✅ **Client tests pass**  
✅ **Workspace structure is correct**  

## Advantages

1. **Modularity**: Each crate has clear responsibilities, easy to maintain
2. **Reusability**: rsshub-api can be used as an independent library
3. **Unified Management**: Workspace manages dependency versions uniformly
4. **Better Testing**: Independent test client facilitates debugging
5. **Clear Structure**: Code organization is more clear and professional

## Next Steps

- Publish rsshub-api to crates.io
- Add independent documentation for each crate
- Add more integration tests
- Consider adding performance benchmarks

## API Documentation

- <https://rsshub.app/api/reference>

## References

- <https://github.com/glidea/zenfeed>
- <https://github.com/RechardLLee/RSSHUB-MCP>
