---
description: Phase 8 — End-to-end creative pipeline test: story → art → voice → assemble. Prove Trinity works as a studio.
---

# Phase 8: E2E Creative Pipeline Test

## Objective

Prove Trinity works as a creative production studio by producing one complete creative asset end-to-end: a 500-word LitRPG scene with illustrations and narration.

## Prerequisites

- Phases 3-6 complete (clean architecture, trimmed tools, security)
- `cargo check` passes
- DiffusionGemma (P) running on :8000
- ComfyUI (A) running on :8188 with Janus-Pro + VibeVoice nodes loaded

## Pipeline Steps

### Step 1: Start Trinity
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo run -p trinity --release -- --headless &
sleep 5
curl -s http://localhost:3000/api/health | python3 -m json.tool
```

### Step 2: Start ComfyUI (if not already running)
```bash
curl -X POST http://localhost:3000/api/inference/hotel/studio
sleep 10
curl -s http://localhost:8188/system_stats | python3 -m json.tool
```

### Step 3: Generate Story (P: DiffusionGemma)
```bash
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Write a 500-word LitRPG scene where the hero discovers that cognitive load theory applies to magic system design. Include vivid descriptions of the setting and character emotions.",
    "mode": "dev",
    "max_tokens": 2000
  }'
```
- Save the response to a file: `~/.local/share/trinity/workspace/projects/first_test/scene.txt`

### Step 4: Generate Art (A: Janus-Pro via ComfyUI)
```bash
# Queue image generation via ComfyUI API
curl -X POST http://localhost:8188/prompt \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": {
      "1": {"class_type": "JanusModelLoader", "inputs": {"model_name": "deepseek-ai/Janus-Pro-7B"}},
      "2": {"class_type": "JanusImageGeneration", "inputs": {
        "model": ["1", 0],
        "processor": ["1", 1],
        "prompt": "A wizard studying glowing spell diagrams in a vast library, magical energy swirling around them, fantasy art style",
        "seed": 42,
        "batch_size": 1,
        "cfg_weight": 5.0,
        "temperature": 0.8,
        "top_p": 0.9
      }},
      "3": {"class_type": "SaveImage", "inputs": {"images": ["2", 0], "filename_prefix": "litrpg_scene"}}
    }
  }'
```
- Wait for completion, check ComfyUI history for the output image
- Run ESRGAN upscale on the result

### Step 5: Generate Voice (A: VibeVoice via ComfyUI)
```bash
# Queue TTS via ComfyUI VibeVoice nodes
# Use the scene text from Step 3 as input
```

### Step 6: Assemble and Verify
- Verify all assets exist in the project workspace:
  - `scene.txt` (story)
  - `litrpg_scene_*.png` (art)
  - `narration_*.wav` (voice)
- This is the first "product" of the studio

### Step 7: Test via PWA
- Open `http://localhost:3000/trinity/phone.html`
- Send a chat message
- Generate an image via the UI
- Verify the full flow works from the browser

## Success Criteria

- [ ] Story generated (500+ words, coherent narrative)
- [ ] Art generated (1024x1024 image, matches scene description)
- [ ] Art upscaled (4096x4096 via ESRGAN)
- [ ] Voice generated (narration of the scene)
- [ ] All assets saved to project workspace
- [ ] PWA can trigger the full pipeline
- [ ] No crashes, panics, or hangs during the pipeline
- [ ] Total pipeline completes in under 10 minutes

## If Something Fails

- **P not responding**: Check vLLM/podman on :8000, check hotel status
- **ComfyUI not responding**: Check :8188, check if Janus-Pro nodes loaded
- **Image generation fails**: Fall back to LongCat-Image (already verified)
- **VibeVoice fails**: Check if torchaudio fix applied, fall back to Kokoro ORT
- **Agent loop crashes**: Check logs, the bug fixes from the previous session should prevent this

## Documentation

After successful completion:
- Save the generated assets as proof
- Update `MASTER_WORKFLOW.md` pipeline status to ✅ for E2E
- Update `SESSION-HANDOFF.md` with the successful test
