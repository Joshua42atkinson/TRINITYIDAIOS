# PLAN 2: Hygiene
## Trinity ID AI OS — Workspace Cleanup & Intent Alignment

---

## 1. The Problem

After multiple sessions with different AI models, the codebase has accumulated:
- **15 empty Rust files** that were declared but never implemented
- **Stale model references** (Nemotron, GPT-OSS as defaults) that no longer match the actual PARTY
- **Mismatched labels** in the model inventory (labels pointing to wrong file paths)
- **Orphaned docs** referencing deprecated architectures
- **Duplicate implementations** across archive and active crates

This plan cleans the workspace so a cheaper, faster model can execute Plan 3 without getting confused by stale context.

---

## 2. Empty Files Audit

These 15 files exist but are empty (0 bytes). Each needs a decision: **implement**, **stub with intent comment**, or **delete**.

| File | Decision | Rationale |
|------|----------|-----------|
| `trinity-addie/src/skills.rs` | **Stub** | Skills tracked in `character_sheet.rs` already. Add re-export comment. |
| `trinity-data/src/embeddings.rs` | **Stub** | Embedding generation needed for Qdrant. Stub with fastembed intent. |
| `trinity-data/src/qdrant/mod.rs` | **Delete dir** | Qdrant client exists in `rag/vector_db.rs`. This is a duplicate path. |
| `trinity-data/src/qdrant/vector_db.rs` | **Delete** | Same as above — real impl is in `rag/vector_db.rs`. |
| `trinity-data/src/surreal/graph_rag.rs` | **Delete dir** | SurrealDB client exists in `rag/graph_rag.rs`. Duplicate path. |
| `trinity-data/src/surreal/mod.rs` | **Delete** | Same as above. |
| `trinity-eye/src/prompts.rs` | **Stub** | Vision prompts for screenshot analysis. Stub with intent. |
| `trinity-eye/src/quest.rs` | **Stub** | Quest-specific vision tasks. Stub with intent. |
| `trinity-iron-road/src/book.rs` | **Implement (Plan 3)** | Core Iron Road narrative — append-only chapter ledger. |
| `trinity-iron-road/src/great_recycler.rs` | **Implement (Plan 3)** | NPU/background synthesis of journal → book chapters. |
| `trinity-iron-road/src/narrative.rs` | **Implement (Plan 3)** | LitRPG prose generation from quest events. |
| `trinity-render/src/dockable.rs` | **Stub** | Bevy dockable UI — deferred to Layer 3. |
| `trinity-render/src/graphics.rs` | **Stub** | Bevy graphics — deferred to Layer 3. |
| `trinity-render/src/screens.rs` | **Stub** | Bevy screen management — deferred to Layer 3. |
| `trinity-render/src/ui.rs` | **Stub** | Bevy UI components — deferred to Layer 3. |

---

## 3. Stale Model References to Fix

| Location | Current (Wrong) | Correct |
|----------|----------------|---------|
| `conductor_leader.rs` comment line 11 | "Nemotron/Step-Flash" | "Mistral Small 4 / Ming" |
| `conductor_leader.rs` comment line 165 | "Qwen3.5-97B-A10B" | "Mistral Small 4 119B MoE" |
| `conductor_leader.rs` error msg line 265-268 | "Qwen3.5-97B-A10B REAP model not found" | "Mistral Small 4 not found" |
| `tools.rs` sidecar_status label | "Base Brain (port 8080)" | "Conductor Pete (port 8080)" |
| `pete_core.rs` model field | "mistralai/Mistral-Small-24B-Instruct-2501" | Should use config, not hardcoded model name |
| `vllm_client.rs` Brain impl | Hardcodes "Qwen2.5-97B-Instruct" | Should be configurable |
| `trinity-inference/src/lib.rs` comment | "ProductionBrain (97B inference)" | Outdated — update to current PARTY |
| Various `mini_bible_*.md` files | Reference old model names | Update to current PARTY roster |

---

## 4. Duplicate/Orphan Code to Consolidate

| Item | Location | Action |
|------|----------|--------|
| `qdrant/` and `surreal/` empty dirs | `trinity-data/src/` | Delete — real impls are in `rag/` |
| `memory_tracker_rusqlite_backup.rs` | `trinity-inference/src/` | Move to archive |
| `sidecar_manager_old.rs` | `trinity-inference/src/` | Move to archive |
| `conductor_model_path()` | `main.rs` line 140 | Delete — no longer used (was for Nemotron) |
| `voice_model_path()` | `main.rs` line 144 | Keep but update comment — PersonaPlex path |
| `onnx_model_path()` | `main.rs` line 150 | Keep — will be used by NPU pipeline |

---

## 5. Documentation Alignment

| Document | Status | Action |
|----------|--------|--------|
| `01-ARCHITECTURE.md` | Partially accurate | Update §5 Party Formation with real models, §6 Iron Road with real mechanics |
| `02-IMPLEMENTATION.md` | Unknown accuracy | Audit against current code |
| `03-OPERATIONS.md` | Has .rej file (merge conflict) | Clean up, update launch commands |
| `04-MODELS.md` | Model cards outdated | Rewrite with real filesystem inventory |
| `TRINITY_TECHNICAL_BIBLE.md` | In Downloads, partially stale | Merge relevant sections into bible/ |
| `GAP_ANALYSIS_MARCH_18_2026.md` | Current | Keep as-is, reference from plans |
| `PYTHON_BRIDGE_STRATEGY.md` | Good strategy doc | Update model names to current PARTY |

---

## 6. Config/Environment Alignment

| Item | Current | Target |
|------|---------|--------|
| `LLAMA_URL` env var | Defaults to `:8080` | Correct — Pete serves here |
| `VLLM_URL` env var | Does not exist | Add to main.rs — Ming serves on `:8000` |
| `COMFYUI_URL` env var | Does not exist | Add — ComfyUI on `:8188` |
| `DATABASE_URL` | Exists | Correct — PostgreSQL |
| Model paths | Hardcoded in multiple places | Centralize in a `TrinityPaths` struct |

---

## 7. Execution Order

1. Delete empty duplicate directories (`qdrant/`, `surreal/`)
2. Add intent stubs to the 8 files that should remain but are empty
3. Fix all stale model references listed in §3
4. Move orphan files to archive
5. Update `04-MODELS.md` with real inventory
6. Clean up `03-OPERATIONS.md` merge conflict
7. Add `VLLM_URL` and `COMFYUI_URL` env vars to main.rs
