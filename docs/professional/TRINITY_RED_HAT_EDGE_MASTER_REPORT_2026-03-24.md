# TRINITY Red Hat Edge Master Report

**Timestamp**: 2026-03-24 09:30 UTC-04:00  
**Document status**: Single authoritative report  
**Review perspective**: Red Hat AI security and safety team point of view  
**Deployment context**: Public Purdue-facing access to `LDTAtkinson.com` from a residential edge-hosted Trinity node  
**Project**: `TRINITY ID AI OS`

---

## Authoritative Note

This is the **single master document** for the current safety review.

It consolidates:

- live local runtime observations captured today
- code-backed Trinity findings
- deployment-path findings
- current external references from authoritative online sources
- Red Hat-style deployment guidance and go/no-go judgment

---

## Handling Note

This report intentionally refers to the environment as a **residential edge deployment** rather than repeating the exact street address. That is a deliberate operational-safety choice.

---

# Executive Summary

## Bottom line

**TRINITY ID AI OS is not safe for public internet exposure in its current form.**

If Purdue professors and students are expected to use the live application from `LDTAtkinson.com`, my Red Hat-style recommendation is:

- **[deployment decision]** **No public production sign-off**
- **[public student/faculty use]** **No-go in current form**
- **[private prototype use]** Acceptable only in tightly controlled private/operator scenarios

## Why this is a no-go today

- **[critical]** Public unauthenticated tool execution exists
- **[critical]** Public runtime mutation endpoints exist
- **[critical]** Public arbitrary inference backend URL switching enables possible data exfiltration
- **[critical]** The live service is effectively a shared single-user runtime, not a true multi-user system
- **[critical]** Session, history, project, and artifact routes are exposed without meaningful auth boundaries
- **[critical]** Shell and Python controls are blacklist-based, not hardened isolation
- **[high]** The edge host is internet-facing through residential port forwarding and reverse proxy exposure
- **[high]** Current runtime evidence shows the service is live on `0.0.0.0:3000` with public edge listeners on `:80` and `:443`

## Red Hat one-sentence verdict

**Rust was the right foundation, but the current public deployment is not Red Hat-safe because the exposed trust boundary, shared-state model, and unauthenticated dangerous controls outweigh the language-level safety benefits.**

---

# Part I — Real-Time Edge System Snapshot

## Snapshot method

The following snapshot was captured locally on:

- **[snapshot time]** `2026-03-24T09:30:42-04:00`

Using read-only local inspection of:

- **[host info]** `hostnamectl`
- **[runtime load]** `uptime`, `free -h`, `df -h`
- **[network listeners]** `ss -ltnp`
- **[local Trinity state]** `curl http://127.0.0.1:3000/api/health`
- **[local hardware state]** `curl http://127.0.0.1:3000/api/hardware`
- **[active model state]** `curl http://127.0.0.1:3000/api/models/active`

## Live host details

- **[hostname]** `trinity`
- **[OS]** `Ubuntu 24.04.4 LTS`
- **[kernel]** `Linux 6.19.4-061904-generic`
- **[architecture]** `x86-64`
- **[hardware vendor]** `GMKtec`
- **[hardware model]** `NucBox_EVO-X2`
- **[firmware version]** `EVO-X2 1.05`
- **[firmware date]** `2025-06-06`

## Live resource snapshot

- **[uptime]** ~`1 hour`
- **[load average]** `0.94, 0.94, 0.93`
- **[RAM total]** `124 GiB`
- **[RAM used]** `90 GiB`
- **[RAM available]** `34 GiB`
- **[swap used]** `2.3 GiB / 8.0 GiB`
- **[disk]** `/` and `/home` on `1.9T`, ~`29%` used

## Live network listeners observed

- **[public app bind]** `0.0.0.0:3000` served by process `trinity`
- **[public edge listeners]** `*:80` and `*:443`
- **[local LLM listener]** `127.0.0.1:8080` served by `longcat-sglang`
- **[local Postgres listener]** `127.0.0.1:5432`

## Live Trinity health snapshot

