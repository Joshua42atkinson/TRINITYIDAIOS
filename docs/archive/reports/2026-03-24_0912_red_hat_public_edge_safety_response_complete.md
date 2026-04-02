# Red Hat Public Edge Safety Response

**Timestamp**: 2026-03-24 09:12 UTC-04:00  
**Reviewer Perspective**: Red Hat AI security and safety team point of view  
**Context**: Public Purdue-facing deployment from a home-hosted edge device via `LDTAtkinson.com`  
**Scope**: Public internet safety, educational/privacy risk, exposed API surface, deployment hardening, and public-use readiness  
**Project**: `TRINITY ID AI OS`

---

## Handling Note

This report intentionally refers to the deployment as a **home-hosted residential edge deployment** rather than repeating the exact street address. That is a deliberate safety choice and aligns with minimizing infrastructure disclosure.

---

## Executive Verdict

**TRINITY ID AI OS is not safe for public internet exposure in its current form.**

If professors and students are meant to use the live application from `LDTAtkinson.com`, then from a Red Hat-style security and safety standpoint the answer today is:

**No public production sign-off. No public student/faculty exposure on the current build.**

### Why this is a no-go today

- **Public unauthenticated tool execution** exists.
- **Public runtime control endpoints** exist.
- **A critical exfiltration pivot** exists through arbitrary inference backend URL switching.
- **All users share one mutable runtime state** rather than having isolated identities and sessions.
- **Sensitive data routes are exposed without auth**.
- **Shell and Python are guarded by blacklist logic, not real isolation**.
- **The system is publicly reachable from a residential network** via reverse proxy and port forwarding.

### Red Hat disposition

- **Research prototype**: promising
- **Private single-user local system**: workable with caution
- **Operator-led live demo**: possible if tightly controlled
- **Public Purdue-facing website for live use**: **unsafe in current form**

---

## Why Rust Was the Right Choice, But Not the Whole Answer

Rust is an excellent foundational decision.

### What Rust helps with

- **Memory safety**
- **Reduced corruption risk**
- **Safer concurrency patterns**
- **Strong type guarantees**

### What Rust does not solve by itself

- **Unauthenticated dangerous routes**
- **Authorization failures**
- **Shared-state multi-user design flaws**
- **Reverse-proxy misconfiguration**
- **Data exfiltration by business logic**
- **Unsafe subprocess exposure**
- **Weak sandboxing**
- **Poor public deployment defaults**

### Red Hat view

Rust is a **strong base**, but Red Hat-grade safety requires more than memory safety. It requires:

- **Secure defaults**
- **Auth before action**
- **Least privilege**
- **Tenant separation**
- **Isolation of dangerous execution**
- **Constrained egress**
- **Honest control claims**
- **Operational hardening that matches the real deployment**

The current system does not yet meet that bar for public use.

---

## Deployment Reality Confirmed by Code and Config

The public deployment is not merely “local.” It is an internet-exposed service.

### Confirmed deployment facts

- **Public domain routing**: `configs/Caddyfile` proxies `ldtatkinson.com` traffic to `localhost:3000`
- **Residential exposure**: `scripts/deploy_website.sh` instructs router forwarding of ports `80` and `443` to the home LAN host
- **Network bind**: `crates/trinity/src/main.rs` binds to `0.0.0.0:3000`
- **CORS posture**: the app uses `CorsLayer::permissive()`
- **Public API surface**: the same server exposes frontend, docs, chat, tooling, persistence, creative, voice, quest, and runtime-control routes

### Why this matters

A local-first application can still be unsafe once publicly exposed.

The relevant safety question is not “Does it run on your machine?”

The relevant safety question is:

**What can an untrusted internet user reach, change, execute, extract, or overload?**

On that question, the answer is currently: **far too much**.

---

## Red Hat Safety Standard Applied Here

For a public educational AI system, I would expect the following minimum controls.

### Required public-service safety properties

- **Authentication before sensitive routes**
- **Authorization by role**
- **Per-user session isolation**
- **No dangerous tools on the public trust boundary**
- **No unrestricted runtime reconfiguration by public users**
- **No arbitrary egress redirection**
- **Request-size, rate, and concurrency controls**
- **Clear retention and privacy boundaries**
- **Claims that match actual enforcement**

By that standard, the current deployment does not pass.

---

## Risk Summary

### Critical findings

- **C1** — Public unauthenticated tool execution
- **C2** — Public arbitrary inference backend URL switching enables exfiltration
- **C3** — Public shared single-user state across all users
- **C4** — No meaningful authn/authz on sensitive public routes
- **C5** — Public exposure of session, project, and generated artifact data
- **C6** — Shell/Python execution is not sandboxed in a public-safe sense

