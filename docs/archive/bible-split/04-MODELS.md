# Trinity Models Bible
**Document:** 04-MODELS.md  
**Purpose:** Model cards, P-ART-Y assignments, inference routing  
**Last Verified:** March 18, 2026 (from `ls -lh ~/trinity-models/`)  
**Isomorphic to:** [01-ARCHITECTURE.md §5](01-ARCHITECTURE.md), [PLAN_1_ARCHITECTURE.md](../plans/PLAN_1_ARCHITECTURE.md)

---

## 1. Model Portfolio Overview

**Hardware:** GMKtek EVO X2 128GB, AMD Strix Halo Zen5 AMD 395+  
**Unified Memory:** 128 GB LPDDR5X  
**Storage Location:** `~/trinity-models/gguf/` (GGUF) and `~/trinity-models/safetensors/` (safetensors)  
**Total On Disk:** ~555 GB across GGUF + safetensors

### 1.1 P-ART-Y Roster (Active Models)

| Role | Model | File | Size | Format | Serving | Port |
|------|-------|------|------|--------|---------|------|
| **P (Conductor/Pete)** | Mistral Small 4 119B MoE | `Mistral-Small-4-119B-2603-Q4_K_M-0000{1,2}-of-00002.gguf` | 68 GB | Split GGUF | llama-server | 8080 |
| **Y (Yardmaster)** | Ming-flash-omni-2.0 | `Ming-flash-omni-2.0/model-0000{1..42}-of-00042.safetensors` | ~195 GB | Safetensors | vLLM (PyO3) | 8000 |
| **A-R-T (R)** | Crow 9B | `Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf` | 5.3 GB | GGUF | llama-server | 8081 |
| **A-R-T (R)** | REAP 25B MoE | `Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf` | 15 GB | GGUF | llama-server | 8081 |
| **A-R-T (T)** | OmniCoder 9B | `OmniCoder-9B-Q4_K_M.gguf` | 5.4 GB | GGUF | llama-server | 8082 |
| **A-R-T (A)** | SDXL Turbo + ComfyUI | `~/trinity-models/safetensors/sdxl-turbo/` | ~6.5 GB | FP16 | ComfyUI HTTP | 8188 |

### 1.2 Reserve Models

| Model | File | Size | Format | Notes |
|-------|------|------|--------|-------|
| GPT-OSS-20B | `gpt-oss-20b-UD-Q4_K_XL.gguf` | 12 GB | GGUF | Legacy conductor, fast dev testing |
| Qwen3.5-27B Claude Opus | `Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf` | 21 GB | GGUF | Advanced reasoning / Evaluator |
| Qwen3.5-35B-A3B | `Qwen3.5-35B-A3B-Q4_K_M.gguf` | 20 GB | GGUF | Visionary (vision projector) |
| MiniMax-M2.5-REAP-50 | `MiniMax-M2-5-REAP-50-Q4_K_M.gguf` | 66 GB | GGUF | Colossus reserve |
| Step-3.5-Flash-REAP-121B | `Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf` | 83 GB | GGUF | Heavy engineer reserve |

---

## 2. Model Cards (Detailed)

### 2.1 P — Conductor (Pete): Mistral Small 4 119B MoE

| Attribute | Value |
|-----------|-------|
| **Full Name** | Mistral-Small-4-119B-2603-Q4_K_M |
| **Parameters** | 119B total, ~6.5B active per token (MoE) |
| **Format** | Split GGUF (2 shards: 37GB + 31GB = 68GB) |
| **Quantization** | Q4_K_M |
| **Context** | 256K tokens (with Q4 KV cache quantization) |
| **Vision** | ✅ Multimodal capable |
| **Speed** | 40+ tokens/sec on Strix Halo |
| **Serving** | llama-server on port 8080 |
| **Always Loaded** | ✅ Yes — the Conductor never unloads |
| **Role** | Orchestrates ADDIECRAPEYE, Socratic dialogue (Ask Pete), VAAM management, quest routing |

**Capabilities:**
- Quest orchestration and phase management
- ADDIE-C-R-A-P-E-Y-E workflow guidance
- Iron Road narrative generation
- P-ART-Y role assignment
- User intent classification

**Performance:**
- Load time: ~15-20 seconds
- Response time: 500ms-2s depending on complexity
- Memory footprint: 14GB

**Launch:**
```bash
llama-server \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 99 -c 32768 --port 8080
```

---

### 2.2 Engineer — Dual Sword & Shield

The Engineer uses a **dual-model pattern**:

#### Shield (Qwen2.5-Coder-32B)

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen2.5-Coder-32B-Instruct (Opus variant) |
| **Format** | GGUF Q6_K |
| **Size** | 21GB |
| **Context** | 32K tokens |
| **Role** | Planning, reasoning, code review |
| **Why** | Smarter but slower |

