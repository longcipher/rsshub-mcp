use std::time::Duration;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple HTTP Client Test for RSSHub MCP Server ===");

    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8000";

    // First check if the server is running
    println!("1. Checking server connection...");
    let response = client.get(base_url).send().await;
    match response {
        Ok(resp) => println!("   Server response status: {}", resp.status()),
        Err(e) => {
            println!("   Server connection failed: {e}");
            return Ok(());
        }
    }

    // Try MCP initialization
    println!("\n2. Attempting MCP initialization...");
    let init_payload = json!({
        "jsonrpc": "2.0",
        "id": "init-1",
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "simple-test-client",
                "version": "1.0.0"
            }
        }
    });

    // Test different endpoints
    let endpoints = vec!["/", "/mcp", "/message"];

    for endpoint in endpoints {
        println!("\n   Testing endpoint: {base_url}{endpoint}");
        let response = client
            .post(format!("{base_url}{endpoint}"))
            .header("Content-Type", "application/json")
            .json(&init_payload)
            .send()
            .await;

        match response {
            Ok(resp) => {
                println!("     Status: {}", resp.status());
                let headers: Vec<String> = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| format!("{k}: {v:?}"))
                    .collect();
                println!("     Response headers: {headers:?}");

                if resp.status().is_success() {
                    match resp.text().await {
                        Ok(body) => println!("     Response body: {body}"),
                        Err(e) => println!("     Failed to read response body: {e}"),
                    }
                } else {
                    match resp.text().await {
                        Ok(body) => println!("     Error response: {body}"),
                        Err(_) => println!("     No response body"),
                    }
                }
            }
            Err(e) => println!("     Request failed: {e}"),
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
