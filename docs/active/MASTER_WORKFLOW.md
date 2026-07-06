# Trinity Creative Studio — Production Roadmap
## Updated: July 5, 2026

> **Hardware:** AMD Ryzen AI Max+ 395 (Strix Halo), 128GB LPDDR5X, gfx1151
> **Architecture:** P (DiffusionGemma 42GB) + A (ComfyUI 17GB resident, 53GB Tier 2)
> **Tools:** Blender 5.0.1, Godot 4.4.1, Cascade/Windsurf IDE
> **Vision:** Local-first creative production studio for games, videos, and stories

---

## PIPELINE STATUS

| Stage | Status | Verified |
|-------|--------|----------|
| ✅ P: DiffusionGemma 26B | Resident | :8000 podman, 1M context |
| ✅ A: ComfyUI + Janus-Pro-7B | Resident | :8188, 3 nodes loaded |
| ✅ A: VibeVoice-1.5B | Installed | :8188, 6 nodes loaded |
| ✅ ESRGAN 4x upscale | Working | RealESRGAN_x4.pth, 1024→4096 |
| ✅ LongCat-Image (Tier 2) | Verified | E2E test passed, 11MB output |
| ✅ HunyuanVideo (Tier 2) | Available | ComfyUI node loaded |
| ✅ TRELLIS (Tier 2) | Available | 16GB, glTF export |
| ✅ FLUX Q4 (Tier 2) | Available | 6.4GB |
| ✅ TripoSR (Tier 2) | Available | 1.6GB |
| ⚠️ ACE-Step (Tier 2) | Installed | Needs torchaudio fix |
| ✅ Hotel Studio mode | Verified | POST /hotel/studio launches A in 7s |
| ✅ Trinity release build | Clean | 39MB binary, 65s compile |
| ✅ Godot 4.4.1 | Installed | /usr/local/bin/godot |
| ✅ Blender 5.0.1 | Installed | system-wide |
| 🔨 Janus-Pro generation test | Pending | Need to verify on ROCm |
| 🔨 VibeVoice generation test | Pending | Need to verify multi-speaker |
| � Full pipeline E2E | Pending | story→art→voice→video |
| � Godot OpenXR plugin | Pending | For VR development |
| � Product crate separation | Pending | Move middleware out of core Cargo.toml |

---

## COMPLETED

### Infrastructure ✅
- Hotel restructured: Studio/Solo/Closed modes, P+A always resident
- `always_resident` field added to HotelGuestConfig
- `/hotel/team` → `/hotel/studio` API endpoint
- ComfyUI upgraded to PyTorch 2.9.1+rocm7.2.0, Triton 3.5.1
- ROCm 7.2 compute libs installed (libMIOpen, rocblas, roctracer, etc.)
- AMD audio fix applied (`.float().cpu().numpy()`)
- Critical env vars: HSA_OVERRIDE_GFX_VERSION=11.5.1, HSA_ENABLE_SDMA=0, HSA_USE_SVM=0
- Critical flags: --disable-mmap --bf16-vae --cache-none

### Models ✅
- Janus-Pro-7B (14GB) downloaded, ComfyUI nodes loaded
- VibeVoice-ComfyUI installed, 6 nodes loaded
- ESRGAN RealESRGAN_x4.pth downloaded
- 6 redundant models deleted (CogVideoX, Hummingbird, SDXL, Qwythos, zen-musician, gemma4-coding) — 46GB freed

### Dependencies ✅
- rust-bert removed from MCP server (was unused, pulled in 2GB libtorch)
- Build time: 111s → 65s (40% faster)
- MCP server binary: smaller, no torch dependency

### Identity & Docs ✅
- TRINITY_IDENTITY.md written — 3-tier architecture (Core/Middleware/Product)
- Interface contract defined (network-only, one-way deps, state isolation)
- 8 stale docs deleted (LM_STUDIO_SETUP, context_old, lesson_plans, etc.)
- HOW_TO_USE_TRINITY.md rewritten for creators
- PARTY_FRAMEWORK.md rewritten as Studio Framework
- TRINITY_QUICKSTART.md rewritten for studio startup

---

## REMAINING WORK — In Order

### Phase 1: Verify Creative Pipeline (~30 min)
**Goal:** Test Janus-Pro + VibeVoice end-to-end on ROCm.

1. Queue Janus-Pro image generation via ComfyUI API
2. Queue VibeVoice single-speaker TTS
3. Verify both complete without ROCm errors
4. If Janus-Pro fails, fall back to LongCat (already verified)

### Phase 2: First Creative Output (~1 hour)
**Goal:** Produce one complete creative asset using the full pipeline.

1. P writes a 500-word LitRPG scene
2. Janus-Pro generates 2 illustrations → ESRGAN upscale
3. VibeVoice generates narration
4. Save all assets to project workspace
5. This is the first "product" of the studio

### Phase 3: Product Crate Separation (~2 hours)
**Goal:** Remove middleware dependencies from Trinity Core.

1. Remove `trinity-iron-road`, `trinity-quest`, `trinity-voice` from main crate Cargo.toml
2. Feature-gate `docx-rs`, `zip`, `rodio` behind `export` feature
3. Verify core builds without middleware
4. Middleware crates remain in workspace but are optional deps

### Phase 4: Godot VR Prototype (~3 hours)
**Goal:** First VR prototype with TRELLIS-generated assets.

1. Generate 3D model with TRELLIS from a Janus-Pro image
2. Export as glTF
3. Import into Godot 4.4.1
4. Set up OpenXR scene
5. Test in VR headset (if available) or desktop mode

### Phase 5: First YouTube Video (~2 hours)
**Goal:** Animated educational short with Trinity-generated content.

1. P writes script (educational topic)
2. Janus-Pro generates 3-4 illustrations
3. VibeVoice generates narration
4. Blender: combine illustrations + narration into video
5. Export and review

---

## DAILY WORKFLOW

```bash
# One-command startup
bash ~/trinity-models/start-diffusiongemma.sh
~/Workflow/TRINITYIDAIOS/target/release/trinity --headless &
curl -X POST http://localhost:3000/api/inference/hotel/studio

# Studio is now live:
#   P (story/code)  → http://localhost:3000
#   A (art/voice)   → http://localhost:8188
#   Blender         → blender
#   Godot           → godot
```

---

## DECISION LOG

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-07-05 | Restructured hotel to Studio/Solo/Closed | P+A resident, R/T removed, cloud handles review |
| 2026-07-05 | Janus-Pro-7B replaces LongCat as resident image model | 14GB vs 28GB, decoupled vision, 99% positional accuracy |
| 2026-07-05 | VibeVoice replaces Kokoro for TTS | 1.5B vs 82M, multi-speaker, 90-min capacity |
| 2026-07-05 | Removed rust-bert from MCP server | Unused, pulled in 2GB libtorch, 40% build speedup |
| 2026-07-05 | 3-tier architecture (Core/Middleware/Product) | Clear boundaries, Core never depends on Product |
| 2026-07-05 | Deleted 6 redundant models | 46GB freed, simplified inventory |
| 2026-07-05 | Godot 4.4.1 installed | VR game development with OpenXR |
| 2026-07-03 | Removed 9 dead modules (2,861 lines) | scope_creep broken, perspective caused timeouts |
| 2026-07-03 | PWA instead of Bevy NDK | PWA is instant, NDK needs APK rebuild |

---

*This document is the single source of truth for production status.*
*Update after each phase completion.*
