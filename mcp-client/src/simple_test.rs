use std::time::Duration;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 简单HTTP客户端测试RSSHub MCP服务器 ===");

    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8000";

    // 首先检查服务器是否运行
    println!("1. 检查服务器连接...");
    let response = client.get(base_url).send().await;
    match response {
        Ok(resp) => println!("   服务器响应状态: {}", resp.status()),
        Err(e) => {
            println!("   服务器连接失败: {e}");
            return Ok(());
        }
    }

    // 尝试MCP初始化
    println!("\n2. 尝试MCP初始化...");
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

    // 测试不同的端点
    let endpoints = vec!["/", "/mcp", "/message"];

    for endpoint in endpoints {
        println!("\n   测试端点: {base_url}{endpoint}");
        let response = client
            .post(format!("{base_url}{endpoint}"))
            .header("Content-Type", "application/json")
            .json(&init_payload)
            .send()
            .await;

        match response {
            Ok(resp) => {
                println!("     状态: {}", resp.status());
                let headers: Vec<String> = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| format!("{k}: {v:?}"))
                    .collect();
                println!("     响应头: {headers:?}");

                if resp.status().is_success() {
                    match resp.text().await {
                        Ok(body) => println!("     响应体: {body}"),
                        Err(e) => println!("     读取响应体失败: {e}"),
                    }
                } else {
                    match resp.text().await {
                        Ok(body) => println!("     错误响应: {body}"),
                        Err(_) => println!("     无响应体"),
                    }
                }
            }
            Err(e) => println!("     请求失败: {e}"),
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
