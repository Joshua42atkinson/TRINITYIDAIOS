use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run --example mcp_client <method> <params_json>");
        std::process::exit(1);
    }

    let method = &args[1];
    let mut params: Value = serde_json::from_str(&args[2])?;

    // Special handling for index_document to read from file if needed
    if method == "tools/call" && params["name"] == "index_document" {
        if let Some(path) = params["arguments"]["path"].as_str() {
            if params["arguments"]["content"] == "@FILE" {
                let content = std::fs::read_to_string(path)?;
                params["arguments"]["content"] = json!(content);
            }
        }
    }

    let request = json!({
        "id": "1",
        "method": method,
        "params": params
    });

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut request_str = request.to_string();
    request_str.push('\n');
    stream.write_all(request_str.as_bytes())?;

    let mut response_data = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        response_data.extend_from_slice(&buffer[..n]);
        if response_data.contains(&b'\n') {
            break;
        }
    }

    let response: Value = serde_json::from_slice(&response_data)?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