- **[app health]** `healthy`
- **[LLM connected]** `true`
- **[active LLM URL]** `http://127.0.0.1:8080`
- **[active backend]** `longcat-sglang`
- **[model hint]** `Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf`
- **[database]** `connected`
- **[persisted message count]** `70`
- **[persisted tool-call count]** `25`
- **[creative sidecars]** `ComfyUI: false`, `MusicGPT: false`
- **[voice sidecar]** `false`
- **[Trinity uptime from health endpoint]** `2120 seconds`

## Live Trinity hardware snapshot

- **[server state]** `running`
- **[inference connected]** `true`
- **[memory usage reported by Trinity]** `92844 MB`
- **[memory percent reported by Trinity]** `72.57%`
- **[CPU load reported by Trinity]** `2.215115`
- **[GPU load]** `3.0`
- **[NPU load]** `0.0`
- **[models available]**
  - `P: Mistral Small 4 119B MoE (Conductor/Pete) [68GB]`
  - `A-R-T (R): Crow 9B [5.3GB]`
  - `A-R-T (R): REAP 25B MoE [15GB]`

## What this live snapshot means

- **[exposure confirmation]** Trinity is not just "installed locally"; it is actively listening on `0.0.0.0:3000`
- **[public ingress confirmation]** `:80` and `:443` are open on the edge host
- **[resource pressure]** The machine is already using substantial memory, which increases the importance of request controls and abuse resistance
- **[operational reality]** This is an active edge node, not a hypothetical deployment

---

# Part II — Current Trinity Safety Decision

## Public-use decision

- **[public anonymous access]** Not acceptable
- **[public professor/student use]** Not acceptable
- **[private operator use]** Acceptable only with caution and scope limits
- **[live demo use]** Acceptable only if tightly controlled and not treated as a secure multi-user service

## Why the edge context changes everything

A local-first app can still be unsafe once publicly exposed.

For a residential/public edge deployment, the correct safety question is:

- **[primary question]** What can an untrusted internet user reach, change, extract, or overload?

For Trinity in its current form, the answer is:

- **[answer]** Too much

---

# Part III — Trinity-Specific Findings

## Critical findings

### C1. Public unauthenticated tool execution

- **[evidence]** `POST /api/tools/execute` is publicly routed in `main.rs`
- **[evidence]** `tools.rs` executes requested tools directly via `run_tool()`
- **[evidence]** the stricter internal agent-side Ring enforcement path is not the same as the public HTTP tool path
- **[risk]** anonymous callers can target a dangerous tool surface
- **[Red Hat judgment]** hard blocker

### C2. Public arbitrary inference backend URL switching

- **[evidence]** `POST /api/models/switch` accepts a `url`
- **[evidence]** `InferenceRouter::set_active_url()` will add a brand-new backend and make it active
- **[risk]** a remote caller may redirect prompts and responses to an attacker-controlled endpoint
- **[Red Hat judgment]** hard blocker

### C3. Public shared single-user state across all users

- **[evidence]** shared global `AppState` contains `conversation_history`, `game_state`, `character_sheet`, `session_id`, `bestiary`, `book`, and more
- **[evidence]** startup creates a single session identity and shared state model
- **[risk]** student, professor, and anonymous user traffic all collides in one mutable runtime
- **[Red Hat judgment]** hard blocker

### C4. No meaningful authn/authz boundary on sensitive routes

- **[evidence]** no application auth middleware was found in the reviewed public-serving path
- **[evidence]** no proxy auth was visible in the checked Caddy config
- **[risk]** no separation exists between anonymous user, student, professor, operator, and admin
- **[Red Hat judgment]** hard blocker

### C5. Public exposure of sessions, projects, history, and artifacts

- **[evidence]** public routes exist for sessions, session history, projects, project mutation, journals, assets, and creative outputs
- **[evidence]** persisted content includes messages, image payloads, tool call params, result previews, and project JSON
- **[risk]** confidentiality and privacy are not defensible for public educational use
- **[Red Hat judgment]** hard blocker

