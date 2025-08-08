use std::env;

use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RSSHub MCP Tool Quick Test ===\n");

    let client = reqwest::Client::new();
    let session_id = Uuid::new_v4().to_string();
    let url =
        env::var("RSSHUB_MCP_URL").unwrap_or_else(|_| "http://127.0.0.1:8000/mcp".to_string());

    // 1. Initialize connection
    let init_request = json!({
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
    });

    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&init_request)
        .send()
        .await?;

    println!("1. Initializing connection...");
    if response.status().is_success() {
        println!("   ✅ Initialization successful!");
    } else {
        println!("   ❌ Initialization failed: {}", response.status());
        return Ok(());
    }

    // 2. List tools
    println!("\n2. Listing tools (tools/list)...");
    let list_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": "list-1"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&list_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Tools available: {}",
            result
                .get("result")
                .and_then(|r| r.get("tools"))
                .and_then(|t| t.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0)
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 3. Test tool: get_categories
    println!("\n3. Testing get_categories...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_categories",
            "arguments": {}
        },
        "id": "tool-categories"
    });

    let response = client
        .post(&url)
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
            println!(
                "   ✅ Success! Retrieved category data, first 100 chars: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ Success! Response: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 4. Test tool: get_namespace
    println!("\n4. Testing get_namespace (bilibili)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_namespace",
            "arguments": {
                "namespace": "bilibili"
            }
        },
        "id": "tool-namespace"
    });

    let response = client
        .post(&url)
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
            println!(
                "   ✅ Success! bilibili namespace data, first 100 chars: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ Success! Response: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 5. Test tool: get_category
    println!("\n5. Testing get_category (programming, format=json)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_category",
            "arguments": {
                "category": "programming",
                "format": "json"
            }
        },
        "id": "tool-category"
    });

    let response = client
        .post(&url)
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
            println!(
                "   ✅ Success! programming category data, first 100 chars: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ Success! Response: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 6. Test new feature: search_namespaces
    println!("\n6. Testing search_namespaces (searching for 'bili')...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "search_namespaces",
            "arguments": {
                "query": "bili"
            }
        },
        "id": "tool-search"
    });

    let response = client
        .post(&url)
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
            println!("   ✅ Success! Search results: {content}");
        } else {
            println!(
                "   ✅ Success! Response: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 7. Test new feature: get_feed (retrieve actual RSS content)
    println!("\n7. Testing get_feed (retrieving RSS content, format=json)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_feed",
            "arguments": {
                "path": "ithome/news",
                "format": "json"
            }
        },
        "id": "tool-feed"
    });

    let response = client
        .post(&url)
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
            println!(
                "   ✅ Success! RSS content, first 200 chars: {}",
                &content.chars().take(200).collect::<String>()
            );
        } else {
            println!(
                "   ✅ Success! Response: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 8. Test tool: get_all_namespaces
    println!("\n8. Testing get_all_namespaces...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_all_namespaces",
            "arguments": {}
        },
        "id": "tool-all-ns"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! Received namespaces response length: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 9. Test tool: get_radar_rules
    println!("\n9. Testing get_radar_rules (format=json)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_radar_rules",
            "arguments": {"format": "json"}
        },
        "id": "tool-radar-rules"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! Radar rules response length: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 10. Test tool: get_radar_rule (github.com)
    println!("\n10. Testing get_radar_rule (github.com, format=text)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_radar_rule",
            "arguments": {"rule_name": "github.com", "format": "text"}
        },
        "id": "tool-radar-rule"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! get_radar_rule result size: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 11. Test tool: search_routes
    println!("\n11. Testing search_routes (query='live', namespace='bilibili')...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "search_routes",
            "arguments": {"query": "live", "namespace": "bilibili", "limit": 5}
        },
        "id": "tool-search-routes"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! search_routes result size: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 12. Test tool: get_route_detail
    println!("\n12. Testing get_route_detail (bilibili, /live/room/:roomID)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_route_detail",
            "arguments": {"namespace": "bilibili", "route_key": "/live/room/:roomID"}
        },
        "id": "tool-route-detail"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! get_route_detail result size: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    // 13. Test tool: suggest_route_keys
    println!("\n13. Testing suggest_route_keys (namespace='bilibili', partial='live/ro')...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "suggest_route_keys",
            "arguments": {"namespace": "bilibili", "partial": "live/ro", "limit": 5}
        },
        "id": "tool-suggest-keys"
    });
    let response = client
        .post(&url)
        .header("mcp-session-id", &session_id)
        .header("Content-Type", "application/json")
        .json(&tool_request)
        .send()
        .await?;
    if response.status().is_success() {
        let result: Value = response.json().await?;
        println!(
            "   ✅ Success! suggest_route_keys result size: {}",
            serde_json::to_string(&result)?.len()
        );
    } else {
        println!("   ❌ Failed: {}", response.status());
    }

    println!("\n=== Quick test completed ===");
    Ok(())
}
