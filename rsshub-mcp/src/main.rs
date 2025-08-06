mod config;
mod log;
mod service;

use std::sync::Arc;

use clap::Parser;
use eyre::Result;
use shadow_rs::shadow;
use tracing::info;
use ultrafast_mcp::{ServerCapabilities, ServerInfo, ToolsCapability, UltraFastServer};

use crate::{
    config::{Cli, Config},
    log::init_log,
    service::RSSHubService,
};

shadow!(build);

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.version {
        println!("{}", build::VERSION);
        return Ok(());
    }
    init_log("info")?;
    let config = Config::new(cli.config)?;
    info!("{:?}", config);

    // Create server info
    let server_info = ServerInfo {
        name: "rsshub-mcp".to_string(),
        version: build::VERSION.to_string(),
        description: Some("RSSHub Model Context Protocol server".to_string()),
        authors: Some(vec!["akjong".to_string()]),
        homepage: Some("https://github.com/akjong/rsshub-mcp".to_string()),
        license: Some("MIT".to_string()),
        repository: Some("https://github.com/akjong/rsshub-mcp".to_string()),
    };

    // Create server capabilities
    let capabilities = ServerCapabilities {
        tools: Some(ToolsCapability {
            list_changed: Some(true),
        }),
        ..Default::default()
    };

    // Create and configure the server
    let rsshub_service = Arc::new(RSSHubService::new());
    let server =
        UltraFastServer::new(server_info, capabilities).with_tool_handler(rsshub_service.clone());

    info!("Starting RSSHub MCP server at {}", config.sse_server_addr);

    // Start the server with HTTP transport
    let addr: std::net::SocketAddr = config.sse_server_addr.parse()?;
    server
        .run_streamable_http(addr.ip().to_string().as_str(), addr.port())
        .await?;

    info!("RSSHub MCP server started successfully");

    Ok(())
}