### High findings

- **H1** — Reverse proxy lacks visible auth and security-header hardening
- **H2** — Broad file-access scope and permissive path model
- **H3** — No body-size, rate, or concurrency protection for abuse-prone endpoints
- **H4** — Admin-style runtime mutation endpoints are public
- **H5** — Operational telemetry and hardware details are publicly exposed
- **H6** — Plaintext persistence and broad local artifact retention

### Medium findings

- **M1** — Safety/privacy claims are stronger than the implementation supports
- **M2** — Residential edge hosting increases blast radius beyond normal app hosting

---

# Critical Findings

## C1. Public unauthenticated tool execution

### Evidence

- `main.rs` exposes `POST /api/tools/execute`
- `tools.rs` implements `execute_tool()` that directly invokes `run_tool()`
- `agent.rs` contains stronger permission and rate-limit checks in `execute_tool_internal()`, but those checks are tied to the agent path, not the public tool endpoint

### Why this is critical

The documentation describes a ring-based safety model. In practice, the public route allows direct access to the tool dispatch layer without the same internal persona-clearance path.

That means anonymous public traffic can target a tool-execution surface.

### Impact

Possible abuse paths include:

- **File access attempts**
- **Write operations**
- **Shell execution attempts**
- **Python execution attempts**
- **Project archiving or destructive state changes**
- **Heavy compute misuse**

Even if some calls fail or hit blocklists, the public exposure of this surface is unacceptable.

### Red Hat judgment

This is a **hard blocker** for public deployment.

---

## C2. Public arbitrary inference backend URL switching enables exfiltration

### Evidence

- `main.rs` exposes `POST /api/models/switch`
- the handler accepts either a backend name or a `url`
- `inference_router.rs` implements `set_active_url(url)`
- if the URL is not already known, the router **adds it as a new custom backend** and marks it active
- many inference paths consult `state.inference_router.read().await.active_url()`

### Why this is critical

An unauthenticated caller can likely redirect inference traffic away from your local model server and toward a remote host.

That turns a local-first system into a possible remote exfiltration path.

### What could be exposed

- **User prompts**
- **Conversation history**
- **Generated content**
- **Narrative context**
- **Professor/student interactions**
- **Potentially structured pedagogical state carried into prompts**

### Example abuse pattern

A remote caller sends a request to set the active model URL to an attacker-controlled service. Future inference traffic then leaves the machine while users still believe the system is local.

### Red Hat judgment

This is a **Critical blocker** and one of the most serious public deployment flaws in the current system.

---

## C3. Public shared single-user state across all users

### Evidence

`AppState` in `main.rs` contains shared mutable global state, including:

- `conversation_history`
- `game_state`
- `character_sheet`
- `session_id`
- `bestiary`
- `book`
- `book_updates`

Other confirmed details:

- startup generates a **single session ID**
- the character sheet is loaded from a **single local file** under `~/.local/share/trinity/character_sheet.json`
- quest state is loaded as shared server state
- journal state is stored in a shared local Trinity data directory

### Why this is critical

This is not a public multi-user architecture. It is a shared single-user runtime being exposed to many users.

### Impact

- **One user can influence another user’s state**
- **One user can overwrite the shared character sheet**
- **One user can affect quest progression and shared narrative state**
- **Confidentiality and integrity both fail**
- **Students and professors are not isolated from one another**

### Red Hat judgment

A public educational service without real tenant separation is a **hard blocker**.

---

## C4. No meaningful authentication or authorization on sensitive public routes

### Evidence

I did not find application-layer auth middleware or proxy-layer auth in the reviewed public-serving path.

Sensitive public routes include:

- `/api/tools/execute`
- `/api/models/switch`
- `/api/inference/switch`
- `/api/inference/refresh`
- `/api/mode`
- `/api/character`
- `/api/sessions`
- `/api/sessions/history`
- `/api/projects`
- `/api/projects/archive`
- `/api/projects/restore`
- `/api/journal`
- `/api/journal/:id`
- `/api/quest/*`
- `/api/creative/*`
- `/api/voice*`
- `/api/ingest`

### Why this is critical

Without auth and authz, the system has no meaningful distinction between:

- **Anonymous internet user**
- **Student**
- **Professor**
- **Trusted operator**
- **Administrator**

That is fundamentally incompatible with a Red Hat-style safety posture for public educational use.

### Red Hat judgment