### C6. Shell and Python are not sandboxed in a public-safe sense

- **[evidence]** `tool_shell()` uses `bash -c`
- **[evidence]** controls are based on blocked pattern matching, not hard isolation
- **[evidence]** `python_exec` was previously confirmed to allow arbitrary execution behavior
- **[risk]** this is not a Red Hat-grade public execution boundary
- **[Red Hat judgment]** hard blocker

## High findings

### H1. Reverse proxy hardening is insufficient for public service

- **[evidence]** `Caddyfile` proxies `/api/*`, `/ws`, `/docs/*`, and frontend traffic directly to the Trinity server
- **[evidence]** no visible auth gate or header hardening was found in the checked config
- **[risk]** the edge is acting more like a pass-through than a hardened boundary

### H2. File and path access are broader than they should be

- **[evidence]** read validation allows the entire home directory
- **[evidence]** write validation allows multiple broad local paths
- **[risk]** residential-host blast radius is larger than just the app workspace

### H3. Resource abuse controls are weak or absent

- **[evidence]** no visible body-size middleware, concurrency limits, or app-level request-limit layers were found in the reviewed Axum setup
- **[evidence]** no proxy rate-limit policy was found in the checked Caddy config
- **[risk]** voice, creative, chat, and ingest routes can be abused for denial of service or cost/resource exhaustion

### H4. Admin-style mutation routes are public

- **[evidence]** public routes can switch mode, switch model backends, refresh inference state, update character state, archive/restore projects, and mutate quest state
- **[risk]** these are operational powers, not ordinary user actions

### H5. Public operational telemetry leaks internal runtime details

- **[evidence]** health and hardware routes expose backend, load, model, and system data
- **[risk]** this materially helps attacker reconnaissance

### H6. Persistent storage and retention are not mature enough for public educational use

- **[evidence]** sensitive interaction data is stored plainly in DB/local state structures
- **[risk]** privacy and data-governance controls are not strong enough for public student/faculty use

---

# Part IV — Standards and External Reference Alignment

## Why external references matter here

Because Trinity is being positioned as a real public edge system, the safety judgment should not rest only on internal documentation or the fact that it is written in Rust.

It should be measured against:

- **[AI governance standards]** trustworthy AI deployment guidance
- **[network/identity standards]** zero trust and edge hardening guidance
- **[API security standards]** authn/authz and resource-consumption guidance
- **[Red Hat edge guidance]** lifecycle, visibility, security, and operational consistency

---

## 1. NIST AI Risk Management Framework

### Source

- **[title]** `AI Risk Management Framework | NIST`
- **[URL]** `https://www.nist.gov/itl/ai-risk-management-framework`
- **[key date]** NIST states AI RMF 1.0 was **released on January 26, 2023**
- **[key date]** NIST states the Generative AI Profile was **released on July 26, 2024**
- **[access date]** `2026-03-24`

### Relevance to Trinity

NIST frames the AI RMF as guidance to incorporate **trustworthiness considerations into the design, development, use, and evaluation** of AI systems.

### Gap versus Trinity

- **[gap]** Trinity's public deployment does not yet align trust claims with actual runtime controls
- **[gap]** public deployment creates privacy, safety, and misuse risks that are not sufficiently governed
- **[gap]** the live system allows dangerous operational paths that are inconsistent with trustworthy public use

### Red Hat interpretation

Trinity presently shows **ambition toward trustworthy AI**, but not enough deployed control maturity to claim public operational trustworthiness.

---

## 2. NIST SP 800-207 — Zero Trust Architecture

### Source

- **[title]** `NIST SP 800-207, Zero Trust Architecture`
- **[URL]** `https://csrc.nist.gov/pubs/sp/800/207/final`
- **[publication date]** **August 2020**
- **[access date]** `2026-03-24`

### Relevance to Trinity

This is directly relevant because the Trinity deployment is public and internet-facing.

### Gap versus Trinity

