// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Edge Guard Middleware
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        edge_guard.rs
// PURPOSE:     Defense-in-depth route protection for public tunnel traffic
//
// Red Hat Findings Addressed:
//   C1: Block tool execution from tunnel
//   C2: Block model/inference switching from tunnel
//   C4: Block session/project access from tunnel
//   C6: Block shell/python paths from tunnel
//   H3: Rate limiting (basic per-IP)
//   H4: Block admin mutation from tunnel
//   H5: Block telemetry leaks from tunnel
//
// ARCHITECTURE:
//   Cloudflare Tunnel adds `Cf-Connecting-Ip` header to all proxied requests.
//   If that header is present, the request came from the internet (not localhost).
//   This middleware blocks dangerous routes for tunnel traffic.
//
//   This is a SECOND layer of defense. The primary layer is the Caddyfile.
//   Both must be bypassed for an attack to succeed.
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// ═══════════════════════════════════════════════════════════════════════════
/// PROTOTYPE MODE — Set to `true` for Purdue demo presentations.
/// When active, ALL routes are accessible from the tunnel (with rate limiting).
/// Set back to `false` for production hosting.
/// ═══════════════════════════════════════════════════════════════════════════
const PROTOTYPE_MODE: bool = true;

/// Blocked path prefixes for tunnel (non-local) traffic.
/// These routes are only accessible from localhost (direct browser access).
const BLOCKED_PREFIXES: &[&str] = &[
    "/api/tools",
    "/api/models/switch",
    "/api/inference/switch",
    "/api/inference/refresh",
    "/api/sessions",
    "/api/projects",
    "/api/chat",
    "/api/v1/trinity",
    "/api/mode",
    "/api/quest",
    "/api/character",
    "/api/pearl",
    "/api/bestiary",
    "/api/creative",
    "/api/voice",
    "/api/mcp",
    "/api/ground",
    "/api/intent",
    "/api/ingest",
    "/api/rag",
    "/api/eye",
    "/api/yard",
    "/api/journal",
    "/api/book",
    "/api/narrative",
    "/api/perspective",
    "/api/rlhf",
    "/api/tts",
    "/api/bevy",
    "/api/hardware",
    "/api/status",
    "/api/models",
];

/// Simple per-IP rate limiter state.
/// Tracks request counts per IP within a rolling window.
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if an IP is within rate limits.
    /// Returns true if the request should be allowed.
    fn check(&self, ip: &str, max_requests: usize, window_secs: u64) -> bool {
        let now = Instant::now();
        let mut map = self.requests.lock().unwrap();
        let entries = map.entry(ip.to_string()).or_default();

        // Remove expired entries
        entries.retain(|t| now.duration_since(*t).as_secs() < window_secs);

        if entries.len() >= max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}

/// Returns true if the request appears to come from the Cloudflare tunnel
/// (i.e., from the internet, not from localhost).
fn is_tunnel_request(req: &Request<Body>) -> bool {
    // Cloudflare adds these headers to all proxied requests
    req.headers().contains_key("cf-connecting-ip")
        || req.headers().contains_key("cf-ray")
        || req.headers().contains_key("cf-ipcountry")
}

/// Public-facing portfolio endpoints — exempted from the blocked list.
/// These are read-only, stateless, and rate-limited more aggressively.
const PORTFOLIO_ALLOWED: &[&str] = &[
    "/api/chat/portfolio",
];

/// Tier 2: Zen Mode demo — curated routes safe for external access.
/// These power the Zen Mode experience for remote demonstrations.
/// No tools, no shell, no model switching, no session/project access.
/// Rate limited at 30 req/min per IP.
const ZEN_DEMO_ALLOWED: &[&str] = &[
    "/api/chat/zen",        // Socratic conversation (SSE stream)
    "/api/quest/state",     // Read current quest phase
    "/api/quest/complete",  // Mark objective done (state mutation, rate-limited)
    "/api/character",       // Character Sheet read + update
    "/api/models/active",   // Which model is loaded (read-only)
    "/api/health",          // Health check
    "/api/tts",             // Voice narration
    "/api/pearl",           // PEARL state (read/refine)
    "/docs/",               // Documentation (Four Chariots + Hook Book)
];

