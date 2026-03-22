# Trinity Future Ideas — Captured During Sessions
> Last Updated: 2026-03-22 12:05 EDT
> Status: Parked items — do NOT implement until Phase 6 graduation is complete

## 1. Full Hard Drive Refactor
**Problem**: Models (GGUF, ONNX, safetensors) scattered across public/private folders.
Large files in random locations make the workspace hard to navigate.

**Plan**: 
- Consolidate all models into `~/trinity-models/{gguf,onnx,comfyui,voice}/`
- Remove duplicates and dead checkpoints
- Update all paths in configs and launch scripts
- **Record the cleanup session with OBS as a demo** — Trinity doing the work

**When**: Next session after graduation. Could be a Trinity agentic demo.

---

## 2. Trinity Self-Recording (OBS Integration)
**Idea**: At user request, Trinity can record its work sessions via OBS CLI.
This creates demo videos showing autonomous development in action.

**Implementation path**: `obs-cli` or `obs --startrecording` via tool call.
Could also use `ffmpeg` screen capture as a lighter alternative.

**Value**: Real proof-of-work for Purdue demos and user trust.

---

## 3. Mobile Push Notifications (Tethered App)
**Idea**: Trinity sends progress updates to the user's phone.
Videos of game updates, improvement screenshots, alignment requests.

**Implementation path**: 
- `ntfy.sh` (self-hosted push notifications, no app store needed)
- Signal CLI (encrypted, already on many phones)
- Simple webhook to a Telegram bot

**Value**: User stays in the loop without sitting at the desk.

---

## 4. Quest Queue — Super Prompts for Async Work
**Idea**: User and Mistral brainstorm together, building out the next several
workflows in different directions. The refined "super prompts" get stored
as queued quests. When compute is free, Trinity executes them autonomously.

**What already exists**:
- `task_queue.md` in Yardmaster (basic task list)
- `work_log` tool (records completed work)
- DAYDREAM archive (save/restore project states)
- Agentic loop in `agent.rs` (chained tool calls)

**What's needed**:
- Quest serialization to a durable queue (PostgreSQL or JSON file)
- Scheduler that picks up queued quests when LLM is idle
- Status API: `/api/quest/queue` — list pending, running, completed quests
- Mobile notification when a quest completes

**Value**: The user becomes the strategist, Trinity becomes the builder.

---

## 5. NPU Acceleration (Strix Halo XDNA 2)
**Idea**: Use the 50 TOPS NPU for speculative decoding (EAGLE), embedding generation, 
or Qianfan-OCR inference. Currently the NPU sits idle.

**Research done**: See `docs/research/TRINITY_INFERENCE_SYSTEMS_RESEARCH_PAPER.md`
**Blockers**: ONNX Runtime GenAI NPU support is model-limited, driver maturity

---

## 6. 3D Bevy Yard Restoration
**Idea**: Restore `trinity-bevy-graphics` (30K+ LOC) to full workspace membership.
**Blocker**: winit 0.30.13 + Rust 1.94 = 63 E0282 type inference errors.
**When**: Wait for winit 0.30.14+ or Bevy 0.19.

---

## 7. ~~Dual KV Cache Slots for Instant Persona Switching~~ ✅ DONE
**Implemented**: March 22, 2026
- `id_slot` field on `CompletionRequest` in `inference.rs`
- `persona_slot()` routing in `agent.rs`: recycler=0, programmer=1
- Auto-launch with `--parallel 2` in `main.rs`
- Config: `[inference.slots]` in `default.toml`
- 500K total context budget: 256K per slot

---

## Priority Order (Updated March 22)
1. ~~ART Studio gallery~~ ✅ DONE
2. ~~EYE Export system~~ ✅ DONE (HTML5 quiz/adventure/JSON)
3. ~~Researcher sub-agent~~ ✅ DONE (Qianfan-OCR)
4. ~~Shared HTTP client~~ ✅ DONE (20 duplications → 3 shared clients)
5. ~~Dual persona~~ ✅ DONE (Great Recycler / Programmer Pete)
6. ~~Dual KV cache slots~~ ✅ DONE (id_slot routing + --parallel 2)
7. Audio input pipeline (voice → Pete)
8. Hard drive refactor (next cleanup session)
9. Quest queue system (after graduation)
10. NPU acceleration (research project)
11. Mobile notifications (month+ out)
12. OBS recording (whenever convenient)
13. 3D Bevy Yard (blocked by winit)
