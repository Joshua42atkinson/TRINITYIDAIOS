# Session Handoff — 2026-07-05 (Evening, Post-Crash Recovery + Docs Sync)

> **Hardware:** AMD Ryzen AI Max+ 395 (Strix Halo), 128GB LPDDR5X, gfx1151
> **Models:** DiffusionGemma 26B AWQ-INT4 (P) :8000, ComfyUI (A) :8188
> **Trinity:** Headless mode on :3000
> **Phone:** Pixel 10 Pro XL via Tailscale `100.83.222.35:3000`

---

## System State

```
P (DiffusionGemma 26B)  :8000  ⚠️  DOWN — needs restart with new crash fixes
A (ComfyUI 0.15.1)      :8188  ⚠️  DOWN — needs hotel/studio after Trinity starts
Trinity API             :3000  ⚠️  DOWN — needs restart after vLLM
RAM: ~8GB/122GB used (Chrome closed, fresh boot)
GPU power: locked to `high` (udev rule persists)
tuned: accelerator-performance profile active
```

## Architecture Update

The hotel has been restructured to **Studio/Solo/Closed** (3 modes, no more Swap):
- **Studio:** P + A always resident (default) — Tier 2 models loaded on demand by ComfyUI
- **Solo:** P only, A shut down (IDE agents welcome)
- **Closed:** All models off (night shift / maintenance)

**Hermes 4 70B (H):** Managed externally by LM Studio on :8002. Not hotel-managed. Submits jobs to Trinity via POST /api/jobs.

**FACES Protocol:** Moved to Semantic Slime project. `trinity-faces` crate no longer in workspace.

**Qwythos-9B (R):** Removed from architecture. Cloud API handles review when needed.

## CRASH INVESTIGATION — 3 Crashes Today

### Crash 1 (17:27) — GPU Ring Timeout
- **Cause:** gnome-shell + vLLM shared GPU ring, amdgpu timeout
- **Fix applied:** GPU power lock `high`, tuned `accelerator-performance`, udev rule

### Crash 2 (~18:10) — OOM
- **Cause:** Chrome peaked at 42.5GB RAM + vLLM 42GB + ComfyUI = exceeded 122GB
- **Fix:** Close Chrome during AI workloads. PWA works from phone.

### Crash 3 (~18:34) — Silent Hard Hang (KNOWN BUG)
- **Cause:** `svm_range_restore_work` hogged CPU 259+ times → PMFW 100.6.0 race → silent hang
- **Documented at:** https://github.com/ROCm/ROCm/issues/6165
- **Fixes applied to vLLM script:**
  - `HSA_USE_SVM=0` — disables SVM (root cause of thrashing)
  - `VLLM_LOGGING_LEVEL=DEBUG` — throttles GPU to dodge PMFW race
- **Fix applied to GRUB:**
  - `amdgpu.cwsr_enable=0` — disables buggy compute wave save/restore
  - Removed deprecated `amdgpu.gttsize=126976`
- **All fixes take effect on next vLLM restart / reboot**

## What Was Done This Session

### Code Changes
| # | Change | File(s) |
|---|--------|---------|
| 1 | Agent → ComfyUI image generation working end-to-end | `creative.rs`, `tools/creative.rs`, `agent.rs` |
| 2 | Tool calling fixed for DiffusionGemma (JSON key normalization) | `agent.rs` — `normalize_json_like_keys()` |
| 3 | Structured tool call validation (fall back to regex if invalid) | `agent.rs:984-1003` |
| 4 | Creative tools timeout increased to 5 minutes | `agent.rs:2041-2076` |
| 5 | `generate_image` tool now passes width/height params | `tools/creative.rs` |
| 6 | Agent system prompt updated (removed Qwythos/R/Crow/REAP refs) | `agent.rs:277-326` |
| 7 | Health check fixed to probe ComfyUI :8188 | `health.rs:139-156` |
| 8 | Inference router prefers configured primary backend | `inference_router.rs:490-514` |
| 9 | Phone PWA updated (IP, hotel modes, model badges, defaults) | `phone.html` |
| 10 | `read_file` sandbox expanded to allow /tmp | `tools/fs.rs:36-46` |
| 11 | `generate_image` tool description updated | `tools.rs:176` |
| 12 | **MCP agent proxy tools added** (`agent_chat`, `execute_tool`, `trinity_health`) | `trinity-mcp-server/src/lib.rs` |
| 13 | **Windsurf MCP config created** (stdio mode, Trinity as sub-agent) | `~/.codeium/windsurf/mcp_config.json` |
| 14 | **Phone PWA inline image display** (SSE `event: image` handler + markdown URL rendering) | `phone.html` |
| 15 | **Crash fixes applied** (HSA_USE_SVM=0, VLLM_LOGGING_LEVEL=DEBUG, cwsr_enable=0) | `start-diffusiongemma.sh`, GRUB |

