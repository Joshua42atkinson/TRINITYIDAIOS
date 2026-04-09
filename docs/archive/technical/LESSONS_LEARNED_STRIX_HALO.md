# Lessons Learned — Strix Halo Inference

**Updated:** 2026-03-20

## Critical: RADV_PERFMODE=nogttspill

**Problem:** AMD Strix Halo APU with RADV (Mesa) Vulkan driver caps each memory heap budget at ~40GB, even though the physical heap is 83GB. Models >64GB hang during Vulkan buffer allocation.

**Solution:** Set `RADV_PERFMODE=nogttspill` environment variable BEFORE launching longcat-sglang or any Vulkan inference process. This disables GTT spilling, allowing full heap usage.

```bash
RADV_PERFMODE=nogttspill longcat-sglang --model ... --n-gpu-layers 999 --no-mmap
```

**Source:** `crates/archive/trinity-sidecar-llama-cpp/src/main.rs` line 48-50

## Critical: --no-mmap for Unified Memory

**Problem:** Memory mapping (`mmap`) can cause hangs when model size approaches GTT memory limits on unified memory systems.

**Solution:** Always use `--no-mmap` flag for longcat-sglang on Strix Halo.

## Critical: Kernel Parameters

Must be set in GRUB for unified memory access:
```
iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432
```

## Known Issue: llama-cpp-2 Rust Crate

The `llama-cpp-2` crate (v0.1.139) bundles an older version of llama.cpp that hangs during Vulkan initialization with KHR_coopmat on GFX1151 (Strix Halo). The standalone `longcat-sglang` binary built from latest llama.cpp git works fine.

**Workaround:** Use standalone `longcat-sglang` binary via HTTP (sidecar pattern), not embedded FFI.

**Root cause:** Crate's bundled llama.cpp version doesn't properly handle GFX1151 cooperative matrix shader compilation.

## Reference: kyuz0/amd-strix-halo-toolboxes

The definitive community resource for Strix Halo + llama.cpp:
- https://github.com/kyuz0/amd-strix-halo-toolboxes
- Recommends Fedora 42, kernel 6.18.6+
- Vulkan RADV backend recommended over ROCm for stability
