# Trinity Creative Studio — Quick Start

> **Start P → Start Studio → Open Browser → Create**
> Total time: ~2 minutes (models already downloaded)

---

## What You Need

| Requirement | Why |
|-------------|-----|
| **Trinity binary** | The studio orchestrator (Rust, pre-built) |
| **Podman** | Runs DiffusionGemma (P) in a container |
| **ComfyUI** | The art department (images, voice, video, 3D) |
| **ROCm 7.2** | AMD GPU compute (gfx1151) |
| A browser | Chrome, Firefox, or Edge |

---

## Step 1: Start the Producer (P)

```bash
# Start DiffusionGemma 26B via podman (always-on, 42GB VRAM)
bash ~/trinity-models/start-diffusiongemma.sh

# Verify: should return model info
curl http://localhost:8000/v1/models
```

---

## Step 2: Start the Studio

```bash
# Start Trinity server
~/Workflow/TRINITYIDAIOS/target/release/trinity --headless

# Launch ComfyUI as always-resident (Studio mode)
curl -X POST http://localhost:3000/api/inference/hotel/studio

# Verify ComfyUI is up
curl http://localhost:8188/system_stats
```

---

## Step 3: Open the Studio

```
http://localhost:3000
```

### What You'll See

| Tab | What It Is |
|-----|-----------|
| **Studio** | Chat with P (DiffusionGemma) for story, code, design |
| **Art** | Generate images via Janus-Pro, voice via VibeVoice |
| **Yardmaster** | Agentic terminal — P can use tools, read/write files |
| **Hotel** | Switch between Studio / Solo / Closed modes |

---

## Step 4: Create Something

### Generate a story
Type in the chat: *"Write a 500-word LitRPG scene where the hero discovers a magic system based on cognitive load theory"*

### Generate art
```bash
curl -X POST http://localhost:8188/prompt \
  -H "Content-Type: application/json" \
  -d '{"prompt": {"1": {"class_type": "JanusModelLoader", "inputs": {"model_name": "deepseek-ai/Janus-Pro-7B"}}, "2": {"class_type": "JanusImageGeneration", "inputs": {"model": ["1", 0], "processor": ["1", 1], "prompt": "A wizard studying spell diagrams", "seed": 42, "batch_size": 1, "cfg_weight": 5.0, "temperature": 0.8, "top_p": 0.9}}, "3": {"class_type": "SaveImage", "inputs": {"images": ["2", 0], "filename_prefix": "my_art"}}}}'
```

### Generate voice
Use VibeVoice nodes in ComfyUI: `VibeVoiceSingleSpeakerNode` or `VibeVoiceMultipleSpeakersNode`

---

## Hotel Modes

```bash
# Studio mode (default): P + A always resident
curl -X POST http://localhost:3000/api/inference/hotel/studio

# Solo mode: P only, A shut down (free VRAM for IDE agents)
curl -X POST http://localhost:3000/api/inference/hotel/solo

# Closed: all models off (night shift, full IDE)
curl -X POST http://localhost:3000/api/inference/hotel/close
```

---

## Troubleshooting

| Problem | Solution |
|---------|---------|
| "P not detected" | Check podman: `podman ps`, verify :8000 is up |
| "ComfyUI not detected" | Check :8188, verify ROCm env vars are set |
| OOM / VRAM error | Switch to Solo mode, or close Tier 2 models in ComfyUI |
| Slow image generation | Janus-Pro (~30s) is faster than LongCat (~5min). Use Janus for drafts. |

---

## More Resources

- **[HOW_TO_USE_TRINITY.md](HOW_TO_USE_TRINITY.md)** — Full user guide
- **[TRINITY_IDENTITY.md](TRINITY_IDENTITY.md)** — Architecture boundaries (Core vs Middleware vs Product)
- **[PARTY_FRAMEWORK.md](PARTY_FRAMEWORK.md)** — Studio framework and model taxonomy
- **[INSTALL.md](INSTALL.md)** — Building from source

---

*Questions? Open an issue at [github.com/Joshua42atkinson/TRINITYIDAIOS](https://github.com/Joshua42atkinson/TRINITYIDAIOS)*