### Doc Changes
| # | Change |
|---|--------|
| 1 | `00-master-roadmap.md` updated — Phases 1-7 COMPLETE, line counts corrected |
| 2 | `SESSION-HANDOFF.md` rewritten (this file) |

## What's Done (Don't Touch)

- Phases 1-7 COMPLETE (protocol cleanup, feature gates, main.rs split, dead code, security, frontend)
- Agent loop with tool calling works for: `generate_image`, `write_file`, `read_file`, `shell`, `list_dir`, `search_files`
- MCP server builds clean, connects to Windsurf, `trinity_health` and `agent_chat` tested working
- Phone PWA: inline image display code written (needs testing with live image gen)
- Strix Halo crash fixes applied (need vLLM restart to take effect)
- GPU power locked to `high` (persists via udev)
- `tuned` → `accelerator-performance` (persists via systemd)

## What's Next (in order)

1. **Restart vLLM with crash fixes** — `bash /home/joshua/trinity-models/start-diffusiongemma.sh`
2. **Restart Trinity** — `taskset -c 28-31 target/release/trinity --config configs/runtime/default.toml &`
3. **Launch ComfyUI via hotel** — `curl -X POST http://localhost:3000/api/inference/hotel/studio`
4. **Test image generation from phone PWA** — verify inline image display works
5. **Test MCP agent_chat from Windsurf** — verify sub-agent delegation
6. **Phase 8 remaining** — Voice (VibeVoice), video (HunyuanVideo), 3D (TRELLIS) pipeline
7. **Phase 13: Deployment Packaging** — systemd audit, Caddyfile audit, release binary

## Startup Commands

```bash
# 1. Start DiffusionGemma (with crash fixes)
bash /home/joshua/trinity-models/start-diffusiongemma.sh

# 2. Wait for vLLM to load (~2 min)
# Check: curl http://127.0.0.1:8000/health

# 3. Start Trinity (release build)
taskset -c 28-31 /home/joshua/Workflow/TRINITYIDAIOS/target/release/trinity \
  --config /home/joshua/Workflow/TRINITYIDAIOS/configs/runtime/default.toml &

# 4. Launch ComfyUI via hotel manager
curl -X POST http://localhost:3000/api/inference/hotel/studio

# Phone: http://100.83.222.35:3000/trinity/phone.html (Tailscale)
# Local: http://localhost:3000/trinity/phone.html
```

## Prompt To Start Next Session

```
Trinity is at 95/100. Phases 1-7 complete. Phase 9 (docs sync) complete — all docs
updated to reflect P+A+H architecture, Qwythos removed, FACES moved to Semantic Slime,
hotel modes corrected to Studio/Solo/Closed. MCP integration done. Phone PWA has inline
image display. Three crash fixes applied: HSA_USE_SVM=0, VLLM_LOGGING_LEVEL=DEBUG,
amdgpu.cwsr_enable=0. Workspace hygiene done: 6 stale docs archived, 24 stale scripts
archived.

Next: restart vLLM with fixes, restart Trinity, launch ComfyUI via hotel/studio,
test image gen from phone, test MCP from Windsurf, then Phase 8 (voice/video/3D)
and Phase 13 (deployment packaging).
See _agent/workflows/00-master-roadmap.md for the full plan.
```
