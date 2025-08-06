# RSSHub MCP Codebase Instructions

## Architecture Overview

This is a **3-crate Cargo workspace** implementing a Model Context Protocol (MCP) server for RSSHub:

- **`rsshub-api/`** - Pure HTTP client library for RSSHub APIs (6 core methods)
- **`rsshub-mcp/`** - MCP server wrapping rsshub-api with ultrafast-mcp framework
- **`mcp-client/`** - Testing clients (`quick_test`, `simple_test`) for validation

**Key Pattern**: The MCP server (`rsshub-mcp/src/service.rs`) implements `ToolHandler` trait, exposing 6 tools that directly map to `rsshub-api` methods. Each tool follows the same pattern: extract parameters → call API client → format response as debug string.

## Essential Commands

```bash
# Development workflow
just lint          # TOML format + cargo fmt + clippy + machete (dependency audit)
just format         # Auto-format all code
just test          # Run all tests
cargo build --workspace  # Build all crates

# Testing the MCP server
cargo run -p rsshub-mcp --bin rsshub-mcp                    # Start server (127.0.0.1:8000)
cargo run -p mcp-client --bin quick_test                    # Test all 6 tools
cargo run -p mcp-client --bin simple_test                   # Basic connection test
```

## Project-Specific Patterns

### 1. MCP Tool Implementation Pattern
Each tool in `service.rs` follows this exact structure:
```rust
// In list_tools(): Define tool schema with JSON schema
Tool {
    name: "get_namespace".to_string(),
    input_schema: json!({
        "properties": {"namespace": {"type": "string"}},
        "required": ["namespace"]
    }),
    // ...
}

// In handle_tool_call(): Extract params → call API → format response
"get_namespace" => {
    let namespace = request.arguments.get("namespace")?.as_str()?;
    self.handle_get_namespace(namespace).await
}

// Private handler method: Direct API call + debug formatting
async fn handle_get_namespace(&self, namespace: &str) -> Result<String> {
    let routes = self.client.get_namespace(namespace).await?;
    Ok(format!("{routes:#?}"))  // Always use debug formatting for responses
}
```

### 2. Workspace Dependency Management
All dependencies are centralized in root `Cargo.toml` under `[workspace.dependencies]`. Individual crates reference them as:
```toml
[dependencies]
reqwest = { workspace = true }
serde = { workspace = true }
```

### 3. Testing Strategy
- **Unit tests**: In `rsshub-api/src/lib.rs` using mockito for HTTP mocking
- **Integration tests**: Live clients in `mcp-client/` that test the full MCP protocol
- **Mock data**: JSON fixtures in `rsshub-api/tests/` for consistent testing

### 4. Error Handling Convention
- **`rsshub-api`**: Uses `eyre::Result` for all public methods, converts HTTP errors to eyre
- **`rsshub-mcp`**: Converts all errors to MCP format with `is_error: true` in responses
- **Never panic**: Always use `.expect()` with descriptive messages instead of `.unwrap()`

## Critical Integration Points

### MCP Protocol Flow
1. Client sends JSON-RPC 2.0 to `http://127.0.0.1:8000/mcp`
2. `ultrafast-mcp` handles protocol, calls `RSSHubService::handle_tool_call()`
3. Service extracts parameters, calls `rsshub-api` methods
4. API client makes HTTP calls to RSSHub instance (default: `https://rsshub.akjong.com`)
5. Response formatted as debug string and returned via MCP

### Configuration
- **MCP server**: `rsshub-mcp/config.toml` sets bind address (`sse_server_addr`)
- **API client**: Configurable via `RsshubClientConfig` (host, timeout)
- **Build info**: Uses `shadow-rs` to embed version info at compile time

## Development Guidelines

- **Linting is strict**: `cargo clippy` fails on `unwrap_used`, `uninlined_format_args`
- **Format strings**: Always use `format!("{variable}")` not `format!("{}", variable)`
- **Dependencies**: Run `cargo machete` to detect unused deps (part of `just lint`)
- **Async patterns**: Everything is async, use `tokio::main` for binaries
- **MCP session management**: Test clients use UUID session IDs in headers

## Key Files for Understanding

- `rsshub-mcp/src/service.rs` - Core MCP tool implementations and patterns
- `rsshub-api/src/lib.rs` - HTTP client patterns and type definitions  
- `mcp-client/src/quick_test.rs` - Real-world MCP protocol usage example
- `Justfile` - Complete CI/lint pipeline
- Root `Cargo.toml` - Workspace dependency strategy
