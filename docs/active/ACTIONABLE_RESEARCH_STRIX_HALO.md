# Actionable Research: Strix Halo XR Dev Workstation

**Sources:** "Spatial Computing Dev Workstation Blueprint.md" + broader web research (July 5, 2026)
**Verified:** July 5, 2026 against kernel docs, ROCm issues, GitHub issues, community guides, and current system state
**Status:** Triaged — items marked APPLY NOW, TEST FIRST, or REFERENCE ONLY

---

## 1. Kernel Parameter Deltas (APPLY NOW)

Current `/proc/cmdline`:
```
iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432 ttm.page_pool_size=33554432
```

### 1a. `numa_balancing=disable` — APPLY NOW

**Verified:** AMD Instinct OS Tuning guide recommends disabling NUMA balancing for all AMD GPU systems. openSUSE/SUSE kernel docs confirm `numa_balancing=disable` is a valid kernel parameter.

**Rationale:** Strix Halo is a monolithic SoC with unified LPDDR5X. There are no real NUMA nodes. The kernel's automatic NUMA balancing attempts to migrate pages across non-existent nodes, causing severe degradation in APU workload performance. This is pure overhead with zero benefit on this architecture.

**Apply:**
```bash
sudo grub-editenv /boot/grub/grub.cfg set kernel_params="... numa_balancing=disable"
# or via /etc/default/grub:
# GRUB_CMDLINE_LINUX_DEFAULT="... numa_balancing=disable"
```

**Can also set at runtime without reboot:**
```bash
echo 0 | sudo tee /proc/sys/kernel/numa_balancing
```

### 1b. `amdgpu.vm_update_mode=3` — APPLY NOW

**Verified:** Linux kernel GPU docs (cdn.kernel.org/doc/html/latest/gpu/amdgpu/module-parameters.html):
> "Override VM update mode. VM updated by using CPU (0 = never, 1 = Graphics only, 2 = Compute only, 3 = Both). The default is -1 (Only in large BAR(LB) systems Compute VM tables will be updated by CPU, otherwise 0, never)."

