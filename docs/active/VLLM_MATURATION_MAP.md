# vLLM Maturation Map: Native Gemma 4 & P-ART-Y Integration
**Target Hardware:** AMD Ryzen AI Max+ 395 (Strix Halo APU) `gfx1151`
**Target Framework:** vLLM 0.19.1rc1 (TheRock build)
**Date:** April 17, 2026

---

## 1. The North Star Objective
To transition the Trinity AI OS to a production-grade inference backend using vLLM to natively support the **Gemma 4** architecture (Tempo, Pete, Recycler). This requires mastering AMD Unified Memory Architecture (UMA), escaping unstable PyTorch dependency hell, and enabling native multi-modal function calling (`gemma4` tool-call parser).

---

## 2. Execution Plan & Progress

### Phase 1: Environmental Stabilization 🟩 (Completed)
- [x] **Container Migration:** Abandon `vllm-quant` distrobox (which suffered from PyTorch 2.5 `pydantic` silent crashes).
- [x] **Stable Image Deployment:** Create `vllm-019` distrobox using the community-verified `kyuz0/vllm-therock-gfx1151:latest` container (vLLM 0.19.1rc1).
- [x] **Rust Engine API Patch:** Update `crates/trinity/src/inference.rs` to handle the vLLM 0.19 `reasoning` API field rename, while maintaining legacy `reasoning_content` fallbacks.
- [x] **Launch Script Configuration:** Inject required environment vars (`PYTORCH_ROCM_ARCH`, `NCCL_P2P_DISABLE`) and critical vLLM flags (`--quantization awq`, `--tool-call-parser gemma4`) into all `scripts/launch/` bash scripts.

### Phase 2: Single-Node Hardware Validation 🟩 (Completed)
- [x] **Test Tempo (Fast-Twitch Brain):** Run `launch_tempo_e4b.sh` on Port 8001. Validated at `0.25` VRAM envelope (V0 engine).
- [x] **Test Pete Coder (MoE):** Run `launch_pete_coder.sh` on Port 8000. Validated with `compressed-tensors` quantization flag.
- [x] **Test Recycler (Dense):** Run `launch_recycler_dense.sh` on Port 8002.
- [x] **Tool Calling Syntax Check:** Execute a prompt specifically requesting an image to ensure the `--tool-call-parser gemma4` intercepts the JSON and returns standard OpenAI Tool Calls.

### Phase 3: The Hotel Swap Protocol Validation 🟩 (Completed)
- [x] **Test Dynamic Loading:** Used `conductor_leader.rs` Hotel Swap to load Pete (Port 8000), verify VRAM occupation, then kill the process.
- [x] **Test VRAM Flushing:** Verified that `kill -9 $(lsof -ti:PORT)` properly flushes the PyTorch expandable segments back to the Strix Halo shared memory pool.
- [x] **Test Next Guest:** Launched Recycler Dense (Port 8002). Verified stable load, identified `config.json` shape mismatch, and patched to achieve full stability.
- [ ] **Latency Benchmarking:** Document the time it takes to kill, load, and serve the next model entirely off the NVMe.

### Phase 4: Multimodal & OMNI Integration 🟥 (Pending)
- [ ] **Vision Input Testing:** Pass an Iron Road screenshot to Pete; ensure the `ClippableLinear` multimodal ingestion layers execute without crashing.
- [ ] **Creative API Proxies:** Validate that when Pete uses the `generate_image` tool, the `vllm_router.py` correctly proxies the request to the separate FLUX sidecar process without stalling the text generation loop.

---

## 3. Engineering Logbook & Lessons Learned

### Lesson 1: The D-State Memory Death Trap
**Symptom:** System freezes, GUI locks up, 100% disk usage.
**Cause:** vLLM defaults to 90% GPU memory allocation. Because Strix Halo shares physical RAM between CPU and GPU, allocating 90% starves the Linux OS kernel, forcing it to violently page memory to the NVMe (Uninterruptible Sleep/D-State).
**Solution:** The system *must* be bound to a strict global maximum of `.55` (for single massive models like LongCat) or isolated constraints per-model (e.g., `.15` for Tempo, `.20` for Pete).

### Lesson 2: AWQ Fused Tensor Failure
**Symptom:** Models fail to load with `ValueError: ... parameter named 'input_max'`.
**Cause:** Newer versions of Transformers (>= 5.5.4) and vLLM fail to auto-detect AWQ layouts on certain architectures.
**Solution:** Explicitly appending `--quantization awq` to the `vllm serve` command bypasses auto-detection and maps the tensors correctly.

### Lesson 3: The Strix Halo Support Gap
**Symptom:** Upstream vLLM fails to detect the hardware, requiring building from source.
**Cause:** `gfx1151` is specialized hardware. Official AMD ROCm binaries target server cards (`gfx9` or `gfx10`).
**Solution:** Rely on community-maintained Docker/Distrobox images (like `kyuz0/vllm-therock-gfx1151`) which patch `amdsmi` and enable gated performance kernels (like TunableOp) specifically for the Strix Halo iGPU.

### Lesson 4: vLLM V1 Engine Crash on Unified Memory
**Symptom:** `RuntimeError: Engine core initialization failed... ValueError: No available memory for the cache blocks.`
**Cause:** vLLM 0.19 defaults to the new V1 engine architecture. On Strix Halo's Unified Memory, the V1 engine's pre-allocation of HIP expandable segments completely fails, resulting in an immediate OOM crash.
**Solution:** Force the legacy, stable engine using `export VLLM_USE_V1=0` in all launch scripts, and ensure `gpu-memory-utilization` is slightly padded (e.g. `0.25` for Tempo) to fit the 32K context KV cache.

### Lesson 5: Quantization Flag Mismatches
**Symptom:** `Cannot find the config file for awq` when loading compressed models.
**Cause:** NeuralMagic's compression tool writes `"quant_method": "compressed-tensors"` into the `config.json` instead of `"awq"`, even for AWQ models.
**Solution:** The `--quantization` flag passed to `vllm serve` MUST exactly match the `quant_method` in the model's `config.json`. Pass `--quantization compressed-tensors` for models that have that signature (Pete/Recycler), and manually inject the AWQ config block for models that are missing it entirely (Tempo).

### Lesson 6: The Gemma 4 31B Dense Config.json Typo
**Symptom:** `AssertionError: Tried to load weights of size torch.Size([5376, 1152]) to a parameter of size torch.Size([2048, 1152])`.
**Cause:** The downloaded AWQ model config for `Gemma 4 31B Dense` incorrectly listed `"hidden_size": 2048`. Because vLLM 0.19 now natively wires the `vision_tower` projections into the unified model graph using the text hidden size, this 2048 typo caused a matrix multiplication mismatch with the actual `5376` projection weights.
**Solution:** Manually patched `gemma-4-31B-it-AWQ-4bit/config.json` to `"hidden_size": 5376`. The model immediately passed the PyTorch module loader and initialized.

### Lesson 7: UMA KV Cache Constraints
**Symptom:** `ValueError: To serve at least one request with the models's max seq len (8192), (6.89 GiB KV cache is needed...`
**Cause:** The 31B Dense model occupies 22.56 GiB of VRAM just for weights. When restricted to `gpu-memory-utilization 0.25` (32 GiB total) on the Strix Halo, only 5.31 GiB remains for the KV cache, which cannot support an 8192 token context window.
**Solution:** Reduced `--max-model-len` to `4096` for the 31B Dense model, bringing the required KV cache down to ~3.45 GiB. The model initialized successfully.

---

## 4. Next Actions
*Phase 4: Full Multi-Modal and UI integration validation.*
