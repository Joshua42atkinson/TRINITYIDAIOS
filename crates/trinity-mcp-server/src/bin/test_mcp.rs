//! Test Trinity MCP Server

use anyhow::Result;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    // Test 1: Check implementation status
    println!("🧪 Testing Trinity MCP Server...");

    // Connect to server
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;

    let request = json!({
        "id": "test1",
        "method": "tools/call",
        "params": {
            "name": "check_implementation_status",
            "arguments": {
                "feature": "97B Model Integration"
            }
        }
    });

    socket.write_all(request.to_string().as_bytes()).await?;
    socket.write_all(b"\n").await?;

    let mut response = String::new();
    let mut buf = [0; 8192];
    let n = socket.read(&mut buf).await?;
    response.push_str(&String::from_utf8_lossy(&buf[..n]));

    println!(
        "Response: {}",
        serde_json::from_str::<serde_json::Value>(&response)?
    );

    // Test 2: Search documentation
    println!("\n2. Testing search_documentation...");
    let request = json!({
        "id": "test2",
        "method": "tools/call",
        "params": {
            "name": "search_documentation",
            "arguments": {
                "query": "97B model",
                "limit": 5
            }
        }
    });

    socket.write_all(request.to_string().as_bytes()).await?;
    socket.write_all(b"\n").await?;

    response.clear();
    let n = socket.read(&mut buf).await?;
    response.push_str(&String::from_utf8_lossy(&buf[..n]));

    let response_json: serde_json::Value = serde_json::from_str(&response)?;
    println!(
        "Found {} results",
        response_json["result"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0)
    );

    // Test 3: List tools
    println!("\n3. Testing tools/list...");
    let request = json!({
        "id": "test3",
        "method": "tools/list",
        "params": {}
    });

    socket.write_all(request.to_string().as_bytes()).await?;
    socket.write_all(b"\n").await?;

    response.clear();
    let n = socket.read(&mut buf).await?;
    response.push_str(&String::from_utf8_lossy(&buf[..n]));

    let response_json: serde_json::Value = serde_json::from_str(&response)?;
    if let Some(tools) = response_json["result"].as_array() {
        println!("Available tools:");
        for tool in tools {
            println!("  - {}", tool["name"]);
        }
    }

    println!("\n✅ MCP Server test complete!");
    Ok(())
}