- **[gap]** Trinity currently relies too heavily on network reachability assumptions rather than identity-based enforcement
- **[gap]** sensitive functions are not protected by strong identity, role, and policy controls
- **[gap]** anonymous traffic can reach routes that should require explicit trust decisions

### Red Hat interpretation

From a Zero Trust standpoint, Trinity currently has too much **implicit trust** in callers reaching the public API boundary.

---

## 3. CISA guidance on protecting network edge devices

### Source

- **[title]** `Guidance and Strategies to Protect Network Edge Devices | CISA`
- **[URL]** `https://www.cisa.gov/resources-tools/resources/guidance-and-strategies-protect-network-edge-devices`
- **[access date]** `2026-03-24`

### Key relevance from retrieved content

CISA frames this guidance as protection for:

- **[covered systems]** firewalls, routers, VPN gateways, IoT devices, internet-facing servers, and other internet-facing edge systems

It emphasizes:

- **[secure by design]** reducing compromise risk at the edge
- **[monitoring/logging]** logging and remote monitoring capabilities
- **[hardening]** securing and managing edge devices effectively

### Gap versus Trinity

- **[gap]** Trinity is functioning as an internet-facing edge application without a sufficiently hardened boundary
- **[gap]** the public app/API/control plane are too tightly combined
- **[gap]** public runtime mutation is too permissive

### Red Hat interpretation

CISA's edge guidance reinforces the conclusion that edge exposure increases the importance of hardening, visibility, isolation, and lifecycle discipline.

---

## 4. CISA BOD 26-02 — Mitigating Risk From End-of-Support Edge Devices

### Source

- **[title]** `BOD 26-02: Mitigating Risk From End-of-Support Edge Devices | CISA`
- **[URL]** `https://www.cisa.gov/news-events/directives/bod-26-02-mitigating-risk-end-support-edge-devices`
- **[access date]** `2026-03-24`

### Key relevance from retrieved content

The retrieved CISA text emphasizes that edge devices are attractive targets because of:

- **[identity adjacency]** their integrations with identity management systems
- **[network reach]** their extensive reach into the organization's network
- **[patch risk]** newly discovered unpatched vulnerabilities
- **[public exposure]** public-facing placement on the network edge

CISA also highlights actions related to:

- **[MFA]** multifactor authentication
- **[asset management]** inventory and identification
- **[isolation]** isolation of critical workloads and strong access policy
- **[encryption]** encryption of data in transit
- **[continuous discovery]** continuous discovery and lifecycle management

### Gap versus Trinity

- **[gap]** public access currently reaches sensitive runtime controls without MFA or strong access policy
- **[gap]** there is no meaningful separation between public functions and critical workloads
- **[gap]** the current deployment is publicly exposed without a mature lifecycle/security boundary

### Red Hat interpretation

Even though CISA is speaking at the edge-device and federal-operations level, the same lesson applies here: **public edge systems demand disciplined lifecycle control, strong access policy, and explicit isolation**.

---

## 5. OWASP API Security Top 10 — 2023 Edition

### Source

- **[title]** `OWASP Top 10 API Security Risks – 2023`
- **[URL]** `https://owasp.org/API-Security/editions/2023/en/0x11-t10/`
- **[edition date]** `2023 edition`
- **[access date]** `2026-03-24`

### Why this matters

Trinity is exposed as an API-heavy edge service. OWASP is directly applicable.

### Most relevant OWASP categories for Trinity

- **[API2:2023]** Broken Authentication
- **[API4:2023]** Unrestricted Resource Consumption
- **[API5:2023]** Broken Function Level Authorization
- **[API8:2023]** Security Misconfiguration
- **[API9:2023]** Improper Inventory Management
- **[API10:2023]** Unsafe Consumption of APIs

### Red Hat interpretation

Trinity's public deployment maps closely to multiple OWASP API-risk categories, especially around auth, admin-function access, and resource abuse.

---

## 6. OWASP API2:2023 — Broken Authentication

### Source