/// Edge Guard middleware — blocks dangerous routes for tunnel traffic
/// and applies rate limiting to all tunnel requests.
pub async fn edge_guard(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Local requests pass through unrestricted
    if !is_tunnel_request(&req) {
        return Ok(next.run(req).await);
    }

    // ═══ PROTOTYPE MODE: All routes open, rate limiting only ═══
    if PROTOTYPE_MODE {
        if let Some(ip) = req
            .headers()
            .get("cf-connecting-ip")
            .and_then(|v| v.to_str().ok())
        {
            static PROTO_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
            let limiter = PROTO_LIMITER.get_or_init(RateLimiter::new);
            if !limiter.check(ip, 60, 60) {
                tracing::warn!("🛡️ Edge Guard [PROTOTYPE]: Rate limited {}", ip);
                return Err(StatusCode::TOO_MANY_REQUESTS);
            }
        }
        tracing::debug!("🔓 Edge Guard [PROTOTYPE]: Allowing {}", req.uri().path());
        return Ok(next.run(req).await);
    }

    let path = req.uri().path().to_string();

    // NOTE: Portfolio is now served at root `/` via fallback_service in main.rs.
    // The old redirect from `/` → `/portfolio/` has been removed.

    // Portfolio endpoints: allowed from tunnel but with tighter rate limiting
    let is_portfolio = PORTFOLIO_ALLOWED.iter().any(|p| path.starts_with(p));
    if is_portfolio {
        if let Some(ip) = req
            .headers()
            .get("cf-connecting-ip")
            .and_then(|v| v.to_str().ok())
        {
            static PORTFOLIO_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
            let limiter = PORTFOLIO_LIMITER.get_or_init(RateLimiter::new);

            // Strict: 10 requests per minute per IP (protects GPU from abuse)
            if !limiter.check(ip, 10, 60) {
                tracing::warn!("🛡️ Edge Guard: Portfolio rate limited tunnel IP {}", ip);
                return Err(StatusCode::TOO_MANY_REQUESTS);
            }
        }
        tracing::info!("🌐 Edge Guard: Allowing portfolio chat from tunnel");
        return Ok(next.run(req).await);
    }

    // Zen Demo endpoints: allowed from tunnel with moderate rate limiting
    let is_zen_demo = ZEN_DEMO_ALLOWED.iter().any(|p| path.starts_with(p));
    if is_zen_demo {
        if let Some(ip) = req
            .headers()
            .get("cf-connecting-ip")
            .and_then(|v| v.to_str().ok())
        {
            static ZEN_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
            let limiter = ZEN_LIMITER.get_or_init(RateLimiter::new);

            // Moderate: 30 requests per minute per IP
            if !limiter.check(ip, 30, 60) {
                tracing::warn!("🛡️ Edge Guard: Zen demo rate limited tunnel IP {}", ip);
                return Err(StatusCode::TOO_MANY_REQUESTS);
            }
        }
        tracing::info!("🎮 Edge Guard: Allowing Zen Mode demo from tunnel: {}", path);
        return Ok(next.run(req).await);
    }

    // Check blocked prefixes
    for prefix in BLOCKED_PREFIXES {
        if path.starts_with(prefix) {
            tracing::warn!(
                "🛡️ Edge Guard: Blocked tunnel access to {} from {}",
                path,
                req.headers()
                    .get("cf-connecting-ip")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown")
            );
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Rate limiting for tunnel traffic: 60 requests per minute per IP
    // (Red Hat H3: Resource abuse controls)
    if let Some(ip) = req
        .headers()
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
    {
        // Use a static rate limiter (initialized once)
        static LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
        let limiter = LIMITER.get_or_init(RateLimiter::new);

        if !limiter.check(ip, 60, 60) {
            tracing::warn!("🛡️ Edge Guard: Rate limited tunnel IP {}", ip);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_prefixes_cover_all_critical_findings() {
        // C1: Tool execution
        assert!(BLOCKED_PREFIXES.contains(&"/api/tools"));
        // C2: Model/inference switching
        assert!(BLOCKED_PREFIXES.contains(&"/api/models/switch"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/inference/switch"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/inference/refresh"));
        // C4: Session/project access
        assert!(BLOCKED_PREFIXES.contains(&"/api/sessions"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/projects"));
        // C5: Chat endpoints (resource + privacy)
        assert!(BLOCKED_PREFIXES.contains(&"/api/chat"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/v1/trinity"));
        // C6: Shell/Python (via tools)
        assert!(BLOCKED_PREFIXES.contains(&"/api/tools"));
        // H4: Admin mutation
        assert!(BLOCKED_PREFIXES.contains(&"/api/mode"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/quest"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/character"));
        // H5: Telemetry leaks
        assert!(BLOCKED_PREFIXES.contains(&"/api/hardware"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/status"));
        assert!(BLOCKED_PREFIXES.contains(&"/api/models"));
    }

    #[test]
    fn test_blocked_prefixes_match_subpaths() {
        let dangerous_paths = [
            "/api/tools/execute",
            "/api/tools",
            "/api/chat/stream",
            "/api/chat/zen",
            "/api/chat/yardmaster",
            "/api/models/switch",
            "/api/inference/switch",
            "/api/sessions/history",
            "/api/projects/archive",
            "/api/quest/complete",
            "/api/quest/advance",
            "/api/character/portfolio/artifact",
            "/api/creative/image",
            "/api/voice/conversation",
        ];

        for path in &dangerous_paths {
            let blocked = BLOCKED_PREFIXES.iter().any(|prefix| path.starts_with(prefix));
            assert!(blocked, "Path {} should be blocked but isn't!", path);
        }
    }

    #[test]
    fn test_safe_paths_not_blocked() {
        let safe_paths = [
            "/api/health",
            "/docs/PROFESSOR.md",
            "/trinity/index.html",
            "/assets/logo.png",
            "/index.html",
            "/",
        ];

        for path in &safe_paths {
            let blocked = BLOCKED_PREFIXES.iter().any(|prefix| path.starts_with(prefix));
            assert!(!blocked, "Path {} should NOT be blocked but is!", path);
        }
    }

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new();
        for _ in 0..60 {
            assert!(limiter.check("1.2.3.4", 60, 60));
        }
    }

    #[test]
    fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new();
        for _ in 0..60 {
            limiter.check("1.2.3.4", 60, 60);
        }
        // 61st request should be blocked
        assert!(!limiter.check("1.2.3.4", 60, 60));
    }

    #[test]
    fn test_rate_limiter_separate_ips() {
        let limiter = RateLimiter::new();
        for _ in 0..60 {
            limiter.check("1.2.3.4", 60, 60);
        }
        // Different IP should still be allowed
        assert!(limiter.check("5.6.7.8", 60, 60));
        // Original IP should be blocked
        assert!(!limiter.check("1.2.3.4", 60, 60));
    }

    #[test]
    fn test_tunnel_detection_cf_connecting_ip() {
        let req = Request::builder()
            .uri("/api/tools")
            .header("cf-connecting-ip", "1.2.3.4")
            .body(Body::empty())
            .unwrap();
        assert!(is_tunnel_request(&req));
    }

    #[test]
    fn test_tunnel_detection_local_request() {
        let req = Request::builder()
            .uri("/api/tools")
            .body(Body::empty())
            .unwrap();
        assert!(!is_tunnel_request(&req));
    }

    #[test]
    fn test_zen_demo_allowed_routes() {
        // These routes must be in ZEN_DEMO_ALLOWED for remote Zen Mode
        let zen_routes = [
            "/api/chat/zen",
            "/api/quest/state",
            "/api/quest/complete",
            "/api/character",
            "/api/models/active",
            "/api/health",
            "/api/tts",
            "/api/pearl",
        ];
        for route in &zen_routes {
            let allowed = ZEN_DEMO_ALLOWED.iter().any(|p| route.starts_with(p));
            assert!(allowed, "Zen route {} should be in ZEN_DEMO_ALLOWED", route);
        }
    }

    #[test]
    fn test_dangerous_routes_not_in_zen_demo() {
        // These must NEVER appear in ZEN_DEMO_ALLOWED
        let dangerous = [
            "/api/tools",
            "/api/tools/execute",
            "/api/models/switch",
            "/api/inference/switch",
            "/api/sessions",
            "/api/projects",
            "/api/chat/stream",
            "/api/chat/yardmaster",
            "/api/creative",
            "/api/mcp",
        ];
        for route in &dangerous {
            let allowed = ZEN_DEMO_ALLOWED.iter().any(|p| route.starts_with(p));
            assert!(!allowed, "Dangerous route {} must NOT be in ZEN_DEMO_ALLOWED!", route);
        }
    }
}

