# Trinity Industry-Grade Upgrade Plan
**Date:** March 22, 2026  
**Author:** Joshua + Antigravity research session  
**Status:** ALL PHASES COMPLETE ✅ — Phase 1 ✅, Phase 2 ✅, Phase 3 ✅, Phase 4 ✅  
**Hardware:** GMKtek EVO X2, AMD Ryzen AI Max+ 395 (Strix Halo), 128GB LPDDR5X-8000

> **Purpose:** This is the SINGLE SOURCE OF TRUTH for Trinity's upgrade path.  
> Everything from the March 22 research session is here. Do not scatter this across other docs.  
> Future agents: READ THIS FIRST before making changes.

---

## Table of Contents
1. [System Audit — What's On Disk RIGHT NOW](#1-system-audit)
2. [Architecture Decision — Hotel, Not Swarm](#2-architecture-decision)
3. [Static vs Hotswap Memory Map](#3-static-vs-hotswap)
4. [EAGLE Speculative Decoding — Two Paths](#4-eagle-speculative-decoding)
5. [NPU Strategy — Housekeeping, Not a Second Brain](#5-npu-strategy)
6. [Agent Upgrade — Phase 1 Quick Wins](#6-agent-upgrade-phase-1)
7. [Agent Upgrade — Phase 2 Native Function Calling](#7-agent-upgrade-phase-2)
8. [Agent Upgrade — Phase 3 Multi-Backend Inference Router](#8-agent-upgrade-phase-3)
9. [Educational Tools — Phase 4](#9-educational-tools)
10. [Autopoietic Loop — What's Built, What's Missing](#10-autopoietic-loop)
11. [Future Vision — XR and Classroom Scale](#11-future-vision)
12. [File-Level Implementation Reference](#12-file-level-reference)

---

## 1. System Audit

Verified March 22, 2026 via direct system commands.

### Hardware

| Component | Spec | Status |
|-----------|------|--------|
| CPU | Zen 5, 16C/32T, boost 5.1 GHz | ✅ |
| GPU | RDNA 3.5, 40 CUs, 2900 MHz | ✅ Running Mistral via Vulkan |
| NPU | XDNA 2, 50 TOPS | 🟡 Driver loaded (`/dev/accel0`), unused by Trinity |
| RAM | 128GB LPDDR5X-8000, 256-bit, 256 GB/s | ✅ |
| VRAM | 96GB UMA allocation (BIOS v1.05) | ✅ |
| Kernel | 6.19.4-061904-generic | ✅ NPU driver in-tree |
| ROCm | 7.2.0 | ✅ |
| Vulkan | 1.3.275 | ✅ Current llama.cpp backend |

### Software Installed

| Software | Version | Status | Notes |
|----------|---------|--------|-------|
| llama.cpp (longcat-sglang) | Latest | ✅ Active | Running Mistral Small 4, Vulkan backend |
| vLLM | 0.17.1 | ✅ Installed | Has EAGLE support code (`mistral_large_3_eagle.py`) |
| ONNX Runtime | 1.24.4 | 🟡 CPU provider only | Needs ROCm/XDNA execution provider for NPU |
| xdna-driver | Source built | ✅ Built | `/dev/accel0` present |
| sglang | Dir exists | ❌ Empty | Not set up |
| vllm-omni | Source tree | 🟡 Not built | Present at `~/trinity-models/vllm-omni/` |
| `~/.local/bin/lemonade` | N/A | ❌ WRONG PACKAGE | This is a parser generator, NOT AMD's Lemonade SDK |
| PostgreSQL | Active | ✅ | RAG + persistence |
| Trinity server (Rust) | Active | ✅ | Port 3000 |

### Models On Disk

| Model | Format | Size | Path | Use |
|-------|--------|------|------|-----|
| **Mistral Small 4 119B** | GGUF Q4_K_M | 68GB (2 shards) | `~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-*.gguf` | Conductor (always loaded) |
| Crow 9B | GGUF Q4_K_M | 5.3GB | `~/trinity-models/gguf/Crow-9B-*.gguf` | Swarm helper |
| REAP 25B (Qwen3 Coder) | GGUF Q4_K_M | 15GB | `~/trinity-models/gguf/Qwen3-Coder-REAP-25B-*.gguf` | Brakeman/Engineer |
| **Qwen 2.5 7B** | **ONNX (AWQ/UINT4)** | ~4GB | `~/trinity-models/onnx/qwen-2.5-7b-onnx-amd/` | **NPU ready** — quick queries, classification |
| mimi_encoder | ONNX | 178MB | `~/trinity-models/onnx/mimi_encoder.onnx` | **NPU ready** — voice input (PersonaPlex) |
| mimi_decoder | ONNX | 170MB | `~/trinity-models/onnx/mimi_decoder.onnx` | **NPU ready** — voice output (PersonaPlex) |
| fusion.onnx | ONNX (AWQ) | 8.5MB | `~/trinity-models/onnx/fusion.onnx` | **NPU ready** — multi-modal fusion |
| lm_backbone | ONNX | 4.7MB | `~/trinity-models/onnx/model.onnx` | **NPU ready** — language backbone |
| Qwen 3.5 mmproj | GGUF BF16 | 861MB | `~/trinity-models/vision/mmproj-Qwen3.5-35B-A3B-BF16.gguf` | Vision (GPU) |
| Piper TTS (Amy) | ONNX | Various | `~/trinity-models/onnx/en_US-amy-medium.onnx` | Voice sidecar |
| Piper TTS (Lessac) | ONNX | Various | `~/trinity-models/onnx/en_US-lessac-high.onnx` | Voice sidecar |
| **EAGLE head** | **NOT DOWNLOADED** | ~1-2GB | — | Speculative decoding draft model |

### Current Trinity Configuration

| Setting | Current Value | Changed This Session? |
|---------|--------------|----------------------|
| Context window | 262144 (256K) | ✅ Was 8192 |
| Max response tokens | 16384 (16K) | ✅ Was 1024-4096 |
| Max agent turns | 8 | ❌ Should be 16 |
| Max continuations | 3 | ❌ Should be 5 |
| Tool result truncation | 4000 chars | ❌ Should be 16000 |
| RAG injection cap | 1500 chars | ❌ Should be 16000 |
| Search file types | `.rs, .md, .toml` only | ❌ Missing `.py .jsx .js .css .json .yaml .sh .sql .html .tsx .ts` |
| Tool calling method | Regex (XML/JSON/GPT-OSS) | ❌ Should be native `--jinja` |
| NPU utilization | 0% | ❌ Should run housekeeping models |

---

## 2. Architecture Decision

### Hotel, Not Swarm — For Education

**Source:** `docs/bible/01-ARCHITECTURE.md` (lines 24-31)

> "Early designs considered a 'swarm' of agents all active simultaneously. This fails on 128GB hardware."

| | Swarm | Hotel |
|---|---|---|
| **Quality** | Multiple limited models | One model at full capability |
| **Memory** | Competing for 128GB → OOM risk | One model gets full allocation |
| **Education** | Inconsistent quality | Consistent, trustworthy output |
| **ART pipeline** | ✅ Fast creative batching | Slower for bulk worldbuilding |

**Decision:** Single model (Mistral Small 4) for ALL education, quests, tools, and Iron Road. Swarm only for P-ART creative batch worldbuilding (Aesthetics, Research, Timing).

### The Hotel Memory Layout

From `01-ARCHITECTURE.md`:
- **Room 1:** Conductor (Mistral Small 4) — ALWAYS loaded, 68GB
- **Room 2:** One sidecar at a time — 15-36GB depending on role
- **Queue:** Models waiting to be loaded via `sidecar_start`/`sidecar_stop`
- **Housekeeping:** NPU handles background tasks on separate silicon

### Why This Is Future-Proof

128GB unified memory is shipping NOW in consumer hardware (GMKtek EVO X2, Framework 16, ASUS NUC). In 5 years, this will be in every classroom. The Hotel pattern scales:
- More memory = bigger Room 1 model (or multiple rooms)
- NPU keeps improving (XDNA 3, 4...) = more concurrent housekeeping
- XR/VR rendering shares the same unified memory pool

---

## 3. Static vs Hotswap Memory Map

### Static (Always On): ~70GB

| Component | Memory | Silicon | Why Static |
|-----------|--------|---------|-----------|
| Conductor (Mistral Small 4) | 68GB | GPU | Every interaction goes through it |
| Trinity server + PostgreSQL | ~1GB | CPU | Infrastructure |
| NPU Housekeeping (below) | ~2GB total | **NPU** (separate silicon) | Runs concurrently, never competes with GPU |

### NPU Housekeeping (separate from GPU memory)

| Task | Model | Size | Purpose |
|------|-------|------|---------|
| EAGLE draft generation | EAGLE head (when downloaded) | ~1-2GB | 2-3x faster Conductor responses |
| Embedding generation | Qwen 2.5 7B ONNX or smaller | ~4GB or less | Offload RAG indexing from GPU |
| VAAM vocabulary scanning | Classifier model (TBD) | ~250MB | Detect vocabulary words, award coal |
| Voice input | mimi_encoder.onnx | 178MB | Teacher speech-to-text |
| Voice output | mimi_decoder.onnx | 170MB | TTS for Pete/NPCs |
| Multi-modal fusion | fusion.onnx | 8.5MB | Combine text + audio context |
| Great Recycler | Qwen 2.5 7B ONNX | ~4GB | Background book generation |

### Hotswap (Room 2): up to ~58GB remaining

| Sidecar | Model | Memory | ADDIE Phases |
|---------|-------|--------|-------------|
| Engineer/Yardmaster | REAP 25B or similar | 15-36GB | Develop, Implement, Yoke |
| Artist | Qwen Coder-32B | 21GB | Design, Envision |
| Evaluator | Qwen Coder-32B | 21GB | Analysis, Evaluation, Alignment |
| Pete (Socratic) | Qwen Coder-32B | 21GB | Repetition, Proximity |
| Visionary | Qwen 3.5-35B + mmproj | 21GB | Vision, screenshot analysis |
| P-ART Swarm | Crow 9B + OmniCoder 9B | ~12GB total | ART batch worldbuilding ONLY |

---

## 4. EAGLE Speculative Decoding

### What It Is

A tiny "draft model" guesses the next ~5 tokens. The big model (Mistral) verifies all 5 in one forward pass. If guesses are right (~70-80%), you get **2-3x faster generation with identical quality**.

### What's Available

Mistral released an official EAGLE head: `mistralai/Mistral-Small-4-119B-2603-eagle` on HuggingFace.

**Status on disk:** NOT downloaded. vLLM has the support code (`mistral_large_3_eagle.py`) but not the model weights.

### Two Implementation Paths

| Path | How | Pros | Cons |
|------|-----|------|------|
| **vLLM** (v0.17.1 installed) | Download safetensors → `vllm serve --model mistralai/Mistral-Small-4-119B --speculative-model eagle` | Native support, proven EAGLE impl | vLLM + ROCm on Strix Halo has had stability issues historically |
| **llama.cpp** (current backend) | Convert EAGLE safetensors → GGUF → launch with `--model-draft eagle.gguf` | Proven stable on this hardware | Conversion step needed, may need testing |

### Implementation Steps (Next Session)

**Path A — vLLM (try first):**
```bash
# 1. Activate vLLM env
source ~/trinity-models/trinity-vllm-env/bin/activate

# 2. Download EAGLE head (will cache to HuggingFace)
huggingface-cli download mistralai/Mistral-Small-4-119B-2603-eagle

# 3. Start vLLM with speculative decoding
vllm serve mistralai/Mistral-Small-4-119B-2603 \
  --speculative-model mistralai/Mistral-Small-4-119B-2603-eagle \
  --num-speculative-tokens 5 \
  --port 8080 \
  --max-model-len 262144

# 4. Update Trinity to point at vLLM endpoint (same OpenAI-compatible API)
# No code changes needed — same /v1/chat/completions endpoint
```

**Path B — llama.cpp (fallback):**
```bash
# 1. Download EAGLE safetensors
huggingface-cli download mistralai/Mistral-Small-4-119B-2603-eagle

# 2. Convert to GGUF (if a community GGUF isn't available yet)
python3 llama.cpp/convert_hf_to_gguf.py \
  ~/.cache/huggingface/hub/models--mistralai--Mistral-Small-4-119B-2603-eagle/ \
  --outfile ~/trinity-models/gguf/mistral-small-4-eagle.gguf

# 3. Launch longcat-sglang with draft model
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --model-draft ~/trinity-models/gguf/mistral-small-4-eagle.gguf \
  --draft-max 5 \
  -c 262144 \
  --port 8080 --host 127.0.0.1 \
  -ngl 99 -fa -ctk q4_0 -ctv q4_0 --no-mmap --ctx-shift
```

### Expected Impact

- Current: ~5 tok/s generation on Mistral Small 4
- With EAGLE: ~10-15 tok/s (2-3x improvement)
- No quality loss — same model, just faster verification of draft tokens

---

## 5. NPU Strategy

### Core Principle

**The NPU is housekeeping, not a second brain.** It handles background tasks on separate silicon while the GPU runs the Conductor. NPU models never compete with Mistral for GPU memory.

### What the NPU Can Run Today (ONNX models on disk)

| Task | Model On Disk | NPU Fit? | Status |
|------|--------------|----------|--------|
| Quick LLM queries | `~/trinity-models/onnx/qwen-2.5-7b-onnx-amd/` | ✅ Pre-optimized AWQ/UINT4 | Needs ORT with XDNA provider |
| Voice encode | `~/trinity-models/onnx/mimi_encoder.onnx` | ✅ 178MB | Needs ORT with XDNA provider |
| Voice decode | `~/trinity-models/onnx/mimi_decoder.onnx` | ✅ 170MB | Needs ORT with XDNA provider |
| Multi-modal fusion | `~/trinity-models/onnx/fusion.onnx` | ✅ 8.5MB AWQ | Needs ORT with XDNA provider |
| LM backbone | `~/trinity-models/onnx/model.onnx` | ✅ 4.7MB | Needs ORT with XDNA provider |

### What's Blocking NPU

**ORT 1.24.4 is installed with CPU provider only.** Need to install the AMD XDNA execution provider:

```bash
# Replace CPU-only ORT with AMD NPU-capable version
# Option 1: AMD's pre-built wheel
pip install onnxruntime-genai  # Includes NPU provider

# Option 2: Build from source with XDNA support
# See: https://github.com/microsoft/onnxruntime/blob/main/docs/execution_providers/XDNA-EP.md

# Option 3: Install AMD's Lemonade SDK (NOT the parser generator at ~/.local/bin/lemonade)
pip install lemonade-sdk  # AMD's actual NPU inference SDK
```

### NPU Context Limitation

AMD's NPU models currently support **up to 4096 tokens** combined context. This is fine for:
- Embeddings (short text → vector)
- Classification (is this vocabulary? yes/no)
- Voice (audio frames, not text tokens)
- Short queries to Qwen 2.5 7B

**NOT fine for:** Full 256K Mistral conversations (GPU only for that).

---

## 6. Agent Upgrade — Phase 1: Quick Wins ✅ DONE

**Time estimate: 15 minutes. Can do immediately.**

### 6a. Expand File Search

**File:** `crates/trinity/src/tools.rs`, line 468-474

**Current:** Only searches `.rs`, `.md`, `.toml`

**Change to:**
```
--include=*.rs --include=*.md --include=*.toml
--include=*.jsx --include=*.js --include=*.css
--include=*.py --include=*.json --include=*.yaml
--include=*.yml --include=*.sh --include=*.sql
--include=*.html --include=*.tsx --include=*.ts
```

### 6b. Increase Agent Limits

**File:** `crates/trinity/src/agent.rs`

| Setting | Line | Current | New |
|---------|------|---------|-----|
| `default_max_turns()` | ~80 | 8 | 16 |
| Continuation limit | ~394 | 3 | 5 |
| Tool result truncation | ~477 | 4000 | 16000 |
| RAG injection cap | ~267 | 1500 | 16000 |

### 6c. Add `--jinja` to longcat-sglang launch

**File:** `crates/trinity/src/main.rs`, line ~441 (auto-launch args)

Add `"--jinja"` arg — enables OpenAI-compatible `tools` parameter. Mistral Small 4 supports this natively.

**Also update:** `scripts/launch/start_trinity_full.sh` (line ~83) and `scripts/launch/demo_quick_start.sh`

### 6d. Rebuild Required

```bash
# After code changes:
cargo build --release -p trinity
cd crates/trinity/frontend && npm run build
# Then restart Trinity
```

---

## 7. Agent Upgrade — Phase 2: Native Function Calling ✅ DONE

**Time estimate: 2-3 hours. Core agent rewrite.**

### What Changes

Replace `extract_tool_calls()` (regex-based XML/JSON/GPT-OSS parsing in `agent.rs` lines 596-677) with OpenAI-compatible structured tool calling via the `tools` parameter.

### Files to Modify

**`crates/trinity/src/inference.rs`:**
- Add `tools` field to `CompletionRequest` struct
- Add `ToolDefinition`, `FunctionDef` structs (JSON Schema format)
- Add `tool_calls` field to response parsing
- Add `ToolCall`, `FunctionCallResult` structs

```rust
// New in CompletionRequest:
#[serde(skip_serializing_if = "Option::is_none")]
pub tools: Option<Vec<ToolDefinition>>,

#[derive(Serialize)]
pub struct ToolDefinition {
    pub r#type: String, // "function"
    pub function: FunctionDef,
}

#[derive(Serialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

// New in response:
#[derive(Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCallResult,
}

#[derive(Deserialize)]
pub struct FunctionCallResult {
    pub name: String,
    pub arguments: String, // JSON string
}
```

**`crates/trinity/src/agent.rs`:**
- Build `tools` array from existing 19 tool definitions in `list_tools()`
- Replace `extract_tool_calls()` with structured parsing from `tool_calls` field
- Add `"tool"` role messages for tool results (OpenAI standard)
- Remove tool format instructions from AGENT_SYSTEM prompt (model gets schemas)
- Keep: workspace awareness, safety, quest awareness, autonomous work sections
- Delete: `extract_tool_calls()`, `strip_tool_tags()` functions

### Agent Loop (After Rewrite)

```
1. Build messages (system + history + user)
2. Send request with `tools` array
3. Parse response:
   - If `tool_calls` present → execute tools → add results as `tool` role → goto 2
   - If no `tool_calls` → stream text to frontend → done
4. Repeat until: done, max_turns reached, or [CONTINUE] chaining
```

---

## 8. Agent Upgrade — Phase 3: Multi-Backend Inference Router ✅ DONE

**Completed: March 22, 2026. New module: `inference_router.rs`.**

### New File: `crates/trinity/src/inference_router.rs`

```rust
pub struct InferenceRouter {
    backends: Vec<InferenceBackend>,
    active: Option<usize>,
}

pub struct InferenceBackend {
    name: String,           // "llama.cpp", "vLLM", "Ollama", "LM Studio", "SGLang"
    base_url: String,       // http://127.0.0.1:8080
    supports_tools: bool,
    supports_vision: bool,
    model_name: Option<String>,
    healthy: bool,
}
```

### Auto-Detection Probe List

| Port | Backend | Notes |
|------|---------|-------|
| 8080 | longcat-sglang | Primary (Vulkan/ROCm) |
| 1234 | LM Studio | GUI-based |
| 8000 | vLLM / SGLang | Production serving |
| 11434 | Ollama | Docker/simple setup |

### Config: `configs/runtime/default.toml`

```toml
[inference]
primary = "longcat-sglang"
ctx_size = 262144
max_tokens = 16384

[inference.backends.longcat-sglang]
url = "http://127.0.0.1:8080"
supports_tools = true
jinja = true

[inference.backends.vllm]
url = "http://127.0.0.1:8000"
supports_tools = true

[inference.backends.ollama]
url = "http://127.0.0.1:11434"
supports_tools = true

[inference.backends.lm-studio]
url = "http://127.0.0.1:1234"
supports_tools = false
```

---

## 9. Educational Tools — Phase 4 ✅ DONE

**Time estimate: 1-2 hours per tool.**

### New Tools for `tools.rs`

| Tool | Purpose | Params |
|------|---------|--------|
| `python_exec` | Run Python code (teachers use Python) | `code: str`, `requirements?: [str]` |
| `generate_lesson_plan` | AI-powered lesson plan with standards | `subject, grade, objectives, standards, format` |
| `generate_rubric` | Assessment rubrics from objectives | `objectives, levels, criteria` |
| `generate_quiz` | Quiz questions from content | `content, question_types, difficulty, count` |
| `curriculum_map` | Map objectives to Bloom's taxonomy | `subject, objectives` |

### `python_exec` Implementation

```rust
async fn tool_python_exec(params: &serde_json::Value) -> Result<String, String> {
    // 1. Write code to /tmp/trinity_python_<uuid>.py
    // 2. If requirements, pip install to trinity-venv
    // 3. Execute with ~/trinity-models/trinity-vllm-env/bin/python3
    //    (or system python3)
    // 4. Capture stdout + stderr
    // 5. 60-second timeout
    // 6. Cleanup temp file
}
```

### Tool Call Persistence

**File:** `crates/trinity/src/persistence.rs`

```sql
CREATE TABLE IF NOT EXISTS trinity_tool_calls (
    id SERIAL PRIMARY KEY,
    session_id TEXT REFERENCES trinity_sessions(id),
    tool_name TEXT NOT NULL,
    params JSONB NOT NULL DEFAULT '{}',
    result TEXT,
    success BOOLEAN NOT NULL DEFAULT true,
    duration_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## 10. Autopoietic Loop

### What's Built (80%)

| Component | Status | File |
|-----------|--------|------|
| `work_log()` tool | ✅ | `tools.rs` |
| PostgreSQL persistence (sessions, messages, projects) | ✅ | `persistence.rs` |
| Quest system (12 phases, objectives) | ✅ | `crates/trinity-quest/` |
| VAAM vocabulary scanning + coal | ✅ | `agent.rs` VAAM bridge |
| RAG retrieval from knowledge base | ✅ | `agent.rs` + pgvector |
| DAYDREAM archive (scope creep → scope hope) | ✅ | `persistence.rs` |

### What's Missing (20%)

| Gap | Implementation | Impact |
|-----|---------------|--------|
| **Quest → RAG ingest** | On `/api/quest/compile` success, auto-call `rag::ingest` on the generated GDD | AI draws on user's own past work for future quests |
| **Work log → system prompt** | At session start, query latest `work_log` from DB, inject into system prompt | Agent remembers yesterday |
| **VAAM → quest difficulty** | Read vocabulary mastery scores, adjust `reasoning_effort` and content complexity | Adaptive difficulty |
| **Tool call logging** | Insert into `trinity_tool_calls` table after each execution | Audit trail for education compliance |

### Autopoietic Data Flow

```
User completes quest objective
        ↓
Conductor logs to PostgreSQL + work_log()
        ↓
Quest system compiles GDD
        ↓ (NEW: auto-ingest)
GDD → RAG vector store (pgvector)
        ↓
Next session: system prompt includes latest work_log
        ↓
RAG retrieves relevant past GDDs when generating new content
        ↓
VAAM scans responses for vocabulary → updates mastery
        ↓
Mastery level influences next quest's complexity
        ↓ (loop)
```

---

## 11. Future Vision

### XR/VR Goals (When Tech Is Ripe)

Trinity is being designed for the classroom of 2030+:
- **128GB unified memory** will be commodity hardware
- **NPU** handles spatial AI (head tracking, gesture recognition, spatial audio)
- **GPU** handles both rendering AND LLM inference in shared memory
- **Bevy** game engine targets XR natively
- The Hotel pattern scales: more memory = bigger models or more rooms

### Classroom Scale

One Trinity instance per student workstation:
- Each runs their own Conductor + quest system
- NPU handles local housekeeping
- Teacher dashboard aggregates progress via PostgreSQL
- ADDIECRAPEYE provides structured curriculum path
- VAAM ensures vocabulary integration across all interactions

---

## 12. File-Level Implementation Reference

### Files Changed THIS Session (March 22, 2026)

| File | Change | Status |
|------|--------|--------|
| `crates/trinity/src/hooks/useYardmaster.js` | `max_tokens: 4096 → 16384` | ✅ Done |
| `crates/trinity/src/agent.rs` | `default_max_tokens: 2048 → 16384` | ✅ Done |
| `crates/trinity/src/trinity_api.rs` | `default_max_tokens: 2048 → 16384` | ✅ Done |
| `crates/trinity/src/main.rs` | `default_max_tokens: 1024 → 16384` | ✅ Done |
| `CONTEXT.md` | Updated 256K context + 16K response docs | ✅ Done |

### Files to Change NEXT Session

| File | Change | Phase |
|------|--------|-------|
| `crates/trinity/src/tools.rs` (line ~468) | Expand search includes | Phase 1 (5 min) |
| `crates/trinity/src/agent.rs` (lines ~80, ~267, ~394, ~477) | Bump limits (turns, RAG, continuations, truncation) | Phase 1 (5 min) |
| `crates/trinity/src/main.rs` (line ~441) | Add `--jinja` to auto-launch | Phase 1 (5 min) |
| `scripts/launch/start_trinity_full.sh` | Add `--jinja` flag | Phase 1 (5 min) |
| `scripts/launch/demo_quick_start.sh` | Add `--jinja` flag | Phase 1 (5 min) |
| `crates/trinity/src/tools.rs` | Add `python_exec` tool | Phase 1 (30 min) |
| `crates/trinity/src/inference.rs` | Add `tools` + `tool_calls` structs | Phase 2 (1-2 hrs) |
| `crates/trinity/src/agent.rs` | Rewrite agent loop for structured tools | Phase 2 (2-3 hrs) |
| `crates/trinity/src/inference_router.rs` | **NEW** — multi-backend router | Phase 3 (2-3 hrs) |
| `crates/trinity/src/persistence.rs` | Add `trinity_tool_calls` table | Phase 4 (30 min) |
| `crates/trinity/src/tools.rs` | Add educational tools | Phase 4 (1-2 hrs) |
| `configs/runtime/default.toml` | Add `[inference]` section | Phase 3 (15 min) |

### Build & Verify After Changes

```bash
# 1. Rust rebuild
cargo build --release -p trinity

# 2. Frontend rebuild
cd crates/trinity/frontend && npm run build

# 3. Run tests
cargo test --workspace

# 4. Restart Trinity
# (kill existing, restart with new binary)

# 5. Verify
curl http://evo-x2:3000/api/hardware  # should show model info
# Open Yardmaster tab, test: "search for useState in the frontend"
# Test: "write a python script that prints hello world and run it"
```

---

## Implementation Order (Next Session)

| Order | Phase | Time | Risk |
|-------|-------|------|------|
| 1 | **Phase 1: Quick wins** (search, limits, `--jinja`) | 15 min | ✅ Done |
| 2 | **Phase 1: `python_exec` tool** | 30 min | ✅ Done |
| 3 | **Rebuild + verify** | 10 min | ✅ Done |
| 4 | **EAGLE: Download + try vLLM** | 30 min | ⏭️ Skipped (stability) |
| 5 | **EAGLE: Fallback to llama.cpp** | 30 min | ⏭️ Skipped (not needed yet) |
| 6 | **Phase 2: Structured function calling** | 2-3 hrs | ✅ Done |
| 7 | **Phase 3: Inference router** | 2-3 hrs | ✅ Done (March 22) |
| 8 | **Phase 4: Educational tools + persistence** | 1-2 hrs | ✅ Done |
| 9 | **Phase 5A: Modes + Sidecar Auto-start** | 3 hrs | ✅ Done (March 22) |
| 10 | **NPU: Install ORT XDNA provider** | 30 min | ⏳ Next |
| 11 | **NPU: Test Qwen 2.5 7B + voice models** | 1 hr | ⏳ Next |

**Total: ~10-14 hours spread across multiple sessions.**

---

*This document is the single source of truth. Do not fragment this information across other docs.*
*Created: March 22, 2026 | Last verified: March 22, 2026*