- **[title]** `API2:2023 Broken Authentication`
- **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa2-broken-authentication/`
- **[edition date]** `2023 edition`
- **[access date]** `2026-03-24`

### Key prevention guidance from retrieved content

OWASP advises:

- **[auth discipline]** know all authentication flows
- **[use standards]** do not reinvent auth or token handling
- **[MFA]** implement MFA where possible
- **[anti-brute-force]** rate limiting and lockout protections for auth-sensitive flows
- **[re-auth]** require re-authentication for sensitive operations

### Gap versus Trinity

- **[gap]** Trinity currently exposes sensitive operations without visible authentication at all
- **[gap]** there is no apparent MFA or re-auth path for sensitive runtime control
- **[gap]** public callers can reach dangerous functions directly

### Red Hat interpretation

This is a direct mismatch with OWASP Broken Authentication guidance.

---

## 7. OWASP API5:2023 — Broken Function Level Authorization

### Source

- **[title]** `API5:2023 Broken Function Level Authorization`
- **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa5-broken-function-level-authorization/`
- **[edition date]** `2023 edition`
- **[access date]** `2026-03-24`

### Key prevention guidance from retrieved content

OWASP advises:

- **[consistent authorization module]** use a consistent, easy-to-analyze authorization module
- **[deny by default]** deny all access by default, then explicitly grant by role
- **[admin separation]** ensure admin functions enforce group/role checks

### Gap versus Trinity

- **[gap]** admin-style endpoints are publicly reachable
- **[gap]** the app does not appear to deny by default at the public boundary
- **[gap]** public and administrative behaviors are not clearly separated by role

### Red Hat interpretation

This is one of the strongest external confirmations of the current Trinity risk posture.

---

## 8. OWASP API4:2023 — Unrestricted Resource Consumption

### Source

- **[title]** `API4:2023 Unrestricted Resource Consumption`
- **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa4-unrestricted-resource-consumption/`
- **[edition date]** `2023 edition`
- **[access date]** `2026-03-24`

### Key prevention guidance from retrieved content

OWASP advises:

- **[resource limits]** limit memory, CPU, file descriptors, and processes
- **[payload limits]** enforce maximum sizes on incoming payloads and uploads
- **[rate limiting]** limit how often a client can interact with an API within a defined timeframe
- **[validation]** validate request parameters server-side

### Gap versus Trinity

- **[gap]** no strong evidence of route-level rate limits or body-size enforcement was found in the public Axum stack
- **[gap]** Trinity exposes compute-heavy endpoints for voice, ingest, and creative generation
- **[gap]** live host memory is already high, making abuse protection even more important

### Red Hat interpretation

This is a direct match to one of Trinity's major public-edge availability risks.

---

## 9. Red Hat edge security guidance

### Source

- **[title]** `Red Hat Shares ― Edge computing: Security`
- **[URL]** `https://www.redhat.com/en/blog/red-hat-shares-edge-computing-security`
- **[access date]** `2026-03-24`

### Key relevance from retrieved content

The Red Hat content highlights that edge deployments face increased risk because of:

- **[reduced physical security]** less traditional datacenter control
- **[limited compute footprint]** constrained resources
- **[remote management]** management complexity at the edge
- **[lack of on-site IT]** fewer hands on the system
- **[expanded attack surface]** more data and processing outside the traditional datacenter

### Gap versus Trinity

- **[gap]** Trinity combines a wide public API surface with resource-intensive capabilities on a residential edge host
- **[gap]** the current design does not sufficiently compensate for the increased attack surface described in the Red Hat guidance

### Red Hat interpretation

The actual runtime and deployment path match the classic risk pattern Red Hat warns about in edge systems.

---

## 10. Red Hat Device Edge guidance

### Source

- **[title]** `Red Hat Device Edge`
- **[URL]** `https://www.redhat.com/en/technologies/device-edge`
- **[access date]** `2026-03-24`

### Key relevance from retrieved content

Red Hat Device Edge emphasizes:

- **[resource-constrained reality]** far-edge devices need a different approach
- **[lifecycle management]** security-focused lifecycle management from deployment to decommissioning
- **[visibility/control]** device updates, visibility, and management services
- **[operational consistency]** consistency from core to cloud to edge

