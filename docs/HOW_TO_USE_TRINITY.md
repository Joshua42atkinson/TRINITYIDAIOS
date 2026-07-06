# Trinity Creative Studio — User Guide

> *Trinity is the factory. Your games, videos, and stories are the goods.*

**Version 2.0** — July 5, 2026

---

## Who Is This For?

This guide is for **creators** — game developers, storytellers, animators, and educators — who want to use Trinity to produce creative assets. You don't need to know Rust or AI internals. You need a creative idea and 10 minutes.

Trinity is a **local-first creative production studio OS**. It runs entirely on your machine — no cloud, no API fees, no data leaving your hardware (with one exception: optional external review).

---

## Before You Start

Trinity needs two services running:

1. **Trinity Server** (:3000) — the studio orchestrator (story, code, RAG, API)
2. **ComfyUI** (:8188) — the art department (images, voice, video, 3D)

### Quick Start

```bash
# 1. Start DiffusionGemma (P) via podman
bash ~/trinity-models/start-diffusiongemma.sh

# 2. Start Trinity + ComfyUI (A) in Studio mode
~/Workflow/TRINITYIDAIOS/target/release/trinity --headless
curl -X POST http://localhost:3000/api/inference/hotel/studio

# 3. Open your browser: http://localhost:3000
```

---

## The Studio Architecture

Trinity has two always-resident models and a pool of on-demand models:

| Role | Model | Port | What it does |
|------|-------|------|-------------|
| **P (Producer)** | DiffusionGemma 26B | :8000 | Story, script, code, game design, reasoning |
| **A (Art Dept)** | ComfyUI + Janus-Pro + VibeVoice | :8188 | Images, voice, video, 3D |

**On-demand models** (ComfyUI loads these when needed):
- HunyuanVideo (video clips), TRELLIS (3D assets), LongCat-Image (artistic illustrations)
- ACE-Step (music), FLUX Q4 (alt image), TripoSR (fast 3D)

---

## The Creative Pipeline

```
 0. INTENT   → You describe what you want (via PWA, API, or IDE)
 1. STORY    → P writes the script, dialogue, code, or design doc
 2. ART      → A generates images (Janus-Pro → ESRGAN 4x upscale)
 3. VOICE    → A generates multi-speaker narration (VibeVoice, 90-min capacity)
 4. VIDEO    → A generates animated clips (HunyuanVideo, on-demand)
 5. 3D       → A generates 3D models (TRELLIS → glTF → Godot, on-demand)
 6. MUSIC    → A generates soundtrack/SFX (ACE-Step, on-demand)
 7. ASSEMBLE → Blender edits video, Godot builds games (CPU, always available)
 8. REVIEW   → Cloud API gives external critique (optional, only step that leaves local)
 9. VAULT    → Assets saved to your project workspace folder
```

---

## Step 1: Start a Project

When Trinity opens, you'll see the **PWA** at `http://localhost:3000`.

Choose what you're making:
- **Game** — VR game, 2D game, interactive story
- **Video** — YouTube video, animated tutorial, short film
- **Story** — LitRPG chapter, narrative podcast, audiobook
- **Code** — Godot project, React app, Rust crate

Type your creative prompt. Example: *"A LitRPG story where the protagonist discovers that cognitive load theory applies to magic system design"*

---

## Step 2: Generate Story (P: DiffusionGemma)

P is your producer. It writes:
- **Stories**: Full narrative chapters with dialogue
- **Scripts**: YouTube video scripts with scene descriptions
- **Code**: GDScript for Godot, React components, Rust modules
- **Design docs**: Game design documents, level layouts, character sheets

Talk to P via the PWA chat or the API:
```bash
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Write a 500-word LitRPG scene where the hero learns about intrinsic cognitive load"}'
```

P has 1M context window — it can hold an entire novel in memory.

---

## Step 3: Generate Art (A: Janus-Pro-7B)

Once P writes the story, generate illustrations via ComfyUI:

```bash
# Queue an image generation via ComfyUI API
curl -X POST http://localhost:8188/prompt \
  -H "Content-Type: application/json" \
  -d '{"prompt": {"1": {"class_type": "JanusModelLoader", "inputs": {"model_name": "deepseek-ai/Janus-Pro-7B"}}, "2": {"class_type": "JanusImageGeneration", "inputs": {"model": ["1", 0], "processor": ["1", 1], "prompt": "A wizard studying spell diagrams in a glowing library", "seed": 42, "batch_size": 1, "cfg_weight": 5.0, "temperature": 0.8, "top_p": 0.9}}, "3": {"class_type": "SaveImage", "inputs": {"images": ["2", 0], "filename_prefix": "litrpg_scene"}}}}'
```

Janus-Pro generates at 1024x1024. Add ESRGAN for 4x upscale to 4096x4096.

**Janus-Pro can also understand images** — feed it a screenshot and it'll analyze it:
- "Does this UI follow CRAP principles?"
- "What's wrong with this character pose?"
- "Describe this fretboard diagram"

---

## Step 4: Generate Voice (A: VibeVoice-1.5B)

VibeVoice generates expressive multi-speaker audio:
- Up to **90 minutes** of continuous speech
- Up to **4 distinct speakers** in one generation
- **64K token context** — entire scripts in one pass

Available via ComfyUI nodes: `VibeVoiceSingleSpeakerNode`, `VibeVoiceMultipleSpeakersNode`

---

## Step 5: Generate Video (Tier 2: HunyuanVideo)

For animated scenes, ComfyUI loads HunyuanVideo on demand:
- 5-second video clips from text or image prompts
- ~5 minutes generation time per clip
- 720p output

---

## Step 6: Generate 3D Assets (Tier 2: TRELLIS)

For game development, TRELLIS generates 3D models from images:
- Input: a Janus-Pro generated image
- Output: glTF model → import directly into Godot
- ~16GB VRAM, loaded on demand

---

## Step 7: Assemble (Blender + Godot)

These run on CPU — they don't compete for VRAM:

- **Blender 5.0.1**: Video editing (VSE), compositing, 3D rendering
- **Godot 4.4.1**: VR game development with OpenXR, export to any platform

```bash
# Start Blender for video editing
blender

# Start Godot for game development
godot
```

---

## Step 8: Review (Cloud API, optional)

When you need external critique, use a cloud API. This is the **only step where data leaves your machine** — local models don't yet have the reasoning capacity for reliable final critique.

---

## Hotel Modes

| Mode | What's running | When to use |
|------|---------------|------------|
| **Studio** (default) | P + A always resident | Normal creative work |
| **Solo** | P only, A shut down | When IDE agents need VRAM |
| **Closed** | All models off | Night shift, full IDE mode |

```bash
# Switch modes
curl -X POST http://localhost:3000/api/inference/hotel/studio  # P + A
curl -X POST http://localhost:3000/api/inference/hotel/solo    # P only
curl -X POST http://localhost:3000/api/inference/hotel/close   # All off
```

---

## Tips for Getting the Most Out of Trinity

1. **Start with story, then art.** P writes faster than A generates images. Write the full scene first, then generate illustrations for key moments.

2. **Use ESRGAN for every image.** Janus-Pro at 1024x1024 is good. ESRGAN 4x to 4096x4096 is print-quality.

3. **Voice last.** Generate voice after the script is final. VibeVoice can do 90 minutes in one pass, but you don't want to regenerate because you changed one line.

4. **Video is episodic.** Generate clips one at a time. Don't try to generate an entire video in one go.

5. **Save to project folders.** Use the Vault step to save assets to organized project directories.

---

## Frequently Asked Questions

### "Is my data sent to the cloud?"
No. Trinity runs 100% locally. The only exception is Step 8 (Review), which is optional.

### "What if I don't have 128GB RAM?"
Trinity scales down. Use smaller models via LM Studio or Ollama. The pipeline works — just slower and with less capacity.

### "Can I use Trinity for commercial products?"
Yes. Trinity is the factory. What you produce with it is yours.

### "What genres work best?"
Any. Trinity has been tested with LitRPG, educational content, game prototypes, and music tutorials. The system adapts to your creative direction.

---

> **Trinity** — *Textbook · Reflective · Instructional · Narrative · Intelligence · Technology — Yours*
>
> *The studio awaits. Start creating.*
