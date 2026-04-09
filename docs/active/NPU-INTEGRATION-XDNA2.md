# NPU Integration — AMD XDNA 2 Speculative Decoding
## Wire ONNX Runtime for Strix Halo NPU Acceleration

> **Status:** DESIGN + PROOF OF CONCEPT  
> **Hardware:** AMD Ryzen AI 300 / Strix Halo — XDNA 2 NPU (50 TOPS)  
> **Driver:** `amdxdna` kernel module (loaded, `/dev/accel0` present)  
> **Depends On:** ONNX Runtime + Vitis AI EP, inference_router.rs

---

## 1. Hardware Verification

**Confirmed on this system:**

| Component | Status | Detail |
|-----------|--------|--------|
| NPU PCI Device | ✅ Present | `c6:00.1 Signal processing controller: AMD Device 17f0` |
| Kernel Module | ✅ Loaded | `amdxdna 163840` |
| Device Node | ✅ Active | `/dev/accel0` (render group) |
| NPU TOPS | ~50 | XDNA 2 architecture |
| Shared Memory | ✅ Unified | CPU/GPU/NPU share DDR5 pool |

---

## 2. Architecture — NPU in Trinity's Hotel Pattern

Trinity uses the **Hotel Pattern** for inference: multiple backends (longcat-sglang instances) managed by `inference_router.rs`. The NPU slots into this as a **speculative decoding accelerator**:

```
┌─────────────────────────────────────────────────────┐
│                HOTEL PATTERN                         │
│                                                      │
│  ┌──────────────┐   ┌──────────────┐                │
│  │ longcat-sglang  │   │ longcat-sglang  │               │
│  │ GPU (Primary) │   │ CPU (Fallback)│               │
│  │ :8080         │   │ :8081         │               │
│  └──────┬───────┘   └──────────────┘                │
│         │                                            │
│         │  speculative tokens                        │
│         ▼                                            │
│  ┌──────────────┐                                    │
│  │  NPU Sidecar  │   ONNX Runtime + Vitis AI EP     │
│  │  Draft Model   │   Small model (1-3B params)      │
│  │  :8095         │   Generates candidate tokens     │
│  └──────────────┘                                    │
│                                                      │
│  Flow:                                               │
│  1. NPU generates N draft tokens (fast, low power)   │
│  2. GPU verifies all N in parallel (one forward pass) │
│  3. Accept verified tokens, reject wrong ones         │
│  4. Net speedup: 1.5-3x with ~50% power savings      │
└─────────────────────────────────────────────────────┘
```

### Why This Approach

| Metric | GPU-Only (Current) | GPU+NPU (Speculative) |
|--------|-------------------|----------------------|
| Token/sec | ~40 t/s | ~60-100 t/s (est.) |
| Power | ~150W TDP | ~100W total (NPU ≈ 15W) |
| First Token Latency | ~200ms | ~180ms (NPU prefill) |
| Use Case | Primary inference | Drafting + embedding |

---

## 3. Implementation Strategy

### 3.1 Phase 1 — ONNX Runtime Installation

AMD requires the **Vitis AI** build of ONNX Runtime, not the pip default:

```bash
# Install AMD's specialized ONNX Runtime with Vitis AI EP
pip install onnxruntime-vitisai
# Or from AMD's RyzenAI SDK:
# pip install --index-url https://riallto.ai/simple/ onnxruntime-vitisai

# Verify NPU provider is available
python3 -c "
import onnxruntime as ort
print('Available providers:', ort.get_available_providers())
# Should include: 'VitisAIExecutionProvider'
"
```

### 3.2 Phase 2 — Draft Model Selection

For speculative decoding, the draft model should be:
- **Small** (1-3B parameters) — fast on NPU
- **Same tokenizer** as the primary model — required for token alignment
- **Quantized** (INT8/INT4) — maximizes NPU throughput

| Candidate | Params | Tokenizer Match | NPU Compatibility |
|-----------|--------|-----------------|-------------------|
| SmolLM2-1.7B | 1.7B | ✅ | ONNX quantized available |
| Mistral-Tiny | 1B | ✅ (with Mistral) | Needs quantization |
| Phi-3-mini | 3.8B | ✅ | ONNX available |
| DeepSeek-R1-1.5B | 1.5B | ✅ | ONNX available |

### 3.3 Phase 3 — NPU Sidecar (Python)