**Rationale:** Mode 3 forces CPU-based VM page table updates for both graphics and compute, bypassing the SDMA engine. The SDMA engine is known to hang under heavy VRAM saturation (confirmed in freedesktop bugzilla #102322). Strix Halo IS a large BAR system (128GB unified), so CPU VM updates are the recommended path. Tradeoff: slightly more CPU utilization. Worth it for stability.

**Apply:**
```bash
# Add to GRUB_CMDLINE_LINUX_DEFAULT:
amdgpu.vm_update_mode=3
```

### 1c. `amdgpu.vm_fragment_size=9` — TEST FIRST

**Current state:** NOT set in kernel params at all. Memory says "RECOMMENDED: vm_fragment_size=8" but it was never applied.

**Rationale:** Formula is `page_size * 2^fragment_size`:
- `vm_fragment_size=8` → 4KB * 256 = 1MB fragments
- `vm_fragment_size=9` → 4KB * 512 = 2MB fragments

Larger fragments reduce page table entries the hardware TLB must cache, eliminating micro-stutters during heavy computational loads. The blueprint recommends 9 (2MB). Since Trinity's value was never applied, starting at 9 is reasonable.

**Test:** Apply, reboot, run ComfyUI pipeline + vLLM simultaneously, check for stability.

### 1d. `pci=realloc=off` — TEST FIRST

**Rationale:** Disables PCI resource reallocation. Can prevent BIOS-assigned PCI memory window conflicts on systems with large BAR. Low risk, but verify no PCIe devices drop off after applying.

### 1e. `amdgpu.gttsize` — REVIEW

**Current:** Set to 126976 (124GB).
**Blueprint says:** Omit entirely — claims it "triggers severe deprecation warnings" and "conflicting memory size initialization" on kernel 6.12+ with ROCm 7.x.

**Reality:** Trinity is on kernel 7.0.0-27-generic with ROCm 7.2.0 and it works. Check dmesg for deprecation warnings:
```bash
dmesg | grep -i gttsize
dmesg | grep -i "deprecat"
```
If no warnings, leave as-is. If warnings appear, remove `amdgpu.gttsize` and rely solely on TTM page limits.

---

## 2. GPU Performance State Lock (APPLY NOW)

**Verified:** `power_dpm_force_performance_level` is a documented sysfs attribute (docs.kernel.org/gpu/amdgpu/thermal.html). Setting to "high" locks the GPU to maximum clock speeds, preventing dynamic C-state downclocking.

**Rationale:** During rapid context switches (code → inference → render), dynamic clock scaling causes desynchronization. Locking to high eliminates this.

**Apply:**
```bash
echo 'ACTION=="add", SUBSYSTEM=="drm", DRIVERS=="amdgpu", ATTR{device/power_dpm_force_performance_level}="high"' | sudo tee /etc/udev/rules.d/99-amdgpu-low-latency.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Verify:**
```bash
cat /sys/class/drm/card*/device/power_dpm_force_performance_level
# Should show: high
```

**Note:** This will increase idle power draw slightly. If thermal issues arise, can be reverted by removing the udev rule.

---

## 3. ROCm Mixed-Precision NaN Workarounds (APPLY TO COMFYUI)

### 3a. Large Matmul NaN Bug — CONFIRMED gfx1201, UNCONFIRMED gfx1151

**Source:** ROCm issue #6116 (github.com/ROCm/ROCm/issues/6116)

**Bug:** `torch.mm` and `F.linear` produce data-dependent NaN when input tensor exceeds ~500K rows. Triggered by real model output (sparse convolution features from TRELLIS.2), not random data. Chunking to 100K rows fixes it.

**IMPORTANT CORRECTION:** The blueprint attributes this to "RDNA 3 architectures" and uses a chunk size of 524,288 (512K). The actual bug was filed against **gfx1201 (RDNA 4)**, NOT gfx1151 (RDNA 3.5). The issue explicitly states "Unknown if this affects other gfx12 variants." The blueprint's 524,288 chunk size is ABOVE the 500K trigger threshold — the actual issue recommends 100,000.

**Action for Trinity:** If NaN corruption appears in TRELLIS/Hunyuan3D output on Strix Halo, apply this workaround with **chunk_size=100_000** (not 524,288):

```python
ROCM_SAFE_CHUNK = 100_000  # NOT 524288 — the actual threshold is ~500K rows

def rocm_safe_linear(feats: torch.Tensor, weight: torch.Tensor, bias=None) -> torch.Tensor:
    """F.linear workaround for ROCm large N chunking."""
    N = feats.shape[0]
    if N <= ROCM_SAFE_CHUNK:
        return torch.nn.functional.linear(feats, weight, bias)
    out = torch.empty(N, weight.shape[0], device=feats.device, dtype=feats.dtype)
    for i in range(0, N, ROCM_SAFE_CHUNK):
        chunk_end = min(i + ROCM_SAFE_CHUNK, N)
        out[i:chunk_end] = torch.nn.functional.linear(feats[i:chunk_end], weight, bias)
    return out
```

### 3b. AOTriton bf16 NaN Bug — CONFIRMED gfx1151, DIRECTLY RELEVANT

**Source:** ROCm/aotriton issue #54 (github.com/ROCm/aotriton/issues/54)

**Bug:** The packaged AOTriton binary on gfx1151 selects a lower-precision accumulator for `tl.dot` than the JIT compiler. With bf16 inputs, this produces NaN at certain BLOCK_M x BLOCK_N configurations. The JIT path is clean.

**Relevance:** Trinity uses `TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1` for 19x SDPA speedup. This bug means bf16 flash attention via the AOT binary can produce NaN.

**Workaround (from issue):** Add `out_dtype=tl.float32` to `tl.dot` calls, or use the JIT path instead of the AOT binary. The fix was merged in PR #70 but the packaged AOT binary may still be affected.

**Action for Trinity:** Monitor for NaN in attention outputs when using AOTriton with bf16. If detected:
1. Check if the fix from PR #70 is in the current ROCm 7.2.0 build
2. If not, temporarily disable AOTriton: `unset TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL`
3. Or force fp32 accumulators in custom Triton kernels

---

## 4. `tuned` Performance Profile (APPLY NOW)

**Verified:** Multiple independent Strix Halo guides (ignasivt, hogeheer499, strix-halo-toolboxes.com) confirm +5-8% overall performance improvement. Memory bandwidth improves from ~221 GB/s to ~234 GB/s write. +4-5% token generation improvement measured.

**Rationale:** The `accelerator-performance` profile disables higher latency STOP states (C-states), keeping the CPU ready for immediate compute. This is especially important for Trinity's resident-but-paused architecture where the CPU must rapidly switch between feeding GPU workloads.

**Apply:**
```bash
sudo apt install tuned -y
sudo systemctl enable --now tuned
sudo tuned-adm profile accelerator-performance
```

**IMPORTANT:** Disable `power-profiles-daemon` first — it conflicts with `tuned`:
```bash
sudo systemctl disable --now power-profiles-daemon
```

**Verify:**
```bash
tuned-adm active
# Expected: Current active profile: accelerator-performance
```

---

## 5. Vulkan RADV vs ROCm Backend (CRITICAL FINDING — REVIEW)

**Verified:** 128-run benchmark study (thefrontierlab.ai) + multiple community guides + kyuz0 toolbox benchmarks.

**Finding:** On gfx1151 (Strix Halo), **Vulkan RADV beats ROCm/HIP by 25-32% on token generation** for MoE models. Vulkan also wins prefill by 6-8% at longer contexts.

| Backend | tg128 (t/s) | Effective BW | % of 256 GB/s peak |
|---------|------------|-------------|-------------------|
| ROCm/HIP | 33.7 | ~101 GB/s | ~40% |
| Vulkan | 49.5 | ~149 GB/s | ~58% |

**Root cause (llama.cpp issue #24438):** ROCm/HIP achieves only ~40% of memory bandwidth on gfx1151 for MoE token generation. 92% of GPU time is in `mul_mat_vec_q` (quantized GEMV). The issue is in the GEMV memory-access implementation, not launch geometry.

**Backend selection guide (from community data):**
- **Token generation (interactive):** Vulkan RADV wins by 25-32%
- **Prompt processing (batch):** ROCm 7.x nightlies win by ~10% (hipBLASLt fusion)
- **Long context (32K+):** ROCm with rocWMMA-tuned branch can beat Vulkan (2X at 32K)
- **Gemma models on ROCm 6.4.4:** FAILS — 48 of 48 runs fail on Gemma 4 31B Dense (hipGraphInstantiate OOM, degenerate output). Vulkan runs cleanly.

**Impact on Trinity:**
- P (DiffusionGemma 26B) uses vLLM/ROCm. If this is a Gemma-family model, the ROCm failure risk is real. **Consider testing Vulkan backend for P.**
- H (Hermes 4 70B via llama.cpp) — Vulkan RADV would give ~25% better generation throughput than ROCm. Since H is used for planning (generation-heavy, not batch), **Vulkan is the better backend for H.**
- ComfyUI must stay on ROCm (PyTorch/ROCm dependency, no Vulkan path)

**Action:** Test llama.cpp Vulkan build for H (Hermes 70B). If stable and faster, switch from ROCm to Vulkan RADV.

---

## 6. llama.cpp gfx1151 Prefill Optimization (TEST FIRST)

**Verified:** llama.cpp issue #21284 (github.com/ggml-org/llama.cpp/issues/21284)

**Bug:** Default MMQ parameters for gfx1151 are suboptimal, causing VGPR spilling that exceeds the 256 VGPR limit. ~20% prefill performance left on the table.

**Fix (from issue):**
1. **MMQ defaults:** Set `mmq_x=48, mmq_y=64, nwarps=4` for gfx1151 (currently suboptimal defaults)
2. **Intrinsics:** Replace `expf()` with `__expf()` in MoE routing and SiLU activation
3. **Intrinsics:** Replace `roundf()` with `__float2int_rn` in quantize.cu
4. **Intrinsics:** Use `__builtin_amdgcn_sudot4` for RDNA 3.5 in `ggml_cuda_dp4a`

**Impact:** ~20% prefill uplift on 122B MoE models. Tested by multiple Strix Halo community members.

**Action:** Check if these fixes have been merged into llama.cpp master. If not, apply the patch from the issue gist when building llama.cpp for H.

---

## 7. NPU XDNA 2 Breakthrough (REFERENCE — FUTURE INTEGRATION)

**Verified:** Multiple GitHub repos (halo project, sypherin/strix-halo-setup, amd/RyzenAI-SW issue #366, bong-water-water-bong/1bit-systems)

**MAJOR FINDING:** The XDNA 2 NPU on Strix Halo is now usable on Linux via open-source toolchain, bypassing AMD's proprietary SDK (which doesn't support STX-H on Linux).

**What works (proven on silicon):**
- **IRON + MLIR-AIE + Peano + XRT** toolchain — fully open-source path to NPU compute
- **Zero-copy GPU<->NPU** — one host allocation, `hipHostRegister`'d for iGPU and wrapped as XRT userptr buffer for NPU. Maps to same physical pages. Verified byte-exact at 1/64/256 MiB.
- **int8 GEMM on NPU** — 2.96-3.33x speedup over CPU for MatMulInteger operations
- **ggml backend for llama.cpp** — NPU appears as `NPU: AMD XDNA2 NPU (Strix Halo, aie2p)` in `llama-server --list-devices`. Intercepts `MUL_MAT`, quantizes to int8, dispatches to precompiled xclbins.
- **Full transformer layer** — int8 GEMM -> quantized Linear -> FFN -> attention -> complete transformer layer -> 4-layer forward -> KV-cached generation -> speculative decode. All verified vs fp32.

**Requirements:**
- `amd/xdna-driver` (DKMS, out-of-tree, v2.23.0+)
- XRT 2.23.0+ (built from submodule)
- `ulimit -l unlimited` (memlock >= 100 MB)
- Firmware: `/usr/lib/firmware/amdnpu/17f0_11/` (npu.sbin.1.0.0.166+)
- Kernel 7.0.0+ (Trinity already has this)

**Relevance to Trinity:**
- **FACES-Embed target:** The NPU can run INT8 matmul at 2-3x CPU speed. FACES-Embed (DistilBERT-base, ~66M params, INT8) is a perfect candidate for NPU offload.
- **Zero-copy architecture:** NPU and GPU share the same LPDDR5X with no copy overhead. This aligns perfectly with Trinity's unified memory model.
- **llama.cpp NPU backend:** Could offload some matmul operations from H (Hermes 70B) to the NPU, freeing GPU bandwidth.
- **Rust FFI path:** Community is building Rust FFI shims for NPU dispatch — no Python needed at runtime.

**Not actionable now** — requires building XRT driver and toolchain. File as reference for Phase 10+ (FACES-Embed NPU target) and future H acceleration.

---

## 8. vLLM and ROCm Environment Variables (APPLY NOW)

**Verified:** epheo.eu Strix Halo vLLM guide + community guides.

### 8a. `HIP_FORCE_DEV_KERNARG=1` — APPLY NOW

**Required for Strix Halo's memory model.** Without this, vLLM may fail to initialize or behave unpredictably with large kernels on gfx1151.

### 8b. `LD_PRELOAD=libtcmalloc_minimal` — APPLY NOW

**Significant inference throughput gain** for vLLM. Replaces glibc malloc with tcmalloc, which has better multi-threaded allocation performance.

```bash
export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libtcmalloc_minimal.so.4
```

### 8c. `-mllvm --amdgpu-unroll-threshold-local=600` — TEST FIRST

**LLVM unroll workaround** that restores ROCm 7+ performance. Without this, ROCm 7.x can be slower than 6.4.x due to aggressive loop unrolling on gfx1151.

### 8d. `amdgpu.cwsr_enable=0` — TEST FIRST

Disables compute wave save/restore. Not needed for LLM workloads (no graphics preemption during compute). Frees a small amount of VRAM and reduces context switch overhead.

---

## 9. Kernel 7.0 Performance Confirmation (ALREADY APPLIED)

**Verified:** sypherin/strix-halo-setup benchmarks.

Trinity is already on kernel 7.0.0-27-generic. Community data confirms this was the right choice:

| Metric | Kernel 6.19.9 | Kernel 7.0-rc6 | Change |
|--------|--------------|----------------|--------|
| pp (prompt processing) | 287-351 t/s | 393 t/s | **+12-37%** |
| tg (token generation) | 22-23 t/s | 22 t/s | No change |

Kernel 7.0 significantly improves prompt processing via RADV/Vulkan improvements. Token generation is memory-bandwidth bound and unchanged (as expected). Trinity is already benefiting from this.

---

## 10. llama.cpp Build Freshness (MAINTENANCE)

**Verified:** Multiple community guides. hogeheer499 guide: b8298 to b8460 = **+25% on MoE models** (14% generic + 11% Vulkan-specific). This is the single biggest optimization, more than all kernel tuning combined.

**Action:** Ensure llama.cpp builds for H (Hermes 70B) are kept current. Check for updates monthly. The FA (Flash Attention) refactor and graphics queue improvements in recent builds are specifically beneficial for gfx1151.

---

## 11. Flash Attention Required (VERIFY)

**Verified:** sypherin/strix-halo-setup, multiple guides.

`-fa 1` (Flash Attention) is **required** for good performance on Strix Halo. Without it, performance degrades significantly at longer contexts.

**Also:** `--ubatch-size 1024` is optimal (vs default 512). No benefit at 2048.

**Action:** Verify H (Hermes 70B) llama.cpp launch includes `-fa 1` and `--ubatch-size 1024`.

---

## 12. SpatialClaw Architecture (REFERENCE — FUTURE INTEGRATION)

**Verified:** github.com/NVlabs/SpatialClaw — real project from NVIDIA Research.

**What it is:** Training-free framework for agentic spatial reasoning. Replaces JSON tool-calling with raw Python code as the action interface. Uses a persistent Jupyter kernel where variables, point clouds, and matrices persist across steps.

**Five-stage loop:** Planning → Code Generation → Execution → Feedback Assembly → Answer Submission

**Key components:**
- Persistent Jupyter kernel (via hamelnb or agent-jupyter-toolkit)
- SAM 3 (Meta) for 2D segmentation — 848M param DETR-based
- Depth Anything 3 (ByteDance) for monocular depth estimation
- Back-projection math: 2D pixels + depth → 3D point clouds via camera intrinsics
- AST sanitizer for code safety before execution
- VLM served via vLLM

**Results:** 59.9% average accuracy across 20 spatial reasoning benchmarks.

**Relevance to Trinity:**
- Trinity's agent loop (`max_turns=200`) currently uses JSON tool-calling. SpatialClaw's code-as-action pattern could be a future evolution for spatial reasoning jobs.
- The persistent kernel concept aligns with Trinity's "resident-but-paused" philosophy — keep state in memory, avoid reload cycles.
- SAM 3 + DA3 pipeline is directly relevant to Trinity's XR canvas (EYE phase on Android XR).
- The AST sanitizer pattern is relevant to Trinity's security hardening (Phase 5).

**Integration path (future):**
1. Add a Jupyter kernel tool to Trinity's agent loop
2. Pre-load perception primitives (SAM 3, DA3) as callable functions
3. Route spatial reasoning jobs to the kernel-backed execution path
4. Keep JSON tool-calling for non-spatial jobs (backwards compatible)

**Not actionable now** — file as reference for Phase 12 (Daydream XR test) and beyond.

---

## 5. Godot/StereoKit/3DGS (REFERENCE ONLY — DIFFERENT STACK)

Trinity uses **Bevy/Rust** for 3D/XR, not Godot/C#. These sections are architecturally interesting but not directly applicable:

- **Godot Vulkan Mobile renderer + MSAA 4x + foveated rendering** — The rendering concepts (subpass architecture, MSAA tradeoffs, foveation collapse with post-processing) transfer to Bevy's Vulkan renderer. Worth referencing when implementing Bevy XR rendering.
- **StereoKit** — C#/.NET, not applicable to Trinity's Rust stack.
- **3D Gaussian Splatting via gdgs Godot plugin** — Would need a Bevy equivalent. The math (covariance matrices, spherical harmonics, alpha-blending) is stack-agnostic.
- **Fedora toolbx** — Trinity uses podman on Ubuntu. Same isolation concept, different tooling.

---

## 13. Summary: Priority Action List

| # | Item | Priority | Effort | Risk |
|---|------|----------|--------|------|
| 1 | `numa_balancing=disable` kernel param | HIGH | 1 min | None |
| 2 | `amdgpu.vm_update_mode=3` kernel param | HIGH | 1 min | Low (slight CPU increase) |
| 3 | udev rule for GPU performance lock | HIGH | 2 min | Low (idle power) |
| 4 | `tuned accelerator-performance` profile | HIGH | 5 min | Low (conflicts with power-profiles-daemon) |
| 5 | `HIP_FORCE_DEV_KERNARG=1` for vLLM | HIGH | 1 min | None |
| 6 | `LD_PRELOAD=libtcmalloc_minimal` for vLLM | HIGH | 1 min | None |
| 7 | Test Vulkan RADV backend for H (Hermes) | HIGH | 2 hr | Could gain 25% tg |
| 8 | `amdgpu.vm_fragment_size=9` kernel param | MEDIUM | 1 min + reboot | Low |
| 9 | Check dmesg for gttsize deprecation | MEDIUM | 1 min | None |
| 10 | Verify `-fa 1` and `--ubatch-size 1024` on H | MEDIUM | 5 min | None |
| 11 | Check llama.cpp build freshness (monthly) | MEDIUM | Ongoing | None |
| 12 | `-mllvm --amdgpu-unroll-threshold-local=600` | MEDIUM | 1 min | Low |
| 13 | AOTriton bf16 NaN monitoring | MEDIUM | Ongoing | Existing risk |
| 14 | `amdgpu.cwsr_enable=0` kernel param | LOW | 1 min + reboot | Low |
| 15 | ROCM_SAFE_CHUNK in ComfyUI (if NaN appears) | LOW | 15 min | Only if bug confirmed on gfx1151 |
| 16 | llama.cpp gfx1151 prefill patch (issue #21284) | LOW | 30 min | Check if merged upstream |
| 17 | NPU XDNA 2 toolchain (XRT + IRON) | FUTURE | Days | Reference for FACES-Embed |
| 18 | SpatialClaw architecture study | FUTURE | Future | Reference for XR phase |

**Items 1-6 can be applied immediately** without reboot. Item 7 (Vulkan backend test for H) is the highest-impact change to investigate — a 25% generation throughput gain for Hermes 70B would significantly improve Trinity's planning phase latency. Items 8-9, 14 require a reboot. Items 13, 15 are conditional on observing NaN in production. Items 17-18 are future reference.
