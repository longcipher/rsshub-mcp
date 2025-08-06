use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RSSHub MCP Tool Quick Test ===\n");

    let client = reqwest::Client::new();
    let session_id = Uuid::new_v4().to_string();
    let url = "http://127.0.0.1:8000/mcp";

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
        .post(url)
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

    // 2. Test tool: get_categories
    println!("\n2. Testing get_categories...");
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
        .post(url)
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

    // 3. Test tool: get_namespace
    println!("\n3. Testing get_namespace (bilibili)...");
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
        .post(url)
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

    // 4. Test tool: get_category
    println!("\n4. Testing get_category (programming)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_category",
            "arguments": {
                "category": "programming"
            }
        },
        "id": "tool-category"
    });

    let response = client
        .post(url)
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

    // 5. Test new feature: search_namespaces
    println!("\n5. Testing search_namespaces (searching for 'bili')...");
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
        .post(url)
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

    // 6. Test new feature: get_feed (retrieve actual RSS content)
    println!("\n6. Testing get_feed (retrieving RSS content)...");
    let tool_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "get_feed",
            "arguments": {
                "path": "ithome/news"
            }
        },
        "id": "tool-feed"
    });

    let response = client
        .post(url)
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

    println!("\n=== Quick test completed ===");
    Ok(())
}
