mod api;
mod config;
mod log;
mod service;

use clap::Parser;
use eyre::Result;
use rmcp::transport::SseServer;
use shadow_rs::shadow;
use tracing::info;

use crate::{
    config::{Cli, Config},
    log::init_log,
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

    // let sse_addr = config.sse_server_addr.parse()?;

    // info!("Starting MCP server at {}", sse_addr);

    // let ct = SseServer::serve(sse_addr)
    //     .await?
    //     .with_service(AptosWalletService::new);

    // info!("MCP server started successfully");

    // tokio::signal::ctrl_c().await?;
    // ct.cancel();

    info!("Server shutdown");

    Ok(())
}