This is a **Critical blocker**.

---

## C5. Public exposure of sessions, history, projects, and generated artifacts

### Evidence

- `main.rs` exposes `GET /api/sessions`
- `main.rs` exposes `GET /api/sessions/history`
- `main.rs` exposes `GET /api/projects`
- `main.rs` exposes `POST /api/projects/archive`
- `main.rs` exposes `POST /api/projects/restore`
- `creative.rs` exposes asset listing and asset retrieval routes
- `persistence.rs` stores message content, images, tool call params, and project data in straightforward persisted form

### Stored data confirmed in code

- `trinity_messages.content` as `TEXT`
- `trinity_messages.image_base64` as `TEXT`
- `trinity_tool_calls.params` as `JSONB`
- `trinity_tool_calls.result_preview` as `TEXT`
- `trinity_projects.gdd_json` as `JSONB`

### Why this is critical

In a Purdue-facing deployment, these records may contain:

- **Lesson planning content**
- **Personal or reflective user input**
- **Project and curriculum drafts**
- **Course-related artifacts**
- **Uploaded or ingested content**
- **Generated images/media**

Exposing these without authentication and user isolation is not acceptable.

### Red Hat judgment

This is a **Critical blocker**.

---

## C6. Shell and Python execution are not sandboxed in a Red Hat-safe sense

### Evidence

- `tool_shell()` runs `bash -c`
- `tool_shell()` uses blocked substring matching for dangerous patterns
- `tool_python_exec()` was already confirmed in prior code review as arbitrary Python execution via temp-file execution
- `tool_python_exec()` was previously confirmed to support package installation behavior
- `tool_project_archive()` operates on a caller-provided path via `std::fs::rename()`
- `zombie_check` is classified as `Safe` even though it can send kill signals with the right parameter

### Why this is critical

This is not true sandboxing in the sense Red Hat would require for public exposure.

What is missing includes:

- **Namespace or container isolation**
- **Restricted user separation**
- **Mandatory egress controls**
- **Fine-grained filesystem isolation**
- **cgroup/resource bounds**
- **Explicit execution policy**

Blacklist filtering can reduce accidental misuse, but it is not a public-safe isolation boundary.

### Red Hat judgment

This is a **Critical blocker**.

---

# High Findings

## H1. Reverse proxy lacks visible auth and security-header hardening

### Evidence

The checked `configs/Caddyfile` is a straightforward reverse proxy to `localhost:3000`.

I did not find visible:

- **Proxy auth**
- **Forward auth**
- **IP allowlists**
- **HSTS policy**
- **CSP**
- **X-Frame-Options**
- **X-Content-Type-Options**
- **Referrer-Policy**
- **Permissions-Policy**

### Why this matters

Even if the application were stronger, the public edge should still enforce a hardened baseline. Right now the proxy appears to be mostly a pass-through.

### Red Hat judgment

This is a serious deployment weakness.

---

## H2. File access is broader than a public service should allow

### Evidence

`validate_path_with_mode()` in `tools.rs` allows:

- **Reads from the entire home directory**
- Writes to:
  - the workspace
  - `~/.local/share/trinity/`
  - `~/Workflow/`
  - `/tmp`

Other related issues:

- `tool_project_archive()` uses a raw caller-provided path
- `tool_shell()` accepts a caller-supplied working directory

### Why this matters

On a residential host, the home directory can contain personal and operational data unrelated to Trinity.

A public-facing service should not have a path model this broad.

### Red Hat judgment

Least privilege is not being maintained.

---

## H3. No body-size, rate, or concurrency protection for abuse-prone endpoints

### Evidence

I did not find middleware such as:

- `RequestBodyLimitLayer`
- `DefaultBodyLimit`
- `ConcurrencyLimitLayer`

I also did not find visible proxy-level rate limiting in the checked Caddy config.

Abuse-prone endpoints include:

- `/api/voice`
- `/api/voice/conversation`
- `/api/tts`
- `/api/creative/image`
- `/api/creative/video`
- `/api/creative/music`
- `/api/creative/mesh3d`
- `/api/ingest`
- `/api/chat/yardmaster`

### Why this matters

A public attacker can attempt:

- **Oversized uploads**
- **Repeated generation requests**
- **Repeated voice requests**
- **Repeated ingestion and embedding churn**
- **General compute abuse**

On a home-hosted edge device, that can exhaust:

- **RAM**
- **Disk**
- **CPU**
- **GPU/NPU**
- **Sidecar availability**
- **Residential bandwidth**

### Red Hat judgment

This is a major availability risk.