### Gap versus Trinity

- **[gap]** Trinity's public deployment is not yet separated into a hardened operational model with strong lifecycle/security controls
- **[gap]** the current system is still closer to a powerful prototype than a managed edge platform

### Red Hat interpretation

This reinforces the recommendation that Trinity needs a more deliberate edge architecture rather than direct public exposure of the current app server.

---

# Part V — Crosswalk: External Standards to Trinity Gaps

## Direct alignment

- **[NIST AI RMF]** Trustworthiness must be incorporated into deployment and evaluation  
  **[Trinity gap]** public trust claims currently exceed the live control model

- **[NIST SP 800-207]** Remove implicit trust and enforce identity-centered access  
  **[Trinity gap]** anonymous traffic reaches sensitive routes

- **[CISA edge guidance]** Harden edge systems, improve monitoring, secure by design  
  **[Trinity gap]** the public edge/control plane boundary is too open

- **[CISA BOD 26-02]** Inventory, isolate, enforce access policy, and treat edge exposure as high-risk  
  **[Trinity gap]** public reachability plus broad runtime control is not sufficiently isolated

- **[OWASP API2]** use real auth, MFA, anti-brute-force, and standard mechanisms  
  **[Trinity gap]** no visible meaningful auth at the public boundary

- **[OWASP API5]** deny by default and gate functions by role  
  **[Trinity gap]** admin-style functions are public

- **[OWASP API4]** enforce resource limits and payload bounds  
  **[Trinity gap]** public heavy-compute endpoints lack strong visible abuse controls

- **[Red Hat edge guidance]** edge expands attack surface and requires layered security  
  **[Trinity gap]** public deployment currently overexposes the runtime relative to its safety needs

---

# Part VI — Immediate Required Actions

## Within 24 hours

- **[1]** Remove public access to the Trinity application API
- **[2]** Keep only static portfolio/informational content public at `LDTAtkinson.com`
- **[3]** Disable or unroute `/api/tools/execute` from the public edge immediately
- **[4]** Disable or unroute `/api/models/switch`, `/api/inference/*`, `/api/mode`, `/api/sessions*`, and `/api/projects*` from the public edge
- **[5]** Stop treating the live public deployment as privacy-safe for third-party use until hardening is complete
- **[6]** Review logs for prior unknown use of exposed sensitive endpoints
- **[7]** Ensure the edge host is as isolated as possible from the rest of the residential network

## Within 7 days

- **[identity]** Put the app behind real authentication
- **[authorization]** Separate anonymous, student, professor, and admin/operator roles
- **[exfiltration fix]** Remove arbitrary URL switching from any public route
- **[resource limits]** Add request-size, rate, and concurrency controls
- **[edge hardening]** Add security headers and non-permissive CORS
- **[telemetry]** Reduce public exposure of operational detail endpoints

## Within 30–60 days

- **[architecture]** Split public web tier from private control plane
- **[tenant model]** Introduce real per-user session isolation
- **[dangerous tooling]** Remove shell and Python from the public deployment profile entirely
- **[worker isolation]** Move dangerous or high-cost tasks into isolated workers with strict filesystem and egress controls
- **[privacy governance]** Define retention, deletion, and data-classification policy before any real educational data use

---

# Part VII — Safe Interim Operating Model

## What is safe enough right now

- **[public site]** public portfolio and informational content only
- **[private runtime]** Trinity application available only through:
  - VPN or Tailscale
  - SSH tunnel
  - strict IP allowlist plus external auth
  - operator-led demo sessions

## What is not safe enough right now

- **[not approved]** anonymous public access to Trinity's current API surface
- **[not approved]** public student or faculty use over the open internet
- **[not approved]** any use involving real student data
- **[not approved]** any statement that the present public deployment is Red Hat-grade secure

---

# Part VIII — Final Red Hat Decision

## Decision statement

TRINITY ID AI OS is a compelling and thoughtful project.

Its strengths are real:

