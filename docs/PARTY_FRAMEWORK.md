# The Studio Framework
**Creative Production Architecture for Trinity OS**

Trinity is a local-first creative production studio. It orchestrates AI models across a 10-step pipeline (Intent → Story → Art → Voice → Video → 3D → Music → Assemble → Review → Vault) to produce creative assets: games, videos, stories, and code.

The architecture follows a three-tier model: **Core (OS) → Middleware (Libraries) → Products (Apps)**. See [TRINITY_IDENTITY.md](TRINITY_IDENTITY.md) for the authoritative boundary definition.

---

### **P — Producer** (The Story Engine)
* **Model:** DiffusionGemma 26B A4B AWQ-INT4 (MoE)
* **Port:** :8000 (podman, always resident)
* **VRAM:** ~42 GB (16GB model + 26GB FP8 KV cache)
* **CPU:** threads 0-3 (CCX0, 2 cores)
* **Function:** Writes stories, scripts, dialogue, game design docs, GDScript/React/Rust code. 1M context window — can hold an entire novel. MoE architecture activates 4B of 26B params per token for fast context-switching across creative tasks.

### **A — Art Department** (The Media Studio)
* **Models:**
  1. Janus-Pro-7B FP16 (image generation + visual understanding, ~14 GB)
  2. VibeVoice-1.5B FP16 (multi-speaker TTS, 90-min capacity, ~3 GB)
* **Port:** :8188 (ComfyUI, always resident)
* **VRAM:** ~17 GB resident + up to 53 GB for Tier 2 models
* **CPU:** threads 4-7 (CCX0, 2 cores)
* **Function:** Janus-Pro generates images (1024x1024 → ESRGAN 4x → 4096x4096) and understands images (visual analysis, CRAP critique). VibeVoice generates expressive multi-speaker voice acting with 64K token context. Both run inside ComfyUI.

### **Tier 2 — On-Demand Models** (Loaded by ComfyUI)
* **HunyuanVideo** (~13 GB) — cinematic video clips, technique demonstrations
* **TRELLIS** (~16 GB) — 3D game assets → glTF → Godot
* **LongCat-Image** (~28 GB) — artistic illustrations (Troubadour mode)
* **ACE-Step** (~8 GB) — soundtrack, SFX, ambient audio
* **FLUX Q4** (~6.4 GB) — alternative image generation
* **TripoSR** (~1.6 GB) — fast 3D prototyping

### **Y — You** (The Director)
* **Tools:** Trinity PWA (:3000), Blender 5.0.1, Godot 4.4.1, Cascade/Windsurf IDE
* **CPU:** threads 24-31 (CCX1 SMT, 4 cores)
* **Function:** You provide creative direction. Trinity executes. Blender edits video. Godot builds games. IDE writes code. You make the decisions.

---

## The Hotel: Resource Management

Trinity cannot load all models simultaneously (120GB VRAM limit). The Hotel Manager handles this:

```
┌──────────────────────────────────────────────────────┐
│              TIER 1 — ALWAYS RESIDENT (59 GB)          │
│  P: DiffusionGemma 26B   (42 GB)  — Story, code      │
│  A: ComfyUI              (17 GB)  — Art, voice        │
│     ├─ Janus-Pro-7B      (14 GB)  — Images            │
│     └─ VibeVoice-1.5B     (3 GB)  — Voice             │
├──────────────────────────────────────────────────────┤
│              TIER 2 — ON-DEMAND (53 GB budget)         │
│  ComfyUI loads/unloads these as needed:               │
│  HunyuanVideo(13)  TRELLIS(16)  LongCat(28)          │
│  ACE-Step(8)  FLUX(6.4)  TripoSR(1.6)                │
├──────────────────────────────────────────────────────┤
│              HOST (8 GB)                               │
│  Trinity + OS + Blender + Godot + IDE                 │
│  threads 24-31 (CCX1 SMT)                             │
└──────────────────────────────────────────────────────┘
```

### Hotel Modes

| Mode | Running | VRAM | When |
|------|---------|------|------|
| **Studio** | P + A | 59GB + Tier 2 | Default creative work |
| **Solo** | P only | 42GB | IDE agents need VRAM |
| **Closed** | Nothing | 0GB | Night shift, full IDE |

---

## Pipeline-to-Task Rationale

The key insight: **different creative tasks demand different models**.

| Creative Task | Model | Why |
|--------------|-------|-----|
| **Story, script, code** | P: DiffusionGemma 26B | 1M context, MoE speed, tool calling |
| **Technical diagrams** | A: Janus-Pro-7B | 99% positional accuracy, decoupled vision pathways |
| **Artistic illustrations** | Tier 2: LongCat-Image | BF16, richer artistic style |
| **Character voices** | A: VibeVoice-1.5B | 4 speakers, 90-min, expressive |
| **Video clips** | Tier 2: HunyuanVideo | 720p, 5-second clips |
| **3D game assets** | Tier 2: TRELLIS | Image → glTF → Godot |
| **Music/SFX** | Tier 2: ACE-Step | Soundtrack generation |
| **Video editing** | CPU: Blender | No VRAM needed |
| **Game development** | CPU: Godot | No VRAM needed |

Using Janus-Pro for a 1024x1024 diagram (14GB, 30s) instead of LongCat (28GB, 5min) saves 14GB VRAM and 4.5 minutes. Using DiffusionGemma for code (42GB, instant) instead of cloud API saves money and keeps data local.

---

## Middleware (Ecosystem Libraries)

These crates are **not** Core, but products can depend on them:

| Middleware | Crate | Purpose |
|-----------|-------|---------|
| FACES Protocol | `faces-protocol` (moved to Semantic Slime) | Emotional detection for character AI |
| ADDIECRAPEYE | `trinity-iron-road` | 12-phase instructional design framework |
| Quest System | `trinity-quest` | XP, Coal/Steam economy, Hero's Journey |
| Voice/SSML | `trinity-voice` | VAAM vocal emphasis, PersonaPlex |
| Daydream | `trinity-daydream` | Bevy 3D sandbox for XR prototyping |

**Rule:** Core `Cargo.toml` must NEVER depend on Middleware. The dependency is one-way: Products → Middleware → Core types (`trinity-protocol`).