---

## H4. Admin-style runtime mutation endpoints are public

### Evidence

Publicly reachable mutation routes include:

- `POST /api/mode`
- `POST /api/models/switch`
- `POST /api/inference/switch`
- `POST /api/inference/refresh`
- `POST /api/character`
- `/api/quest/*`
- `POST /api/projects/archive`
- `POST /api/projects/restore`
- `DELETE /api/journal/:id`

### Why this matters

These are operational or owner-level powers exposed to public callers.

### Red Hat judgment

These endpoints should not be public without strong role-based controls.

---

## H5. Operational telemetry and hardware details are public

### Evidence

Public routes expose internal operational information:

- `/api/health`
- `/api/hardware`
- `/api/models`
- `/api/models/active`
- `/api/voice/status`

The reviewed code shows exposure of information such as:

- **Active LLM URL**
- **Backend name**
- **Model hint**
- **Backend availability**
- **DB status**
- **Message/tool-call counts**
- **CPU/memory/GPU/NPU load**
- **Installed model inventory**
- **Voice service status**

### Why this matters

This is valuable attacker reconnaissance and can help tune abuse or discovery activity.

### Red Hat judgment

Operational transparency is useful for trusted operators, but excessive for anonymous public users.

---

## H6. Plaintext persistence and broad local artifact retention

### Evidence

The code persists:

- **Messages**
- **Image payloads**
- **Tool call params**
- **Tool call previews**
- **Project JSON**
- **Character sheet state**
- **Bestiary state**
- **Journal state**
- **Generated assets**

Storage locations and persistence models include PostgreSQL tables and local files under the user-local Trinity directories.

### Why this matters

Even if the system remains on your hardware, public educational use raises the bar. Private prompts, reflection data, or course artifacts should not be treated as casual debug persistence.

### Red Hat judgment

Retention, privacy, and deletion controls are not mature enough for public educational use.

---

# Medium Findings

## M1. Safety claims are stronger than actual enforcement

The project documentation strongly emphasizes:

- **Local-only execution**
- **Ring safety**
- **Sandboxing**
- **Privacy**
- **Safety boundaries**

The implementation shows good intent, but not enough enforcement to support those claims for the public deployment model now in use.

### Red Hat judgment

This is more than a documentation issue. Overstated controls can cause unsafe operator decisions.

---

## M2. Residential edge hosting increases blast radius

### Why this matters

A public service on a residential network adds extra risk factors:

- **Public IP exposure tied to a home network**
- **Router security dependency**
- **Potential lateral movement risk**
- **Less mature monitoring/incident response than datacenter hosting**
- **Availability tied to home infrastructure**

### Important nuance

I did **not** verify a public endpoint that directly forces audio playback on the host speakers. The `voice` and `tts` routes appear to return audio responses, not directly play them on the machine. However, that does **not** materially improve the public safety posture, because the larger concerns are still remote abuse, data exposure, and availability risk.

### Red Hat judgment

Home-edge public hosting is possible only when the application boundary is extremely strong. The current one is not.

---

## Concrete Attack Scenarios

### Scenario 1: Anonymous tool-surface probing

An attacker discovers `/api/tools/execute` and iterates tool names and parameter combinations looking for a successful path into file, process, or compute abuse.

### Scenario 2: Silent prompt exfiltration

An attacker switches the active inference URL to a host they control. Future interactions then leave the machine while users still believe the system is private and local.

### Scenario 3: Shared-state corruption across users

A student or attacker changes shared character state, quest progression, or operating mode, disrupting another user’s work or contaminating the demo/runtime state.

### Scenario 4: History and artifact scraping

A remote caller enumerates sessions, reads conversation history, lists projects, and pulls generated artifacts.

### Scenario 5: Resource exhaustion of the edge device

A remote caller repeatedly hits creative, voice, chat, and ingest endpoints, exhausting local resources and taking down the host.

### Scenario 6: Residential-network foothold after app compromise

If dangerous execution surfaces are successfully abused, the edge host becomes a pivot point inside the home environment.

---

## What I Would Require Before Any Public Purdue Use

## Immediate actions within 24 hours

If the app is currently public, my first recommendations are:

- **Remove public access to the Trinity application API immediately**
- **Keep only static portfolio or informational pages public**
- **Unroute or disable `/api/tools/execute` from the public edge**
- **Unroute or disable `/api/models/switch`, `/api/inference/*`, `/api/mode`, `/api/sessions*`, and `/api/projects*` from the public edge**
- **Stop representing the live public app as privacy-safe for external users until hardening is complete**
- **Confirm the Trinity host is isolated from the rest of the home network as much as possible**
- **Review logs for prior unknown use of sensitive routes**

