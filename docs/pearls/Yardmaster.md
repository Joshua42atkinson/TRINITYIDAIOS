# 🏗️ YARDMASTER TERMINAL (The Exhale)
*Status: Phase 4 Industrialized | Layer: Runtime/Interface | Parent: InferenceRouter*

The Yardmaster is Trinity’s **Exhale** surface—the place where the structured instructions developed during the Planning (Inhale) phase are transformed into live execution. 

Unlike the Art Studio, which focuses on reflective dialogue and vocabulary synthesis (VAAM), the Yardmaster focuses on **Tool Execution, Focus Toggling, and Output Generation**.

## 1. Physical Mechanics (The Surface)
The Yardmaster provides a 3-column, high-density professional IDE layout designed specifically for zero-friction LLM interactions, rebuilt in Phase 4 using premium glassmorphic tokens:
1. **Left (Context & Telemetry):** The Live Quest Card (Objective Tracking) and The Beast Logger (real-time server telemetry and hardware events).
2. **Center (Chat & Flow):** The core chat window, powered by standard SSML/VAAM capabilities inherited from `agent.rs`. The execution window allows for seamless persona switching (`Dev` -> `Recycler` -> `Pete`).
3. **Right (Capability Explorer):** RAG injection settings and **The Hook Book** terminal, broadcasting the user's available vector-searchable tools mapped in real-time to the agent's context.

## 2. Mathematical State (The Under-the-Hood Mechanics)
In **Phase 4: Yardmaster Integration**, the `agent.rs` SSE routing logic was overhauled to divorce "Game Tracking" from "Narrative Friction".

- **Ambient Tracking (`track_mechanics = true`):** No matter which persona the user occupies (even the root `"dev"` mode), the OS quietly scans vocabulary via VAAM to award Coal and evaluates prompt alignment without interrupting the user. This ensures the Iron Road remains unbroken in the background.
- **Narrative Enforcement (`enforce_narrative = true`):** Active only for `"recycler"` and `"programmer"` KV slots. This physically interrupts the chat loop, forcing Scope Creep encounters upon the user or halting generation natively if they drift off the active ADDIECRAPEYE phase. The `SACRED CIRCUITRY` is dynamically appended to system prompts.

## 3. LitRPG Context (The Hook Book)
*“The Hook Book is to Trinity what a spell book is to a wizard — it doesn't just list what you can do. It reveals what you can BECOME.”*

The right-hand panel of the Yardmaster was specifically refactored to directly surface **The Hook Book**. By presenting available tooling explicitly to the user alongside the hardware load, Pete (Slot 1) and the Yardmaster act in tandem. Every time a tool is executed, it validates a competency block on the user's Character Sheet. The UI serves as a live visualization of the system's "Karma", proving functional capability to institutional evaluators.

## 4. [E] Engineering Integration (Rust Backend Mapping)
The Yardmaster UI surface is technically powered by three structural pillars in `crates/trinity/src/`:

1. **`agent.rs` (The Synaptic Loop):** Driving the core inference stream (`POST /api/agent/chat`). Uses Server-Sent Events (SSE) to deliver real-time chunks of LLM output, extracting XML `<thought>` blocks, parsing JSON `{"tool": ...}` directives, and executing filesystem changes on behalf of Programmer Pete.
2. **`jobs.rs` (The Overnight Crew):** Exposed via `<JobsPanel />` submitting to `POST /api/jobs`. This decrypts the agent loop from the browser entirely, spinning up standalone `tokio::spawn` asynchronous workers to run autonomously. Persisted to `trinity_background_jobs` table in SQLite (`persistence.rs`) with cancellation interception support.
3. **`inference_router.rs` (The Hardware Bindings):** Powers the system status bar. Performs runtime health checks on generic OpenAI endpoints (`http://127.0.0.1:1234`) to dynamically map external LLM processes without crashing the IDE.

## 5. [R] Research & Architecture Decisions
- **Decoupling from SSE:** In earlier versions, closing the Yardmaster browser tab severed the SSE connection, killing whatever coding task Pete was executing. We introduced `jobs.rs` specifically to bridge this gap, ensuring that long-turn tasks (like formatting an entire e-learning module) continue entirely in the background, saving final logs and diffs to `~/Workflow/trinity-reports/`.
- **Pre-Escaping Internal Monologue:** The `Yardmaster.jsx` React file manually intercepts `<thinking>` and `<thought>` tags before feeding them into standard markdown parsers. If left un-escaped, Chrome's DOM parser treats the LLM's raw XML thought process as an invalid HTML element and hides it from the user, causing a "Black Box" transparency failure. Pre-escaping forces the cognitive chain-of-thought to render visibly.


### HOW IT WORKS (User Action)
*The Presentation \How\: What the user actually does.*
- **Action:** Click the OS (Work) tab. Use this multi-turn agentic terminal to direct Pete to read files, rewrite scripts, or compile documents if you prefer a power-user experience.
- **Why:** This demystifies the theoretical \why\ into a direct, clickable interaction that drives the system forward.