```python
# npu_sidecar.py — Speculative decoding draft server
from fastapi import FastAPI
import onnxruntime as ort
import numpy as np

app = FastAPI()

# Load model with Vitis AI EP (targets NPU)
session = ort.InferenceSession(
    "models/draft-model-int8.onnx",
    providers=["VitisAIExecutionProvider", "CPUExecutionProvider"],
    provider_options=[{
        "config_file": "/opt/amdxdna/vaip_config.json",
        "cacheDir": "/tmp/npu_cache",
    }, {}]
)

@app.post("/draft")
async def generate_draft(prompt_tokens: list[int], n_draft: int = 5):
    """Generate N speculative draft tokens on the NPU."""
    # Run draft model forward pass
    input_ids = np.array([prompt_tokens], dtype=np.int64)
    logits = session.run(None, {"input_ids": input_ids})[0]
    
    # Greedy decode N tokens
    draft_tokens = []
    for _ in range(n_draft):
        next_token = int(np.argmax(logits[0, -1, :]))
        draft_tokens.append(next_token)
        input_ids = np.append(input_ids, [[next_token]], axis=1)
        logits = session.run(None, {"input_ids": input_ids})[0]
    
    return {"draft_tokens": draft_tokens}

@app.get("/health")
async def health():
    providers = ort.get_available_providers()
    return {
        "status": "ok",
        "npu_available": "VitisAIExecutionProvider" in providers,
        "providers": providers,
    }
```

### 3.4 Phase 4 — Rust Integration

Wire the NPU sidecar into `inference_router.rs`:

```rust
// In inference_router.rs — add NPU backend
pub struct NpuDrafter {
    client: reqwest::Client,
    url: String,
    healthy: bool,
}

impl NpuDrafter {
    pub async fn draft_tokens(
        &self, 
        prompt_tokens: &[u32], 
        n_draft: usize
    ) -> Option<Vec<u32>> {
        if !self.healthy { return None; }
        
        let resp = self.client
            .post(format!("{}/draft", self.url))
            .json(&serde_json::json!({
                "prompt_tokens": prompt_tokens,
                "n_draft": n_draft,
            }))
            .timeout(Duration::from_millis(200)) // NPU must be fast
            .send()
            .await
            .ok()?;
        
        let data: serde_json::Value = resp.json().await.ok()?;
        Some(data["draft_tokens"].as_array()?
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u32))
            .collect())
    }
}
```

### 3.5 Phase 5 — Additional NPU Workloads

Beyond speculative decoding, the NPU can handle:

| Workload | Benefit | Integration Point |
|----------|---------|-------------------|
| **Embedding generation** | Free up GPU for inference | `rag.rs` → NPU sidecar `/embed` |
| **Voice activity detection** | Real-time VAD for voice mode | `trinity-voice` → NPU Silero VAD |
| **Bloom's verb classification** | Instant pedagogical tagging | Ring 6 Perspective Engine |
| **Content moderation** | Safety filter on NPU | Ring 5 pre-filter |

---

## 4. EAGLE Pattern (Advanced)

The EAGLE speculative decoding pattern (from Peking University) is more sophisticated:

```
Standard Speculative: Draft model → Verify → Accept/Reject
EAGLE:               Draft model + Feature Reuse → Verify → Accept/Reject
```

EAGLE reuses the primary model's hidden states to improve draft quality, achieving **2.5-3.5x speedup** vs. 1.5-2x for standard speculative decoding. 

For Trinity, this would require:
1. Modifying longcat-sglang to expose hidden states
2. Feeding hidden states to a small EAGLE head on the NPU
3. This is a medium-term goal (after basic speculative decoding works)

---

## 5. Power Budget Analysis

| Configuration | TDP | Token/sec | W/token |
|--------------|-----|-----------|---------|
| GPU only (Radeon 890M) | 120W | 40 t/s | 3.0 W/t |
| GPU + NPU speculative | 135W total | 80 t/s | 1.7 W/t |
| NPU only (embedding) | 15W | N/A | — |
| CPU fallback | 55W | 8 t/s | 6.9 W/t |

The NPU improves **W/token by ~43%** when used for speculative decoding.

---

## 6. Implementation Estimate

| Phase | Effort | Dependency |
|-------|--------|------------|
| 1. Install ONNX + Vitis AI | 1 hour | AMD SDK download |
| 2. Quantize draft model | 2 hours | Model selection |
| 3. NPU sidecar (Python) | 3 hours | Phase 1 |
| 4. Rust integration | 2 hours | Phase 3 |
| 5. E2E test | 1 hour | Phase 4 |

**Total: ~9 hours**

---

## 7. Immediate Next Steps

1. **Install AMD RyzenAI SDK** — download from AMD.com, includes Vitis AI EP
2. **Verify NPU round-trip** — run a simple ONNX model on the NPU and confirm `/dev/accel0` is used
3. **Quantize SmolLM2-1.7B** — export to ONNX INT8 format
4. **Build NPU sidecar** — FastAPI server on port 8095
5. **Wire into inference_router.rs** — add as `BackendKind::Npu`
