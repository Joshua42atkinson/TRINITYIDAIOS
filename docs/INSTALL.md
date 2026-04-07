# 🔧 INSTALL — Trinity ID AI OS Setup Guide

> **For evaluators and new developers.** This guide walks you through every step, from zero to a running Trinity instance.

---

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **OS** | Linux (Ubuntu 22.04+, Fedora 38+) | Ubuntu 24.04 LTS |
| **RAM** | 16 GB | 128 GB unified (AMD Strix Halo) |
| **GPU VRAM** | 8 GB (basic chat only) | 24 GB+ or 128 GB unified |
| **Disk** | 20 GB (source + build) | 100 GB (with LLM model) |
| **Rust** | 1.80+ | Latest stable |
| **Node.js** | 18+ | 20 LTS |

---

## Step 1: Install Dependencies

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
```

### Node.js
```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.bashrc
nvm install 20
```

### LLM Backend (pick one)

**Option A: LM Studio (recommended)**
1. Download from [lmstudio.ai](https://lmstudio.ai)
2. Load a model (e.g., Mistral Small 4 119B, Llama 3.1 8B, or Qwen 2.5 7B)
3. Start the server on port 1234 (LM Studio's default)

**Option B: Ollama**
```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama serve  # starts on port 11434
ollama pull mistral-small  # or any model
```

**Option C: llama-server (manual)**
```bash
git clone https://github.com/ggml-org/llama.cpp.git
cd llama.cpp
cmake -B build -DGGML_VULKAN=ON
cmake --build build --config Release -j$(nproc)
sudo cp build/bin/llama-server /usr/local/bin/
```

---

## Step 2: Clone and Build

```bash
git clone https://github.com/Joshua42atkinson/trinity-genesis.git
cd trinity-genesis

# Build the React frontend
cd crates/trinity/frontend
npm install
npm run build
cd ../../..

# Build Trinity (release mode)
cargo build --release
```

---

## Step 3: Download an LLM Model

Trinity works with any GGUF model compatible with llama.cpp. The recommended model:

```bash
# Mistral Small 4 119B (Q4_K_M quantization, ~68 GB)
# Download from: https://huggingface.co/mistralai/Mistral-Small-4-119B-2503
mkdir -p ~/trinity-models/gguf
# Place the .gguf file(s) in ~/trinity-models/gguf/
```

For systems with less RAM, smaller models work too:
- **8 GB**: Phi-4 Mini (3.8B) or Qwen 2.5 7B
- **16 GB**: Mistral 7B or Llama 3.1 8B
- **24 GB**: Mistral Small 24B

---

## Step 4: Start Services

### Option A: Let Trinity auto-detect (recommended)
```bash
# Just start Trinity — the InferenceRouter will find your LLM backend
TRINITY_HEADLESS=1 cargo run -p trinity --release
```

### Option B: Manual LLM start
```bash
# Terminal 1: Start the LLM (if using llama-server)
llama-server -m ~/trinity-models/gguf/YOUR_MODEL.gguf \
  --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on --jinja

# Terminal 2: Start Trinity
TRINITY_HEADLESS=1 cargo run -p trinity --release
```

---

## Step 5: Open Trinity

```bash
xdg-open http://localhost:3000
```

### What You Should See

1. **Iron Road** — A 3-column book layout with chapter navigation, prose/chat area, and game HUD
2. **Character Sheet** — Your LDT Portfolio with cognitive metrics, academic progress, and artifact vault
3. **Yardmaster** — An agentic terminal where Pete (the AI mentor) can use 29+ tools
4. **Help Menu (❓)** — Links to the Four Chariots documentation

### Verify Health

```bash
curl http://localhost:3000/api/health
# Should return: {"status":"healthy","llm":{"connected":true,...},...}
```

---

## Optional: Sidecars

These are optional services for creative features:

```bash
# General AI and Media Engine (vLLM Omni handles Text + Images)
# Already integrated via InferenceRouter if running on port 8000.

# Document intelligence (Qianfan-OCR)
llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768

# Voice pipeline (Kokoro TTS - Apache 2.0)
./scripts/launch/start_kokoro_sidecar.sh  # Starts on port 8200
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `cargo build` fails | Ensure Rust 1.80+: `rustup update stable` |
| Frontend won't build | Ensure Node 18+: `node --version` |
| LLM not detected | Check your backend is running: `curl http://localhost:1234/v1/models` (LM Studio) or `curl http://localhost:11434/api/tags` (Ollama) |
| Out of memory | Use a smaller model or reduce `--ctx-size` |

---

## Project Structure

```
trinity-genesis/
├── crates/                    # Rust workspace
│   ├── trinity/               # Main server (Axum, tools, inference router)
│   ├── trinity-protocol/      # Shared types, ADDIE lifecycle, PEARL
│   ├── trinity-quest/         # Quest board, XP economy
│   ├── trinity-iron-road/     # Iron Road narrative, Pete core, bestiary
│   ├── trinity-voice/         # SSML, VAAM vocal emphasis
│   └── trinity-bevy-graphics/ # 3D vision (parked)
├── LDTAtkinson/               # Portfolio website (Vite + React)
├── configs/                   # Runtime configuration (TOML)
├── quests/                    # Quest definitions
├── migrations/                # PostgreSQL migrations
├── scripts/                   # Build, deploy, and utility scripts
├── TRINITY_FANCY_BIBLE.md     # Full technical reference ("The Bible")
├── ASK_PETE_FIELD_MANUAL.md   # Pete's persona & philosophy ("Field Manual")
├── PROFESSOR.md               # Stakeholder evaluation guide
├── README.md                  # Quick start and overview
├── INSTALL.md                 # This file
└── LICENSE                    # Apache 2.0 License
```

---

*For the full technical reference, see [TRINITY_FANCY_BIBLE.md](TRINITY_FANCY_BIBLE.md).*
*For the live demo, visit [https://LDTAtkinson.com](https://LDTAtkinson.com).*