#### Sword (Qwen3-Coder-25B-A3B)

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen3-Coder-REAP-25B-A3B |
| **Format** | GGUF Q4_K_M |
| **Size** | 15GB |
| **Context** | 16K tokens |
| **Role** | Code generation, quick execution |
| **Why** | Faster for generation |

**Combined Performance:**
- Shield planning: ~90s (2 files) to ~4min (5 files)
- Sword generation: ~30-60s per file
- Shield review: ~60-90s
- **Total cycle:** ~7-9 minutes for complete quest

**Memory footprint:** 36GB (both loaded simultaneously)

**Launch:**
```bash
# Both models load on same server (dual context)
llama-server \
  -m ~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf \
  -ngl 99 -c 32768 --port 8082
```

---

### 2.3 Evaluator — Qwen2.5-Coder-32B (Extended Context)

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen2.5-Coder-32B Q6_K |
| **Format** | GGUF |
| **Size** | 21GB |
| **Context** | 65K tokens (extended) |
| **Primary Role** | Quality evaluation, QM rubrics |
| **Secondary Role** | WCAG audits, Bloom's alignment |

**Capabilities:**
- Quality Matters (QM) rubric compliance checking
- WCAG 2.1 AA accessibility audits
- Bloom's Taxonomy level assessment
- Instructional design evaluation
- 65K context for long document review

**Launch:**
```bash
llama-server \
  -m models/evaluator/opus-27b-Q6_K.gguf \
  -ngl 99 -c 65536 --port 8090
```

---

### 2.4 Artist — Qwen2.5-Coder-32B (Creative)

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen2.5-Coder-32B Q6_K |
| **Format** | GGUF |
| **Size** | 21GB |
| **Context** | 32K tokens |
| **Primary Role** | Game Design Documents, UI wireframes |
| **Secondary Role** | 2D/3D/XR asset specifications |

**Capabilities:**
- GDD generation with learning objectives
- UI/UX wireframe descriptions
- 2D sprite specifications
- 3D model requirements
- XR interaction design

---

### 2.5 Brakeman — Qwen3-Coder-25B

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen3-Coder-25B-A3B Q4_K_M |
| **Format** | GGUF |
| **Size** | 15GB |
| **Context** | 16K tokens |
| **Primary Role** | Test generation, security audits |
| **Secondary Role** | `cargo clippy/test` integration |

**Capabilities:**
- Unit test generation
- Security vulnerability scanning
- Rust best practices enforcement
- CI/CD pipeline validation

---

### 2.6 Pete — Qwen2.5-Coder-32B (Voice-Ready)

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen2.5-Coder-32B Q6_K |
| **Format** | GGUF |
| **Size** | 21GB |
| **Context** | 16K tokens |
| **Primary Role** | Socratic dialogue, questions not answers |
| **Secondary** | Voice integration (PersonaPlex pending) |

**Capabilities:**
- Bloom's-level questioning
- Guided discovery learning
- Reframing user struggles as progress
- "A structural collapse. Let's analyze the rubble."

---

### 2.7 Visionary — Qwen3.5-35B-A3B + mmproj

| Attribute | Value |
|-----------|-------|
| **Model** | Qwen3.5-35B-A3B Q4_K_M |
| **Vision** | mmproj (multimodal projector) |
| **Format** | GGUF |
| **Size** | 20GB (model) + 861MB (mmproj) = 21GB |
| **Context** | 32K tokens |
| **Primary Role** | Screenshot/UI evaluation, vision tasks |

**Capabilities:**
- UI/UX critique from screenshots
- Graphic design evaluation
- Visual accessibility assessment
- Image-to-text understanding

**Upgrade Path:** Qwen3.5-97B-A10B (56GB, not yet downloaded)

---

### 2.8 PersonaPlex-7B — Voice Specialist

| Attribute | Value |
|-----------|-------|
| **Model** | PersonaPlex-7B-v1 |
| **Format** | ONNX (for NPU) |
| **Size** | 14GB |
| **Primary Role** | Audio-to-audio conversation |
| **Latency** | <200ms voice response |
| **Integration** | Ask Pete voice interface |

**Capabilities:**
- Real-time voice dialogue
- Natural conversation flow
- Emotional tone recognition
- Sub-200ms response time

**Status:** Model ready, integration pending

---

### 2.9 Swarm Models (Quick Queries)

#### Crow-9B
- **Purpose:** Fast research queries
- **Size:** 6GB
- **Context:** 32K tokens
- **When to use:** Quick lookups, fast analysis

#### OmniCoder-9B
- **Purpose:** Lightweight coding
- **Size:** 6GB
- **Context:** 32K tokens
- **When to use:** Simple code tasks when Engineer is too heavy

