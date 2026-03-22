# Trinity ID AI OS

**A gamified instructional design system that helps K-12 teachers build educational games using AI.**

Trinity combines instructional design methodology (ADDIE), visual design principles (CRAP), and metacognitive reflection (EYE) into a single AI-powered platform. Teachers describe what they want to teach — Trinity helps them build it.

---

## What It Does

| Capability | Description |
|------------|-------------|
| **Guided Game Design** | Walk through 12 instructional design phases (ADDIECRAPEYE) with Pete, an AI mentor |
| **Socratic Teaching** | Pete never gives direct answers — he asks questions that lead to discovery |
| **Lesson Plans** | Generate Bloom's-aligned lesson plans, rubrics, quizzes, and curriculum maps |
| **Image Generation** | Create visual assets via ComfyUI (SDXL Turbo) directly in conversation |
| **Document Intelligence** | Analyze documents, charts, and images with Qianfan-OCR Researcher sub-agent |
| **HTML5 Export** | Export self-contained quiz games and text adventures that run in any browser |
| **Bevy Game Scaffold** | Generate starter Rust/Bevy game projects pre-loaded with vocabulary and objectives |
| **Voice Conversation** | Talk to Pete using speech (Whisper STT + TTS pipeline) |

## Quick Start

### Prerequisites
- Linux system with 128GB+ unified RAM (developed on AMD Strix Halo)
- [llama.cpp](https://github.com/ggml-org/llama.cpp) built with Vulkan support
- [Mistral Small 4 119B](https://huggingface.co/mistralai/Mistral-Small-4-119B-2503) GGUF (Q4_K_M)
- PostgreSQL 15+ with pgvector extension
- Node.js 18+ and Rust 1.80+

### Run

```bash
# 1. Start the LLM (or let Trinity auto-detect and launch it)
llama-server -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on --jinja

# 2. Build and start Trinity
cd trinity-genesis
cargo build --release
cargo run --release

# 3. Open in browser
open http://localhost:3000
```

Trinity auto-detects running inference backends (llama-server, vLLM, Ollama, LM Studio, SGLang) and will auto-launch llama-server if none are found.

### Optional Sidecars

```bash
# Image generation (ComfyUI)
cd ~/ComfyUI && python main.py --port 8188

# Document intelligence (Qianfan-OCR Researcher)
llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768

# Voice pipeline
python scripts/voice_sidecar.py  # Port 8200
```

## Architecture

```
One brain, many modes.

Pete (Mistral 119B) ─── The AI personality
 ├── Aesthetics mode ─── CRAP visual design prompts
 ├── Research mode ───── Qianfan-OCR document intelligence
 └── Tempo mode ──────── Code generation, game scaffolding

Rust Axum Server (:3000) ─── ADDIECRAPEYE orchestration
React Frontend ──────────── Book-view UI, 3 modes (Iron Road / Express / Yardmaster)
PostgreSQL + pgvector ───── Sessions, RAG knowledge base
ComfyUI (:8188) ─────────── SDXL Turbo image generation
Voice (:8200) ───────────── Whisper STT + Kokoro TTS
```

## Codebase

| Component | Tests | Description |
|-----------|:-----:|-------------|
| `trinity` | 18 | Axum server, 26 agentic tools, inference router, creative pipeline, EYE export |
| `trinity-protocol` | 67 | Shared types, ADDIE lifecycle, PEARL focus agent, VAAM alignment |
| `trinity-quest` | 18 | Quest board, XP/Coal/Steam economy, 432+ objectives |
| `trinity-iron-road` | 16 | Iron Road narrative, Pete core, bestiary, MadLibs |
| `trinity-voice` | 10 | SSML injection, VAAM vocal emphasis |
| **Total** | **130** | **0 failures, 0 warnings** |

## The Name

**TRINITY = ID + AI + OS**
- **ID** = Instructional Design (ADDIE methodology)
- **AI** = Artificial Intelligence (CRAP visual design, automated by AI agents)
- **OS** = Operating System (EYE metacognitive layer — Envision, Yoke, Evolve)

## License

Apache 2.0 — Users own all content created with Trinity.

## Links

- [ConsciousFramework.com](https://consciousframework.com)
- [GreatRecycler.com](https://greatrecycler.com)
