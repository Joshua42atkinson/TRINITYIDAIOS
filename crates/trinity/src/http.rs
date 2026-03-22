// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        http.rs
// PURPOSE:     Shared HTTP clients and health check utilities
//
// ARCHITECTURE:
//   • Three lazy-static reqwest clients with different timeout profiles
//   • One generic health_check() function replacing 8 identical implementations
//   • Follows the Pythagorean Oath: healthier architecture, connection pooling
//   • Aligns with Bible §Sidecar Architecture — unified sidecar interaction pattern
//
// ISOMORPHISM:
//   Every sidecar (ComfyUI, MusicGPT, Hunyuan3D, Voice, Researcher) follows
//   the same pattern: health probe → client → POST → parse → return.
//   This module provides the shared foundation for that pattern.
//
// CHANGES:
//   2026-03-22  Consolidation  Created — 20 client duplications → 3 shared
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::LazyLock;

/// Quick client — 5 second timeout. For health checks, status probes, fast API calls.
pub static QUICK: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .pool_max_idle_per_host(4)
        .build()
        .expect("Failed to build quick HTTP client")
});

/// Standard client — 60 second timeout. For inference, creative generation, tool calls.
pub static STANDARD: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .pool_max_idle_per_host(4)
        .build()
        .expect("Failed to build standard HTTP client")
});

/// Long client — 300 second timeout. For reasoning-heavy inference, large file uploads.
pub static LONG: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .pool_max_idle_per_host(2)
        .build()
        .expect("Failed to build long HTTP client")
});

/// Generic health check — replaces 8 identical check_*_health() functions.
///
/// Probes `{url}/health` (or a custom endpoint) with a 3-second timeout.
/// Returns true if the response is a 2xx status code.
///
/// # Examples
/// ```
/// let comfyui_ok = http::check_health("http://127.0.0.1:8188").await;
/// let voice_ok = http::check_health("http://127.0.0.1:8200").await;
/// ```
pub async fn check_health(base_url: &str) -> bool {
    let url = if base_url.ends_with("/health") {
        base_url.to_string()
    } else {
        format!("{}/health", base_url.trim_end_matches('/'))
    };

    QUICK
        .get(&url)
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clients_exist() {
        // Verify lazy-static clients can be created
        let _ = &*QUICK;
        let _ = &*STANDARD;
        let _ = &*LONG;
    }

    #[tokio::test]
    async fn test_health_check_unreachable() {
        // Health check against a port that's definitely not running
        let result = check_health("http://127.0.0.1:59999").await;
        assert!(!result);
    }

    #[test]
    fn test_health_url_construction() {
        // Verify URL construction logic
        let base = "http://127.0.0.1:8188";
        let expected = "http://127.0.0.1:8188/health";
        let url = if base.ends_with("/health") {
            base.to_string()
        } else {
            format!("{}/health", base.trim_end_matches('/'))
        };
        assert_eq!(url, expected);

        // With trailing slash
        let base2 = "http://127.0.0.1:8188/";
        let url2 = format!("{}/health", base2.trim_end_matches('/'));
        assert_eq!(url2, expected);
    }
}
