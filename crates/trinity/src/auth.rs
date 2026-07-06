use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Get the API key from environment variable.
/// If not set, auth is disabled (local dev mode — safe behind NAT/firewall).
fn expected_api_key() -> Option<String> {
    std::env::var("TRINITY_API_KEY").ok().filter(|s| !s.is_empty())
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

/// Helper to validate if the URL is pointing to a local host or Tailscale IP.
pub fn is_url_allowed_for_inference(url: &str) -> bool {
    let parsed = match reqwest::Url::parse(url) {
        Ok(u) => u,
        Err(_) => return false,
    };
    let host = match parsed.host_str() {
        Some(h) => h,
        None => return false,
    };
    
    if host == "localhost" || host == "127.0.0.1" || host == "[::1]" || host == "::1" {
        return true;
    }
    
    if let Ok(ts_ip) = std::env::var("TRINITY_TAILSCALE_IP") {
        if !ts_ip.is_empty() && host == ts_ip {
            return true;
        }
    }
    
    false
}

/// Paths that require authentication even when key is set.
/// Everything else (chat, health, RAG search, static files) stays public.
const PROTECTED_PREFIXES: &[&str] = &[
    "/api/tools",
    "/api/creative",
    "/api/models/switch",
    "/api/inference/switch",
    "/api/inference/start",
    "/api/inference/stop",
    "/api/inference/hotel/swap",
    "/api/inference/hotel/team",
    "/api/inference/hotel/solo",
    "/api/inference/hotel/close",
    "/api/inference/hotel/open",
    "/api/config/setup",
    "/api/reset/demo",
    "/api/daydream/command",
];

/// Middleware: require `Authorization: Bearer <key>` for protected endpoints.
/// - If TRINITY_API_KEY is not set: all endpoints open (local dev mode)
/// - If TRINITY_API_KEY is set: protected endpoints require Bearer token
/// - Public endpoints (chat, health, RAG) always pass through
pub async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path();

    // Check if this path is protected
    // Exception: /api/creative/assets/ is public to allow loading generated images in img tags
    let is_protected = PROTECTED_PREFIXES.iter().any(|p| path.starts_with(p))
        && !path.starts_with("/api/creative/assets/");
        
    if !is_protected {
        return Ok(next.run(req).await);
    }

    // If no key configured, allow access (local dev / behind NAT)
    let expected = match expected_api_key() {
        Some(k) => k,
        None => return Ok(next.run(req).await),
    };

    // Protected endpoint + key configured → require Bearer token
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header_val) if header_val.starts_with("Bearer ") => {
            let provided = &header_val[7..];
            if constant_time_compare(provided, &expected) {
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
// ═══ Rate Limiting ═══

/// Simple in-memory rate limiter — tracks requests per IP.
/// Sliding window: allows N requests per M seconds per IP.
struct RateLimiter {
    /// IP → list of request timestamps
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    /// Max requests per window
    max_requests: usize,
    /// Window duration
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    async fn check(&self, ip: &str) -> bool {
        let mut reqs = self.requests.lock().await;
        let now = Instant::now();
        let timestamps = reqs.entry(ip.to_string()).or_insert_with(Vec::new);

        // Remove expired entries
        timestamps.retain(|t| now.duration_since(*t) < self.window);

        if timestamps.len() >= self.max_requests {
            return false;
        }

        timestamps.push(now);
        true
    }

    /// Purge IPs with no recent requests to prevent unbounded memory growth
    async fn purge_expired(&self) {
        let mut reqs = self.requests.lock().await;
        let now = Instant::now();
        reqs.retain(|_, timestamps| {
            timestamps.retain(|t| now.duration_since(*t) < self.window);
            !timestamps.is_empty()
        });
    }
}

/// Shared rate limiters for different endpoint categories
static CHAT_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
static TOOL_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
static GLOBAL_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();

fn chat_limiter() -> &'static RateLimiter {
    CHAT_LIMITER.get_or_init(|| RateLimiter::new(15, 60)) // 15/min for chat
}
fn tool_limiter() -> &'static RateLimiter {
    TOOL_LIMITER.get_or_init(|| RateLimiter::new(10, 60)) // 10/min for tools
}
fn global_limiter() -> &'static RateLimiter {
    GLOBAL_LIMITER.get_or_init(|| RateLimiter::new(120, 60)) // 120/min global
}

/// Extract client IP from request safely.
/// Only trusts proxy headers when the request is verified to come from a known proxy:
/// - `cf-connecting-ip` only when `cf-ray` header is also present (Cloudflare verification)
/// - `x-forwarded-for` only when the connection is from localhost (reverse proxy on same host)
fn client_ip(req: &Request) -> String {
    let headers = req.headers();

    // Cloudflare: only trust cf-connecting-ip if cf-ray is also present
    // (cf-ray is injected by Cloudflare edge, not settable by clients)
    if headers.contains_key("cf-ray") {
        if let Some(ip) = headers.get("cf-connecting-ip").and_then(|v| v.to_str().ok()) {
            return ip.to_string();
        }
    }

    // Local reverse proxy: only trust x-forwarded-for from localhost connections
    // axum ConnectInfo extension provides the real remote address
    if let Some(conn_info) = req.extensions().get::<axum::extract::ConnectInfo<std::net::SocketAddr>>() {
        let remote = conn_info.0.ip();
        if remote.is_loopback() {
            if let Some(ip) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
                return ip.split(',').next().unwrap_or("unknown").trim().to_string();
            }
        }
        return remote.to_string();
    }

    "local".to_string()
}

/// Rate limiting middleware — applies per-endpoint limits
pub async fn rate_limit(req: Request, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let ip = client_ip(&req);

    // Periodic purge: clean up expired entries every 100 requests
    static PURGE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let count = PURGE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if count.is_multiple_of(100) {
        global_limiter().purge_expired().await;
        chat_limiter().purge_expired().await;
        tool_limiter().purge_expired().await;
    }

    // Global rate limit (all endpoints)
    if !global_limiter().check(&ip).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // Endpoint-specific limits
    if path.contains("/api/chat/") || path == "/api/chat" {
        if !chat_limiter().check(&ip).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    } else if (path.contains("/api/tools/") || path.contains("/api/inference/"))
        && !tool_limiter().check(&ip).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

    Ok(next.run(req).await)
}
