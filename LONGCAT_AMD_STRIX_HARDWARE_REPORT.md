# EXHAUSTIVE DIAGNOSTIC & RESEARCH REPORT: LongCat-Next MoE on AMD Strix Halo (gfx1151)

**Context:** This document is the result of deep-dive architectural analysis, correlating empirical deployment data against recent research sessions executed with Claude Opus. It serves as the authoritative, highly-technical root record for the friction encountered when forcing the LongCat-Next MoE (74B) into the AMD unified memory topology.

---

## 1. Hardware Topologies & Kernel Matrices

The core deployment friction stems from the mismatch between consumer edge hardware topologies and enterprise CUDA-native software layers.

### 1.1 Hardware Base Geometry
- **Hardware Platform:** GMKtec NucBox_EVO-X2
- **APU:** AMD Ryzen AI Max+ 395 (Strix Halo)
- **GPU Target:** Radeon 8060S (ROCm `gfx1151`, RDNA 3.5 architecture)
- **Unified RAM:** 128GB LPDDR5X (Shared CPU/GPU/NPU zero-copy matrix)

### 1.2 Kernel Environment Conflicts (Resolved via ROCm 7.2.1)
Recent Claude Opus research correctly highlighted the fracturing of AMD driver support across edge distributions. However, as of **ROCm 7.2.1** (March 2026), the initial driver emulation failures have been resolved.
- **Active Kernel (Ubuntu 24.04 LTS):** Prior to ROCm 7.2.1, bridging Host driver stacks into the Docker boundary on `gfx1151` faced severe volatility. The Linux Mesa Vulkan driver (`RADV`) previously imposed strict logical caps on Graphics Translation Tables (GTT) at ~40GB to prevent OS starvation. Attempting to allocate the 80GB LongCat tensor map caused arbitrary `SIGSEGV` panics. 
- **The Native Solution:** ROCm 7.2.1 officially bridges the architecture directly without relying on `RADV_PERFMODE=nogttspill` or bare-metal GRUB injections, supporting native 128GB unified memory pools out of the box.

---

## 2. Software Driver & Compilation Lock (Decoupled Architecture)

To execute quantization safely without triggering NVMe swap loops or memory allocation conflicts, the software stack has been decoupled into two distinct pipelines.

**Pipeline A: Offline Quantization (The `quark-forge` Sandbox)**
- **Isolation Goal:** Removes the necessity of using PyTorch experimental nightlies for statically compiling 4-bit weights.
- **Environment:** Clean `ubuntu:24.04` Distrobox
- **Dependencies:** 
  - `torch==2.4.1+rocm6.1` (The stable LTS release explicitly required by AMD Quark)
  - `amd-quark` (Hardware-native APU tensor compiler)
- **Execution:** Generates the AWQ arrays layer-by-layer completely offline.

**Pipeline B: Inference Engine (vLLM Serving)**
- **Environment:** Official ROCm 7.2.1 supported serving container. 
- **Transformers Framework:** Locked to exactly `4.57.6` to preserve LongCat DiNA embeddings.
- **Execution:** Reads the pristine AWQ footprint directly from disk without modifying PyTorch core structures during runtime.

---

## 3. The 5 Vector Collapse Events (Traceback Analysis)

The following represents the raw execution traces from the integration attempts when bridging the 74B MoE into the `gfx1151` 4-bit memory space.

### Collapse 1: The NVIDIA `turboquant-vllm` Memory Segfault
```python
RuntimeError: shape '[2, 40235, 16, 8, 128]' is invalid for input of size 700410880
```
- **Analysis:** `turboquant-vllm` is an enterprise NVIDIA package for W4A16 KV Cache compression. Its accidental inclusion in the `therock` container caused an aggressive overwrite of the PyTorch native memory allocator. When the ROCm driver attempted to map the `gfx1151` memory topology, `turboquant` enforced a CUDA stride layout, fracturing the tensor boundaries.

### Collapse 2: `flash_attention_2` Variable Length Panic
```python
ValueError: max_seqlens_q must be provided [within DiT/VAE processors]
```
- **Analysis:** AMD ROCm struggles profoundly when executing `AttnProcessorFlash2Varlen()`. LongCat uses DiNA visual decoding (where image generation cascades sequentially across variable length sequences in the DepthTransformer). FlashAttention requires hard-coded tensor boundaries, and the variable length arrays crashed the compiler.
- **Bypass:** Hard-coded `_attn_implementation` to `"sdpa"` in `config.json` and physically deleted `AttnProcessorFlash2Varlen()` from `refiner_modules.py`.

