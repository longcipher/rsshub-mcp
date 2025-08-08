use std::env;

use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        eprintln!("Usage: generic <tool_name> [json_arguments] [--url http://host:port/mcp]");
        eprintln!("Example: generic get_radar_rule '{{\"domain\":\"github.com\"}}'");
        return Ok(());
    }

    let tool_name = args.remove(0);
    let mut json_args: Value = Value::Object(serde_json::Map::new());
    let mut url =
        env::var("RSSHUB_MCP_URL").unwrap_or_else(|_| "http://127.0.0.1:8000/mcp".to_string());

    if let Some(first) = args.first() {
        if !first.starts_with("--url") {
            match serde_json::from_str::<Value>(first) {
                Ok(v) => json_args = v,
                Err(_) => {
                    eprintln!("Invalid JSON for arguments, using empty object");
                }
            }
        }
    }
    // parse optional --url
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--url" {
            if let Some(v) = args.get(i + 1) {
                url = v.clone();
            }
            break;
        }
        i += 1;
    }

    let client = reqwest::Client::new();
    let session_id = Uuid::new_v4().to_string();

    // Initialize
    let init_request = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {
            "capabilities": {},
            "clientInfo": {"name": "generic-client", "version": "1.0.0"},
            "protocolVersion": "2024-11-05"
        },
        "id": "init-1"
    });
    let resp = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&init_request)
        .send()
        .await?;
    if !resp.status().is_success() {
        eprintln!("Initialization failed: {}", resp.status());
        return Ok(());
    }

    // Call tool
    let call = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {"name": tool_name, "arguments": json_args},
        "id": "call-1"
    });
    let resp = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&call)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    println!("Status: {status}\n{body}");

    Ok(())
}
