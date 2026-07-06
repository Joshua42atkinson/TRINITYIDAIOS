---
description: Phase 5 — Security hardening: auth on creative endpoints, CORS lockdown, graceful shutdown, constant-time API key comparison.
---

# Phase 5: Security Hardening

## Objective

Close the security gaps identified during the bug-fix session.

## Prerequisites

- Phase 3 complete (main.rs split — handlers accessible)
- `cargo check` passes

## Issues to Fix

### 5.0a: CRITICAL — Unauthenticated Tool Execution
**Problem**: `/api/tools/execute` allows anyone to run arbitrary tools (file read/write, shell commands) without auth. This is the most critical finding for public deployment.

**Fix**: Add `/api/tools` to `PROTECTED_PREFIXES` in `auth.rs`. All tool execution must require API key when `TRINITY_API_KEY` is set.

### 5.0b: CRITICAL — Model URL Exfiltration
**Problem**: `/api/models/switch` accepts arbitrary URLs via `InferenceRouter::set_active_url`. An attacker can point inference at their own backend, exfiltrating all prompts and responses.

**Fix**: Either:
- Remove the endpoint entirely (model URL should be config-only, not runtime-changeable), OR
- Require auth AND validate URL against an allowlist of known model hosts (localhost, 127.0.0.1, configured Tailscale IP)

### 5.1: Creative Endpoint Auth Bypass
**Problem**: Creative endpoints (`/api/creative/image`, `/api/creative/tempo`, `/api/creative/video`, `/api/creative/mesh3d`) are not in `PROTECTED_PREFIXES` in `auth.rs`. Anyone can trigger GPU-heavy jobs without auth.

**Fix**: Add `/api/creative` to `PROTECTED_PREFIXES` in `auth.rs`, or add a separate `CREATIVE_PREFIXES` list that requires auth when `TRINITY_API_KEY` is set.

### 5.2: CORS Allows Any Origin
**Problem**: `CorsLayer::new().allow_origin(tower_http::cors::Any)` allows any origin.

**Fix**: Restrict to known origins:
```rust
let origins = [
    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
    "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
];
// Add Tailscale IP if configured
if let Ok(tailscale_ip) = std::env::var("TRINITY_TAILSCALE_IP") {
    // Add tailscale origin
}
CorsLayer::new()
    .allow_origin(origins)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
```

### 5.3: No Graceful Shutdown
**Problem**: Server has no signal handling. In-flight requests are killed on SIGTERM/SIGINT.

**Fix**: Add graceful shutdown to the axum server:
```rust
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, draining connections...");
}

// In main():
axum::serve(
    listener,
    app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
)
.graceful_shutdown(shutdown_signal())
.await?;
```

### 5.4: API Key Comparison Not Constant-Time
**Problem**: `provided == expected` in `auth.rs` is vulnerable to timing attacks.

**Fix**: Use `constant_time_eq` or manual constant-time comparison:
```rust
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}
```

### 5.5: No Request Body Size Limits
**Problem**: No explicit body size limits per route. Default is 2MB but some endpoints need more.

**Fix**: Add `DefaultBodyLimit` per route group:
```rust
use axum::extract::DefaultBodyLimit;

// Chat: 1MB max
.route("/api/chat", post(handlers::chat::yardmaster).layer(DefaultBodyLimit::max(1024 * 1024)))

// Ingest (RAG upload): 50MB max
.route("/api/rag/ingest", post(handlers::rag::ingest).layer(DefaultBodyLimit::max(50 * 1024 * 1024)))
```

## Steps

1. Fix creative endpoint auth (5.1)
2. Fix CORS (5.2)
3. Add graceful shutdown (5.3)
4. Fix API key comparison (5.4)
5. Add body size limits (5.5)
6. Test each fix

## Testing

```bash
# Build
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5

# Test auth on creative endpoint (should 401 without API key)
curl -s -o /dev/null -w "%{http_code}" -X POST http://localhost:3000/api/creative/image -H "Content-Type: application/json" -d '{"prompt":"test"}'

# Test CORS (should not have Access-Control-Allow-Origin: *)
curl -s -I http://localhost:3000/api/health | grep -i "access-control"
```

## Completion Criteria

- Creative endpoints require auth when `TRINITY_API_KEY` is set
- CORS restricted to localhost + configured origins
- Server drains connections on SIGTERM/SIGINT
- API key comparison is constant-time
- Body size limits set per route group