---

## 3. Intelligent Delegation Matrix

### 3.1 Task → Model Mapping

| Task Type | Primary Model | Secondary | Latency Target |
|-----------|--------------|-------------|----------------|
| **Voice Dialogue** | PersonaPlex-7B | → 9B Thinking | <200ms |
| **Complex Analysis** | 97B Conductor | → 27B Analyst | <2s |
| **Quick Reasoning** | 9B Thinking | — | <500ms |
| **Code Generation** | 14B Rust-Coder | → 35B Instruct | <1s |
| **Visual Learning** | Vision Projector | → SDXL Turbo | <3s |
| **Instruction Following** | 35B Instruct | → 97B Conductor | <2s |
| **QM Evaluation** | Qwen2.5-Coder-32B (65K) | — | 60-90s |
| **Game Design** | Qwen2.5-Coder-32B (32K) | — | 90-120s |

### 3.2 Memory Budget Strategy

**Always Loaded (72GB):**
- Conductor: 14GB
- Engineer (both): 36GB
- One specialist: 21GB

**On-Demand (56GB swap space):**
- Evaluator: 21GB
- Artist: 21GB
- Brakeman: 15GB
- Pete: 21GB
- Visionary: 21GB

**Hot Swap Priority:**
1. Conductor (never unloads)
2. Engineer (unloads only for other code tasks)
3. Current task specialist

### 3.3 Delegation Decision Tree

```
User Request
    ↓
Conductor analyzes intent
    ↓
┌────────────────┬────────────────┬────────────────┐
│ Coding task    │ Creative task  │ Evaluation   │
↓                ↓                ↓
Engineer         Artist           Evaluator
(Shield plans)   (GDD gen)        (QM audit)
↓                ↓                ↓
Sword executes   Wireframes       Report
↓                ↓                ↓
Shield reviews   Spec complete    Recommendations
↓                ↓                ↓
Return result    Return GDD       Return audit
```

---

## 4. Model Storage Structure

```
~/.trinity/models/
├── conductor/
│   └── GPT-OSS-npu.onnx          # 14GB, NPU-optimized
├── yardmaster/
│   └── Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf  # 12GB, primary
├── engineer/
│   ├── Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf  # 15GB
│   └── Opus-27B-Q6_K.gguf        # 21GB, Shield
├── evaluator/
│   └── opus-27b-Q6_K.gguf        # 21GB, 65K context
├── artist/
│   └── opus-27b-Q6_K.gguf        # 21GB, 32K context
├── swarm/
│   ├── crow-9b.gguf              # 6GB
│   └── omni-coder-9b.gguf        # 6GB
├── visionary/
│   ├── Qwen3.5-35B-A3B-Q4_K_M.gguf  # 20GB
│   └── mmproj-Qwen3.5-35B.gguf   # 861MB
└── voice/
    └── personaplex-7b-v1.onnx    # 14GB
```

---

## 5. Performance Benchmarks

### 5.1 Model Load Times

| Model | Size | Cold Load | From Cache |
|-------|------|-----------|------------|
| Mistral Small 4 | 14GB | 15-20s | 5-8s |
| Qwen2.5-Coder-32B | 21GB | 25-30s | 8-12s |
| Qwen3-Coder-25B | 15GB | 18-22s | 6-9s |
| Crow-9B | 6GB | 8-12s | 3-5s |

### 5.2 Response Times (Strix Halo)

| Task | Model | Tokens | Time |
|------|-------|--------|------|
| Simple chat | Mistral Small 4 | 200 | 800ms |
| Code planning | Qwen2.5-Coder-32B | 2000 | 90s |
| Code generation | Qwen3-Coder-25B | 1000 | 45s |
| QM evaluation | Qwen2.5-Coder-32B (65K) | 500 | 60s |
| GDD creation | Qwen2.5-Coder-32B | 3000 | 120s |

### 5.3 Memory Bandwidth

- **Unified LPDDR5X:** 256 GB/s
- **GPU share:** Dynamic, up to 80GB
- **CPU share:** Remainder
- **Effective for LLMs:** ~72GB active, 56GB swap

---

## 6. Cross-References

| Section | Reference |
|---------|-----------|
| P-ART-Y roles | [01-ARCHITECTURE.md §5](01-ARCHITECTURE.md) |
| Hotel pattern | [01-ARCHITECTURE.md §1](01-ARCHITECTURE.md) |
| Sidecar management | [03-OPERATIONS.md §4](03-OPERATIONS.md) |
| Launch commands | [02-IMPLEMENTATION.md §4](02-IMPLEMENTATION.md) |

---

*End of 04-MODELS.md — Model Portfolio & Delegation*
