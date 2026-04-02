# ART SidECAR — Creative Studio for Bevy Developers

> **A complete AI-powered creative studio for game development, running locally on AMD Strix Halo**

---

## What is ART SidECAR?

ART SidECAR is a multi-modal AI system designed for Bevy game developers. It provides:

- **🧠 Brain** — Reasoning, planning, prompt generation
- **🔧 Hands** — Code generation with Bevy expertise
- **🖼️ Images** — SDXL Turbo diffusion via ComfyUI
- **🎲 3D** — Trellis mesh generation (optional)

All running on unified memory (ROCm/Vulkan) with no cloud dependency.

---

## Quick Start

```bash
# Start the full studio
./scripts/start-art-sidecar.sh

# Start without ComfyUI
./scripts/start-art-sidecar.sh --no-comfy

# Start with 3D mesh generation
./scripts/start-art-sidecar.sh --with-3d
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    ART SIDECAR                           │
├─────────────────────────────────────────────────────────┤
│  TEXT LLMs (llama.cpp + Vulkan/ROCm)                    │
│  ├─ Brain:  Crow-9B-Opus-Distill    Port 8081           │
│  │           ↓ Reasoning, planning, prompts              │
│  └─ Hands:  Qwen-25B-Rust-Coder    Port 8083            │
│              ↓ Code generation, Bevy expertise           │
│                                                         │
│  DIFFUSION (ComfyUI + ROCm)                             │
│  └─ Images: SDXL Turbo              Port 8188           │
│              ↓ 4-step fast generation                   │
│                                                         │
│  3D MESH (Trellis + Python)                             │
│  └─ Meshes: Image → GLTF            Port 8189           │
│              ↓ Game-ready 3D assets                     │
└─────────────────────────────────────────────────────────┘
```

---

## Endpoints

| Service | Port | Endpoint | Purpose |
|---------|------|----------|---------|
| Brain | 8081 | `/v1/chat/completions` | Reasoning, planning |
| Hands | 8083 | `/v1/chat/completions` | Code generation |
| ComfyUI | 8188 | `/api/*` | Image diffusion |
| Trellis | 8189 | `/generate` | 3D mesh generation |

---

## Example Workflows

### 1. Generate Bevy Code

```bash
# Ask Hands to generate Bevy code
curl -s http://localhost:8083/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "qwen-rust",
    "messages": [{
      "role": "user",
      "content": "Create a Bevy system that spawns a 3D camera with WASD controls"
    }],
    "max_tokens": 500
  }' | jq -r .choices[0].message.content
```

### 2. Generate Game Asset Prompt

```bash
# Ask Brain to create a prompt for ComfyUI
curl -s http://localhost:8081/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "crow",
    "messages": [{
      "role": "user",
      "content": "Create a detailed prompt for a fantasy sword icon, pixel art style, 64x64"
    }],
    "max_tokens": 200
  }' | jq -r .choices[0].message.content
```

### 3. Generate Image via ComfyUI

```bash
# Send prompt to ComfyUI (simplified)
curl -s http://localhost:8188/api/prompt \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": {
      "3": {
        "class_type": "KSampler",
        "inputs": {
          "seed": 12345,
          "steps": 4,
          "cfg": 1.0,
          "sampler_name": "euler",
          "scheduler": "normal",
          "denoise": 1.0,
          "model": ["4", 0],
          "positive": ["6", 0],
          "negative": ["7", 0],
          "latent_image": ["5", 0]
        }
      }
    }
  }'
```

---

## Memory Requirements

| Component | Model | Memory | Context |
|-----------|-------|--------|---------|
| Brain | Crow-9B | 6GB + 3GB KV | 16K |
| Hands | Qwen-25B | 15GB + 2GB KV | 8K |
| ComfyUI | SDXL Turbo | 7GB | — |
| **Total** | — | **~33GB** | — |

**Recommended**: 64GB+ unified memory (Strix Halo 128GB ideal)

---

## Bevy Integration

### Rust Client Example

```rust
use reqwest::Client;
use serde_json::json;

pub struct ArtSidecar {
    client: Client,
    brain_url: String,
    hands_url: String,
}

impl ArtSidecar {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            brain_url: "http://localhost:8081".to_string(),
            hands_url: "http://localhost:8083".to_string(),
        }
    }

    /// Generate Bevy code
    pub async fn generate_code(&self, prompt: &str) -> Result<String, Error> {
        let response = self.client
            .post(format!("{}/v1/chat/completions", self.hands_url))
            .json(&json!({
                "model": "qwen-rust",
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 1000
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    /// Generate creative prompt for assets
    pub async fn generate_prompt(&self, description: &str) -> Result<String, Error> {
        let response = self.client
            .post(format!("{}/v1/chat/completions", self.brain_url))
            .json(&json!({
                "model": "crow",
                "messages": [{"role": "user", "content": description}],
                "max_tokens": 200
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
```

### Bevy Plugin (Future)

```rust
// Planned: bevy_art_sidecar plugin
use bevy::prelude::*;

pub struct ArtSidecarPlugin;

impl Plugin for ArtSidecarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ArtSidecar::new())
           .add_systems(Update, generate_assets);
    }
}

fn generate_assets(
    sidecar: Res<ArtSidecar>,
    mut asset_requests: EventReader<AssetRequest>,
) {
    for request in asset_requests.read() {
        // Generate code, prompts, or trigger image generation
    }
}
```

---

## Hardware Support

### Tested Configurations

| Hardware | Status | Notes |
|----------|--------|-------|
| AMD Strix Halo 128GB | ✅ Primary | Unified memory, ROCm/Vulkan |
| AMD RDNA3 + 32GB RAM | ✅ Supported | May need model swapping |
| NVIDIA RTX 4090 | ✅ Supported | Use CUDA build |
| Apple M2 Ultra | ⚠️ Untested | Metal backend |

### Build llama.cpp for Strix Halo

```bash
cd llama.cpp
cmake -B build-vulkan -DGGML_VULKAN=ON
cmake --build build-vulkan --config Release -j$(nproc)
```

### Build for ROCm (Alternative)

```bash
cmake -B build-rocm -DGGML_HIP=ON
cmake --build build-rocm --config Release -j$(nproc)
```

---

## Troubleshooting

### Models Not Loading

```bash
# Check model files
ls ~/trinity-models/gguf/*.gguf

# Check llama-server
~/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server --help
```

### ComfyUI Not Starting

```bash
# Check ComfyUI installation
ls ~/ComfyUI/main.py

# Start manually
cd ~/ComfyUI && python3 main.py --port 8188 --listen
```

### Out of Memory

```bash
# Flush page cache (Strix Halo)
sudo sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches'

# Reduce context size in start script
# -c 16384 → -c 8192
```

---

## Roadmap

- [ ] Voice integration (PersonaPlex)
- [ ] Music generation (ACE-Step)
- [ ] Video generation (CogVideoX)
- [ ] Bevy plugin crate
- [ ] Web UI dashboard
- [ ] Asset pipeline automation

---

## Contributing

ART SidECAR is part of the Trinity project. Contributions welcome:

1. Fork the repo
2. Create a feature branch
3. Submit a PR

---

## License

MIT License — Free for personal and commercial use

---

*Built with ❤️ for the Bevy community*
*Part of the Trinity Educational AI Project*
