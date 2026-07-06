# Trinity Security Model

## Overview

Trinity is served from a local edge device (Strix Halo) and accessed via two pathways:

1. **Tailscale (primary)** — Encrypted mesh VPN, direct device-to-device. Used for daily phone access.
2. **Cloudflare Tunnel (fallback)** — TLS termination, DDoS protection, IP hiding. Used for off-network access.
3. **API Authentication** — Bearer token on dangerous endpoints
4. **Rate Limiting** — Per-IP sliding window
5. **Input Validation** — Message length, turn caps, path sanitization

### Tailscale (Primary)

- Desktop (Trinity): `100.83.222.35:3000`
- Phone (Pixel 10 Pro XL): `100.83.9.71`
- WireGuard-based encryption, no public exposure
- More reliable than Cloudflare quick tunnels (which change URL on restart)

### Cloudflare Tunnel (Fallback)

- Named tunnel: `trinity.ldatkinson.com` → `localhost:3000`
- Used for off-network access (Purdue professors, students)
- Config at `~/.cloudflared/config.yml`

## API Key

Set via environment variable:
```bash
export TRINITY_API_KEY="your-secret-key-here"
```

If not set, all endpoints are open (local dev mode — safe behind NAT/firewall).
If set, protected endpoints require `Authorization: Bearer <key>` header.

## Protected Endpoints

These endpoints require authentication when `TRINITY_API_KEY` is set:

| Endpoint | Method | Risk |
|----------|--------|------|
| `/api/tools/execute` | POST | Arbitrary tool execution |
| `/api/models/switch` | POST | Redirect inference to attacker server |
| `/api/inference/switch` | POST | Switch inference backend |
| `/api/inference/start` | POST | Launch model processes |
| `/api/inference/stop` | POST | Kill model processes |
| `/api/inference/hotel/*` | POST | Hotel mode control |
| `/api/config/setup` | POST | Configuration changes |
| `/api/reset/demo` | POST | Data deletion |
| `/api/daydream/command` | POST | Autonomous agent commands |

## Public Endpoints (no auth required)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Health check |
| `/api/chat/yardmaster` | POST | Agent chat (SSE) |
| `/api/chat/stream` | POST | Chat streaming |
| `/api/rag/search` | POST | RAG semantic search |
| `/api/rag/stats` | GET | RAG statistics |
| `/api/inference/hotel` | GET | Hotel status (read-only) |
| `/api/inference/status` | GET | Inference status (read-only) |
| `/api/creative/image` | POST | Image generation |
| `/trinity/phone.html` | GET | Phone PWA |

## Rate Limits

| Category | Limit | Window |
|----------|-------|--------|
| Chat (`/api/chat/*`) | 15 requests | per minute per IP |
| Tools (`/api/tools/*`, `/api/inference/*`) | 10 requests | per minute per IP |
| Global (all endpoints) | 120 requests | per minute per IP |

Rate limited requests return `429 Too Many Requests`.

## Input Validation

- Message length: max 10,000 characters
- Agent turns: max 50 (was 500, reduced for safety)
- Default max_turns: 50 (was 65)

## CORS

All origins allowed (Cloudflare tunnel uses dynamic URLs). Auth middleware protects dangerous endpoints regardless of origin.

## Phone PWA Authentication

The phone PWA supports API key via URL parameter:
```
# Tailscale (primary)
http://100.83.222.35:3000/trinity/phone.html?key=your-secret-key

# Cloudflare (fallback)
https://trinity.ldatkinson.com/trinity/phone.html?key=your-secret-key
```

The key is stored in `localStorage` for subsequent visits. Auth headers are sent on protected endpoint calls (hotel controls).

## Known Limitations

- **Single session**: No multi-user isolation. All users share the same agent state. Acceptable for single-user desktop use; **must be addressed before public multi-user deployment**.
- **In-memory rate limiter**: Resets on server restart. Not suitable for multi-instance deployments.
- **No HTTPS on local Tailscale**: Tailscale provides encryption at the VPN layer. Local HTTP is only accessible within the mesh.
- **Static key**: No key rotation, no per-user keys. Suitable for 1-5 users, not production.
- **`/api/tools/execute` exposes unauthenticated tool execution** — known critical finding. Must be behind auth before public exposure.
- **`/api/models/switch` accepts arbitrary URLs** — prompt exfiltration risk. Must be behind auth before public exposure.

## Cloudflare Tunnel (Fallback)

### Quick tunnel (temporary, for testing):
```bash
cloudflared tunnel --url http://localhost:3000
```
Generates a random `*.trycloudflare.com` URL. No config needed.

### Named tunnel (permanent, for production):
```bash
cloudflared tunnel run trinity-tunnel
```
Config at `~/.cloudflared/config.yml`. Routes `trinity.ldatkinson.com` → `localhost:3000`.

**Note:** Named tunnel DNS requires manual CNAME creation in Cloudflare dashboard if `cloudflared tunnel route dns` fails.