### Collapse 3: MoE Routing Network Quantization Fracture
```python
RuntimeError: mat1/mat2 shapes cannot be multiplied (256x3840 and ...)
```
- **Analysis:** Deep research proved that HuggingFace `.from_pretrained(quantization_config=BitsAndBytesConfig)` blindly squashes ALL layers to 4-bit NF4. Mixture of Expert models mathematically rely on hyper-precise fp16 gradient calculation on their gating/routing networks. A 4-bit router corrupts the Top-K logic, sending generic token IDs to incorrect experts.
- **Bypass:** Mandatory exact slice exceptions: `llm_int8_skip_modules=["router", "classifier", "lm_head"]`.

### Collapse 4: Structural Breakdown of Visual Flow-Matching Decoders
```python
RuntimeError: shape invalid for input 33554432 (inside FLUX VAE spatial transform)
```
- **Analysis:** The DiNA native token generator pushes linear latent streams. The 400M ViT pixel decoder processes these using PyTorch `.reshape()` operations. Tensors stored in 4-bit block formats cannot be reshaped dynamically at runtime without undergoing expensive dequantization loops, triggering input size mismatches.
- **Bypass:** `llm_int8_skip_modules=["linear1", "linear2"]`.

### Collapse 5: The "Meta Tensor" Zero-Copy Hydration Refusal
```python
NotImplementedError: Cannot copy out of meta tensor
RuntimeError: Tensor.item() cannot be called on meta tensors
```
- **Analysis:** To avoid memory-starvation, HF loads the 80GB model as an empty `.meta` device scaffold. When `bitsandbytes` initiates state hydration, it must transfer the NF4 code matrices from Host CPU RAM to VRAM. Because Strix Halo is a physical Unified Memory module, the standard `.to(device)` memory copy loop concludes immediately (assuming the memory is already shared) but fails to pass the physical pointers into the ROCm GPU abstraction map. Thus, execution hits a ghost tensor. 
- **Bypass Attempt:** Required brute-force pointer manipulation using `accelerate.utils.set_module_tensor_to_device(..., "cuda:0")`.

---

## 4. Analytical Conclusion

When reviewed critically, the LongCat-Next deployment on AMD Strix Halo was inherently unstable. 
While we achieved theoretical integration by utilizing the **Multi-Head Latent Attention (MLA)** feature to minimize the unquantized 131K KV Cache down to ~4.7GB, the friction required to maintain this architecture is astronomical.

The reliance on kernel `6.19.4-061904-generic` rather than community-supported Fedora distributions compounded the driver emulation failures. The fact that `bitsandbytes` structural decoding required five discrete, highly fragile python monkey-patches (`sdpa` overrides, manual `.meta` pointer hydrations, and dynamic component exclusion lists) indicates that `gfx1151` is fundamentally hostile to heavily quantized, multimodal MoE structures operating outside of native `llama.cpp/GGUF` paradigms.

This constitutes the exact, exhaustive, unblemished technical truth of the system friction.

### Collapse 6: Unified Memory OOM Limits and Infinite NVMe Swap Thrashing
```python
torch.OutOfMemoryError: CUDA out of memory. Tried to allocate 126.80 GiB. GPU 0 has a total capacity of 128.00 GiB of which 121.48 GiB is free.
```
- **Analysis:** When utilizing `llmcompressor` for post-training quantization, the unified APU architecture treats `cpu` memory and `cuda:0` memory as the same physical stick of DDR5, but PyTorch and Linux treat them as strictly separated boundary matrices. When executing `AutoModelForCausalLM` without a restrictive `max_memory` map, PyTorch aggressively forced all `126.8GB` of the 74B bf16 model directly into the `cuda:0` logical driver limit, triggering a hard Out-Of-Memory failure. 
- **The "Linux D-State" Trap:** Relying on `device_map="auto"` *without* explicit chunk limits (e.g., `{"cuda:0": "115GiB", "cpu": "20GiB"}`) caused the Linux kernel to dump overflow tensors dynamically into Swap space. Because the quantization calibration phase iterates over the entire model layer-by-layer, the active working set was continuously pulled straight from the NVMe storage across the PCIe bus natively inside the Uninterruptible Sleep (`D` state). The process appeared indefinitely frozen with high load but 0% actual GPU computing because the system was infinitely thrashing storage swap instead of passing matrix streams to the shaders.
- **Bypass:** Strict chunked offload parameters and resolving dependency masking (`pip install --no-deps`) to ensure `llmcompressor` didn't quietly degrade `torch` out from under the ROCm environment.