---

## Safest Interim Operating Mode

If you need something usable for Purdue reviewers soon, the safest interim pattern is:

- **Public `LDTAtkinson.com` serves only portfolio or brochure content**
- **The real Trinity runtime is not open to anonymous internet traffic**
- Access to Trinity is provided only through one of these:
  - **VPN or Tailscale**
  - **SSH tunnel**
  - **Strict IP allowlist plus external auth**
  - **Operator-led demo over screen share**

This is not the final architecture, but it is far safer than the current public exposure model.

---

## Minimum Engineering Changes Before Even a Limited Pilot

### Identity and authorization

- **Add real authentication**
- **Add role-based authorization**
- **Separate operator/admin privileges from professor/student privileges**
- **Remove anonymous access to all state-changing routes**
- **Require explicit authorization for all data access routes**

### Trust-boundary redesign

- **Split the public web tier from the private control plane**
- **Remove direct tool execution from the public-facing trust boundary**
- **Do not expose shell or Python execution to public-origin requests at all**
- **Make operator tooling available only on a private channel**

### Data isolation

- **Introduce real per-user session isolation**
- **Stop using one shared mutable `AppState` as the public runtime identity**
- **Separate student data from operator data**
- **Separate faculty review flows from learner flows**

### Execution isolation

- **Remove `shell` and `python_exec` from the public deployment profile**
- If dangerous execution must exist for development, move it to a separate private worker with:
  - **A different Unix user**
  - **Restricted mounts**
  - **No home-directory access**
  - **Constrained egress**
  - **Resource limits**
  - **Preferably container or VM isolation**

### Network controls

- **Bind the app to `127.0.0.1` by default**
- **Replace permissive CORS with a specific allowlist**
- **Add proxy-level auth or external identity before the app**
- **Add rate limiting and body-size limits at proxy and app layers**
- **Add standard security headers**
- **Remove arbitrary external model URL switching from public APIs**
- **Constrain egress for inference and sidecar traffic**

### Privacy controls

- **Create a real retention policy**
- **Reduce stored sensitive content where possible**
- **Provide deletion and export controls by authenticated user**
- **Do not use real student data until privacy governance is defined**

### Observability and response

- **Log admin actions with actor identity**
- **Alert on model-switch events**
- **Alert on tool-execution attempts**
- **Alert on repeated abuse patterns**
- **Prepare a basic incident response playbook**

---

## Recommended Target Architecture

If you want this to become truly usable and defensible, I would aim for a four-part design.

### 1. Public web tier

- **Static or lightly dynamic frontend**
- **Strict auth**
- **No dangerous tools**
- **No runtime control endpoints**

### 2. Authenticated application tier

- **Per-user pedagogy services**
- **Isolated session state per user**
- **No shell/Python exposure**
- **No arbitrary backend URL changes**

### 3. Private operator tier

- **Developer/operator tooling only**
- **VPN or localhost only**
- **Separate credentials**
- **Strong audit trail**

### 4. Isolated worker tier

- **Sandboxed workers for dangerous or expensive operations**
- **Restricted filesystems**
- **Constrained network egress**
- **Resource limits**
- **Disposable execution where possible**

That is the kind of separation I would want before considering a future public pilot.

---

## What I Would Approve Today

### I would approve

- **Private single-operator local development**
- **Invite-only access through a private network overlay**
- **Operator-led demo sessions**
- **Public informational pages that do not expose Trinity’s control plane**

### I would not approve

- **Anonymous public access to the current Trinity API surface**
- **Student or faculty use over the open internet on the current build**
- **Any use involving real student data**
- **Any public statement that the current public deployment is Red Hat-grade secure or privacy-safe**

---

## Final Red Hat Judgment

TRINITY ID AI OS is an impressive project with real educational vision.

Its strongest qualities are:

- **Thoughtful local-first intent**
- **Strong narrative and pedagogical design**
- **Serious documentation effort**
- **Good foundational language choice in Rust**

But the current public deployment posture is not defensible.

### Final decision

- **As a research artifact**: promising
- **As a private prototype**: workable with caution
- **As a public Purdue-facing live website today**: unsafe
- **As a deployment Red Hat could endorse right now**: no

### One-sentence verdict

**Rust was the right foundation, but the current public deployment is not Red Hat-safe because the exposed trust boundary, shared-state model, and unauthenticated dangerous controls outweigh the language-level safety benefits.**
