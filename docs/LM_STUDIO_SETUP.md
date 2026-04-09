# 🧠 LM Studio Setup — Recommended Settings for Trinity

> Trinity uses a **dual-persona AI system** (Great Recycler + Programmer Pete) that requires specific LM Studio settings to work optimally. This guide gives you the right settings for your hardware.

---

## Why LM Studio?

- **Free** — no API costs, no subscriptions
- **Local** — your data never leaves your machine (FERPA/COPPA safe)
- **Fast** — runs directly on your GPU/CPU
- **Compatible** — Trinity auto-detects LM Studio on port 1234

---

## Quick Setup (All Hardware)

1. **Download**: [lmstudio.ai](https://lmstudio.ai) → Install
2. **Load a model** (see hardware tiers below)
3. **Start the server**: Local Server tab → **Start Server**
4. **Verify**: Open `http://localhost:1234/v1/models` in your browser — you should see your model listed

---

## Hardware Tiers

### 🟢 Tier 1: Consumer Laptop (8-16 GB RAM)

**Best for**: Quick demos, basic lesson planning, testing Trinity

| Setting | Value |
|---------|-------|
| **Model** | `phi-4-mini-instruct` (Q4_K_M) — ~2 GB |
|  | *or* `qwen2.5-7b-instruct` (Q4_K_M) — ~4 GB |
| **Context Window** | `8192` |
| **Max Response Tokens** | `4096` |
| **Flash Attention** | `Enabled` |
| **GPU Offload** | All layers if your GPU has ≥4 GB VRAM |

**What works at this tier:**
- ✅ Chat with Pete (Socratic questioning)
- ✅ Basic lesson plan generation
- ✅ Quality scorecard evaluation
- ⚠️ Limited multi-step tool use
- ❌ Long-context recall (context too small)

---

### 🟡 Tier 2: Workstation / Gaming PC (24-32 GB RAM)

**Best for**: Full instructional design sessions, lesson plan creation

| Setting | Value |
|---------|-------|
| **Model** | `mistral-small-24b-instruct` (Q4_K_M) — ~14 GB |
| **Context Window** | `32768` (32K tokens) |
| **KV Cache Quantization** | `Q8_0` |
| **Max Response Tokens** | `8192` |
| **Flash Attention** | `Enabled` |
| **GPU Offload** | All layers (`99`) |

**What works at this tier:**
- ✅ Everything from Tier 1
- ✅ Full ADDIECRAPEYE lifecycle
- ✅ Code generation and lesson scaffolding
- ✅ Multi-step agentic workflows
- ⚠️ Context fills up in very long sessions

---

### 🔴 Tier 3: Power User (64-128 GB RAM)

**Best for**: Full-power Trinity, overnight autonomous generation, massive context

| Setting | Value | Why |
|---------|-------|-----|
| **Model** | `mistral-small-4-119b-instruct` (Q4_K_M) — ~68 GB | Best reasoning for ed-tech |
| **Context Window** | `256000` (256K tokens) | Deep session memory — entire PEARLs fit |
| **KV Cache Quantization** | `Q8_0` | Halves VRAM for cache vs f16 |
| **Max Response Tokens** | `32768` (32K) | Pete needs room for code + lesson plans |
| **Max Agentic Turns** | `65` | Allows autonomous multi-step builds |
| **Flash Attention** | `Enabled` | Required for 256K context |
| **GPU Offload** | All layers (`99`) | Full acceleration |

**What works at this tier:**
- ✅ Everything
- ✅ 2M+ effective token context (mmap deferred)
- ✅ Overnight autonomous generation
- ✅ Full 28 game mechanics running
- ✅ 40+ tokens/second inference

---

## Understanding the Dual-Persona System

Trinity runs **two AI personas** sharing one model:

| Persona | Role | Behavior |
|---------|------|----------|
| **Great Recycler** 🔮 | The Socratic mentor | Asks WHY, challenges assumptions, guides reflection. **Never produces deliverables.** |
| **Programmer Pete** ⚙️ | The executor | Builds lesson plans, rubrics, code. Produces real artifacts. |

They alternate in a **breathe in / breathe out** pattern:
- **Recycler breathes IN** → questioning, metacognition
- **Pete breathes OUT** → deliverables, execution

This pattern requires sustained context to work well. **The larger your context window, the better the dual-persona system performs** because both personas share conversation memory.

---

## LM Studio Server Settings

Under the **Local Server** tab, verify these settings:

| Setting | Recommended |
|---------|-------------|
| **Server Port** | `1234` (default — Trinity expects this) |
| **CORS** | `Enabled` |
| **Request Queuing** | `Enabled` |
| **Chat Endpoint** | `/v1/chat/completions` (default) |
| **Models Endpoint** | `/v1/models` (default) |

---

## Alternative Backends

Trinity is **backend-agnostic**. If you prefer a different tool:

| Backend | Port | How to Start |
|---------|:----:|-------------|
| **LM Studio** | 1234 | GUI app → Start Server |
| **Ollama** | 11434 | `ollama serve && ollama run mistral-small` |
| **longcat-sglang** | 8080 | `longcat-sglang -m MODEL.gguf --port 8080 -fa` |
| **Any OpenAI-compatible** | Custom | Set your endpoint in Trinity's setup wizard |

Trinity's `InferenceRouter` auto-detects all of these at startup, no configuration needed.

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| LM Studio says "Port in use" | Another instance is running. Close it or use a different port. |
| Model loads then crashes | You've run out of RAM. Try a smaller model or reduce context window. |
| Trinity says "INFERENCE_OFFLINE" | LM Studio server isn't started. Go to Local Server → Start Server. |
| Very slow responses | Reduce context window size, or use a smaller model. |
| "Out of memory" during long sessions | Restart LM Studio to clear the KV cache. |

---

## Model Download Links

For convenience, here are direct HuggingFace links:

| Model | Size | RAM Needed | Link |
|-------|:----:|:----------:|------|
| Phi-4 Mini | ~2 GB | 8 GB | [HuggingFace](https://huggingface.co/microsoft/phi-4-mini-instruct) |
| Qwen 2.5 7B | ~4 GB | 16 GB | [HuggingFace](https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF) |
| Mistral Small 24B | ~14 GB | 24 GB | [HuggingFace](https://huggingface.co/mistralai/Mistral-Small-24B-Instruct-2501) |
| Mistral Small 119B | ~68 GB | 128 GB | [HuggingFace](https://huggingface.co/mistralai/Mistral-Small-4-119B-Instruct-2503) |

> **Tip**: Always download the **Q4_K_M** quantization for the best quality-to-size ratio.

---

*For the full Trinity documentation, see [PROFESSOR.md](PROFESSOR.md) (institutional guide) or [README.md](README.md) (developer guide).*
