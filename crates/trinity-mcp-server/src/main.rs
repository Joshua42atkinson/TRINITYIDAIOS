//! Trinity MCP Server Main Entry Point

use anyhow::Result;
use std::env;
use tokio::net::TcpListener;
use trinity_mcp_server::{McpHandler, TrinityMcpServer};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing to stderr so it does not corrupt stdout JSON RPC streams
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    // Get database URL from environment or fallback to Trinity OS SQLite file
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///home/joshua/.trinity/trinity_memory.db".to_string());

    // Initialize MCP server
    let server = TrinityMcpServer::new(&database_url).await?;

    // Check TRINITY_MCP_MODE
    let mode = env::var("TRINITY_MCP_MODE").unwrap_or_else(|_| "tcp".to_string());

    if mode == "stdio" {
        tracing::info!("Trinity MCP Server starting in stdio mode");
        if let Err(e) = handle_stdio_connection(server.db_pool.clone()).await {
            tracing::error!("Stdio connection error: {}", e);
        }
        return Ok(());
    }

    // Start TCP listener for MCP protocol
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Trinity MCP Server listening on {}", addr);

    // Handle TCP connections
    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("New connection from {}", addr);

        // Clone db_pool for each connection
        let db_pool = server.db_pool.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, db_pool).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(socket: tokio::net::TcpStream, db_pool: sqlx::SqlitePool) -> Result<()> {
    use futures::{SinkExt, StreamExt};
    use tokio_util::codec::{Framed, LinesCodec};

    // Create server instance for this connection
    let server = TrinityMcpServer::new_from_pool(db_pool).await?;

    // Use LinesCodec for message framing (expecting newline delimiter)
    let mut framed = Framed::new(socket, LinesCodec::new());

    while let Some(result) = framed.next().await {
        let line = match result {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to read line: {}", e);
                break;
            }
        };

        tracing::debug!("Received: {} bytes", line.len());

        // Parse MCP request
        let mcp_request: trinity_mcp_server::McpRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                tracing::error!("Failed to parse JSON-RPC request: {}", e);
                continue;
            }
        };

        // Handle request
        match server.handle_request(mcp_request).await {
            Ok(response) => {
                if response.id.is_none() && response.error.is_none() && response.result.is_none() {
                    continue;
                }
                let response_json = serde_json::to_string(&response)?;
                framed.send(response_json).await?;
            }
            Err(e) => {
                tracing::error!("Error handling request: {}", e);
                let error_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32603,
                        "message": format!("Internal error: {}", e)
                    }
                });
                framed.send(serde_json::to_string(&error_response)?).await?;
            }
        }
    }

    Ok(())
}

async fn handle_stdio_connection(db_pool: sqlx::SqlitePool) -> Result<()> {
    use futures::{SinkExt, StreamExt};
    use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

    let server = TrinityMcpServer::new_from_pool(db_pool).await?;

    let mut stdin = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
    let mut stdout = FramedWrite::new(tokio::io::stdout(), LinesCodec::new());

    while let Some(result) = stdin.next().await {
        let line = match result {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to read line: {}", e);
                break;
            }
        };

        if line.trim().is_empty() { continue; }
        tracing::debug!("Received: {} bytes", line.len());

        let mcp_request: trinity_mcp_server::McpRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                tracing::error!("Failed to parse JSON-RPC request: {}", e);
                continue;
            }
        };

        match server.handle_request(mcp_request).await {
            Ok(response) => {
                if response.id.is_none() && response.error.is_none() && response.result.is_none() {
                    continue;
                }
                let response_json = serde_json::to_string(&response)?;
                stdout.send(response_json).await?;
            }
            Err(e) => {
                tracing::error!("Error handling request: {}", e);
                let error_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32603,
                        "message": format!("Internal error: {}", e)
                    }
                });
                stdout.send(serde_json::to_string(&error_response)?).await?;
            }
        }
    }

    Ok(())
}
