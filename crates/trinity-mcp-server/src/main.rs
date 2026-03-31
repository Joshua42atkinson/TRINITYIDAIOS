//! Trinity MCP Server Main Entry Point

use anyhow::Result;
use std::env;
use tokio::net::TcpListener;
use trinity_mcp_server::{McpHandler, TrinityMcpServer};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/trinity".to_string());

    // Initialize MCP server
    let server = TrinityMcpServer::new(&database_url).await?;

    // Start TCP listener for MCP protocol
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Trinity MCP Server listening on {}", addr);

    // Handle connections
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
        let request: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to parse JSON: {}", e);
                continue;
            }
        };

        let mcp_request = trinity_mcp_server::McpRequest {
            id: request["id"].as_str().unwrap_or("unknown").to_string(),
            method: request["method"].as_str().unwrap_or("").to_string(),
            params: request["params"].clone(),
        };

        // Handle request
        match server.handle_request(mcp_request).await {
            Ok(response) => {
                let response_json = serde_json::to_string(&response)?;
                framed.send(response_json).await?;
            }
            Err(e) => {
                tracing::error!("Error handling request: {}", e);
                let error_response = serde_json::json!({
                    "id": request["id"],
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
