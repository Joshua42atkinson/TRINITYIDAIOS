# INSTALL — Trinity Creative Studio Setup Guide

> **For developers building Trinity from source.**

---

## System Requirements

| Component | Minimum | Recommended (Studio) |
|-----------|---------|---------------------|
| **OS** | Linux (Ubuntu 22.04+) | Ubuntu 24.04 LTS |
| **RAM** | 16 GB | 128 GB unified (AMD Strix Halo) |
| **GPU** | 8 GB VRAM | 120 GB unified (gfx1151, ROCm 7.2) |
| **Disk** | 20 GB (source + build) | 100 GB (with models) |
| **Rust** | 1.80+ | Latest stable |
| **Podman** | Required (runs P) | Latest |
| **ROCm** | 6.0+ | 7.2 (for gfx1151) |

---

## Step 1: Install Dependencies

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
```

### Podman (for DiffusionGemma)
```bash
sudo apt install podman
```

### ROCm 7.2 (for AMD Strix Halo gfx1151)
```bash
# Add ROCm repo
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] https://repo.radeon.com/rocm/apt/7.2 noble main" | sudo tee /etc/apt/sources.list.d/rocm.list
sudo apt update
sudo apt install rocm-core hsa-rocr rocm-smi libroctx64-4
```

### ComfyUI (the Art Department)
```bash
git clone https://github.com/comfyanonymous/ComfyUI.git ~/ComfyUI
cd ~/ComfyUI
python3 -m venv venv
source venv/bin/activate
pip install torch torchvision --index-url https://repo.radeon.com/rocm/manylinux/rocm-rel-7.2/
pip install transformers huggingface-hub safetensors accelerate
```

---

## Step 2: Clone and Build

```bash
git clone https://github.com/Joshua42atkinson/TRINITYIDAIOS.git
cd TRINITYIDAIOS

# Build the frontend
cd crates/trinity/frontend
npm install && npm run build
cd ../../..

# Build Trinity (release mode)
cargo build --release
```

---

## Step 3: Download Models

### P: DiffusionGemma 26B (always-on, podman)
```bash
mkdir -p ~/trinity-models
# The start script pulls the model automatically via podman
# See: ~/trinity-models/start-diffusiongemma.sh
```

### A: Janus-Pro-7B + VibeVoice (ComfyUI resident)
```bash
# Janus-Pro-7B
cd ~/ComfyUI
python -c "from huggingface_hub import snapshot_download; snapshot_download('deepseek-ai/Janus-Pro-7B', local_dir='models/Janus-Pro-7B', allow_patterns=['*.bin','*.json','*.txt','*.model'])"

# VibeVoice ComfyUI node
cd custom_nodes
git clone https://github.com/Enemyx-net/VibeVoice-ComfyUI.git
pip install -r VibeVoice-ComfyUI/requirements.txt

# ESRGAN upscale model
python -c "from huggingface_hub import hf_hub_download; import shutil; shutil.copy2(hf_hub_download('ai-forever/Real-ESRGAN','RealESRGAN_x4.pth'), 'models/upscale_models/RealESRGAN_x4.pth')"
```

### Tier 2 models (optional, loaded on demand)
```bash
# LongCat-Image (artistic illustrations)
# HunyuanVideo (video clips)
# TRELLIS (3D assets)
# ACE-Step (music)
# FLUX Q4 (alt image)
# TripoSR (fast 3D)
# Download as needed — ComfyUI loads them on demand
```

---

## Step 4: Start the Studio

```bash
# 1. Start P (DiffusionGemma) via podman
bash ~/trinity-models/start-diffusiongemma.sh

# 2. Start Trinity server
~/Workflow/TRINITYIDAIOS/target/release/trinity --headless &

# 3. Launch ComfyUI as always-resident (Studio mode)
curl -X POST http://localhost:3000/api/inference/hotel/studio

# 4. Open browser
xdg-open http://localhost:3000
```

### Verify Health
```bash
curl http://localhost:3000/api/health        # Trinity server
curl http://localhost:8000/v1/models         # P (DiffusionGemma)
curl http://localhost:8188/system_stats      # A (ComfyUI)
```

---

## Optional: Creative Tools

```bash
# Blender (video editing, 3D compositing)
sudo apt install blender

# Godot 4.x (VR game development)
wget https://github.com/godotengine/godot/releases/download/4.4.1-stable/Godot_v4.4.1-stable_linux.x86_64.zip
unzip Godot_v4.4.1-stable_linux.x86_64.zip -d /tmp/godot
sudo mv /tmp/godot/Godot_v4.4.1-stable_linux.x86_64 /usr/local/bin/godot
sudo chmod +x /usr/local/bin/godot
```

---

## Troubleshooting

| Problem | Solution |
|---------|---------|
| `cargo build` fails | Ensure Rust 1.80+: `rustup update stable` |
| Frontend won't build | Ensure Node 18+: `node --version` |
| P not detected | Check podman: `podman ps`, verify :8000 |
| ComfyUI not detected | Check :8188, verify ROCm env vars |
| OOM / VRAM error | Use Solo mode: `curl -X POST localhost:3000/api/inference/hotel/solo` |
| ROCm import errors | Verify `/opt/rocm/lib` in ldconfig, check `HSA_OVERRIDE_GFX_VERSION=11.5.1` |

---

## Project Structure

```
TRINITYIDAIOS/
├── crates/
│   ├── trinity/               # Core: main server, hotel, inference, creative, RAG
│   ├── trinity-protocol/      # Core: shared types
│   ├── trinity-mcp-server/    # Core: IDE integration
│   # (trinity-faces moved to Semantic Slime project July 2026)
│   ├── trinity-iron-road/     # Middleware: ADDIECRAPEYE framework
│   ├── trinity-quest/         # Middleware: XP/quest system
│   ├── trinity-voice/         # Middleware: SSML/VAAM
│   └── trinity-daydream/      # Middleware: Bevy 3D sandbox
├── configs/                   # Runtime configuration (TOML)
├── docs/                      # Documentation
│   └── TRINITY_IDENTITY.md    # Architecture boundary (read this first)
└── scripts/                   # Build and utility scripts
```

---

## Architecture

See [TRINITY_IDENTITY.md](TRINITY_IDENTITY.md) for the authoritative definition of:
- **Core** (what Trinity needs to function)
- **Middleware** (reusable libraries for products)
- **Products** (end-user apps that use Trinity)
- **Interface Contract** (how they communicate)