- **[strength]** strong local-first intent
- **[strength]** coherent pedagogical design
- **[strength]** unusually disciplined documentation
- **[strength]** good foundational implementation language choice in Rust

But for the actual deployment being discussed here, the safety decision is clear:

- **[as research artifact]** promising
- **[as private prototype]** workable with caution
- **[as public Purdue-facing live website today]** unsafe
- **[as deployment Red Hat could endorse right now]** no

## Final judgment

**No public production sign-off. No public student/faculty exposure on the current build.**

The public edge boundary must be redesigned before Trinity can be considered usable at a Red Hat-grade safety standard.

---

# Part IX — Reference Appendix

## External online references used

- **[NIST]** `AI Risk Management Framework | NIST`  
  **[URL]** `https://www.nist.gov/itl/ai-risk-management-framework`  
  **[date noted from retrieved source]** AI RMF 1.0 released `2023-01-26`; GenAI Profile released `2024-07-26`  
  **[accessed]** `2026-03-24`

- **[NIST]** `NIST SP 800-207, Zero Trust Architecture`  
  **[URL]** `https://csrc.nist.gov/pubs/sp/800/207/final`  
  **[date noted from retrieved source]** published `2020-08`  
  **[accessed]** `2026-03-24`

- **[CISA]** `Guidance and Strategies to Protect Network Edge Devices`  
  **[URL]** `https://www.cisa.gov/resources-tools/resources/guidance-and-strategies-protect-network-edge-devices`  
  **[date noted]** access date used because the retrieved chunk did not surface a publication date  
  **[accessed]** `2026-03-24`

- **[CISA]** `BOD 26-02: Mitigating Risk From End-of-Support Edge Devices`  
  **[URL]** `https://www.cisa.gov/news-events/directives/bod-26-02-mitigating-risk-end-support-edge-devices`  
  **[date noted]** access date used because the retrieved chunk did not surface a publication date  
  **[accessed]** `2026-03-24`

- **[OWASP]** `OWASP Top 10 API Security Risks – 2023`  
  **[URL]** `https://owasp.org/API-Security/editions/2023/en/0x11-t10/`  
  **[date noted]** `2023 edition`  
  **[accessed]** `2026-03-24`

- **[OWASP]** `API2:2023 Broken Authentication`  
  **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa2-broken-authentication/`  
  **[date noted]** `2023 edition`  
  **[accessed]** `2026-03-24`

- **[OWASP]** `API4:2023 Unrestricted Resource Consumption`  
  **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa4-unrestricted-resource-consumption/`  
  **[date noted]** `2023 edition`  
  **[accessed]** `2026-03-24`

- **[OWASP]** `API5:2023 Broken Function Level Authorization`  
  **[URL]** `https://owasp.org/API-Security/editions/2023/en/0xa5-broken-function-level-authorization/`  
  **[date noted]** `2023 edition`  
  **[accessed]** `2026-03-24`

- **[Red Hat]** `Red Hat Shares ― Edge computing: Security`  
  **[URL]** `https://www.redhat.com/en/blog/red-hat-shares-edge-computing-security`  
  **[date noted]** access date used because the retrieved chunk did not surface a publication date  
  **[accessed]** `2026-03-24`

- **[Red Hat]** `Red Hat Device Edge`  
  **[URL]** `https://www.redhat.com/en/technologies/device-edge`  
  **[date noted]** access date used because the retrieved chunk did not surface a publication date  
  **[accessed]** `2026-03-24`

## Local live runtime evidence used

- **[date captured]** `2026-03-24T09:30:42-04:00`
- **[host commands]** `hostnamectl`, `uptime`, `free -h`, `df -h`, `ss -ltnp`
- **[local HTTP checks]** `http://127.0.0.1:3000/api/health`, `http://127.0.0.1:3000/api/hardware`, `http://127.0.0.1:3000/api/models/active`

---

# Final Use Note

If you need to hand one file to Purdue reviewers, faculty, or external stakeholders, use this file:

- **[single file]** `TRINITY_RED_HAT_EDGE_MASTER_REPORT_2026-03-24.md`
