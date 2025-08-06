use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RSSHub MCP工具快速测试 ===\n");

    let client = reqwest::Client::new();
    let session_id = Uuid::new_v4().to_string();
    let url = "http://127.0.0.1:8000/mcp";

    // 1. 初始化
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

    println!("1. 初始化连接...");
    if response.status().is_success() {
        println!("   ✅ 初始化成功!");
    } else {
        println!("   ❌ 初始化失败: {}", response.status());
        return Ok(());
    }

    // 2. 测试工具: get_categories
    println!("\n2. 测试 get_categories...");
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
                "   ✅ 成功! 获取到分类数据，前100字符: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ 成功! 响应: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ 失败: {}", response.status());
    }

    // 3. 测试工具: get_namespace
    println!("\n3. 测试 get_namespace (bilibili)...");
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
                "   ✅ 成功! bilibili命名空间数据，前100字符: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ 成功! 响应: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ 失败: {}", response.status());
    }

    // 4. 测试工具: get_category
    println!("\n4. 测试 get_category (programming)...");
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
                "   ✅ 成功! programming分类数据，前100字符: {}",
                &content.chars().take(100).collect::<String>()
            );
        } else {
            println!(
                "   ✅ 成功! 响应: {}",
                serde_json::to_string_pretty(&result)?
            );
        }
    } else {
        println!("   ❌ 失败: {}", response.status());
    }

    println!("\n=== 快速测试完成 ===");
    Ok(())
}
