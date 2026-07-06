---
description: Strix Halo optimization — How To guide for all actionable kernel, ROCm, vLLM, and llama.cpp tuning items from ACTIONABLE_RESEARCH_STRIX_HALO.md. Run in order, top to bottom.
---

# Strix Halo Optimization — How To

## Objective

Apply all verified optimizations from `docs/active/ACTIONABLE_RESEARCH_STRIX_HALO.md` to the GMKtec Evo X2 (AMD Ryzen AI Max+ 395, 128GB LPDDR5X, gfx1151). Items are ordered by priority and grouped by whether they need a reboot.

**Reference doc:** `docs/active/ACTIONABLE_RESEARCH_STRIX_HALO.md`
**Current kernel:** 7.0.0-27-generic
**Current ROCm:** 7.2.0
**Current Vulkan driver:** Mesa RADV 26.0.3

## Prerequisites

- Read `docs/active/ACTIONABLE_RESEARCH_STRIX_HALO.md` for rationale and source citations
- Back up current GRUB config: `sudo cp /etc/default/grub /etc/default/grub.bak`
- Note current kernel params: `cat /proc/cmdline`
- `cargo check` passes (baseline — ensure Trinity builds before system changes)

## Current State (verified July 5, 2026)

Already applied in `/proc/cmdline`:
```
iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432 ttm.page_pool_size=33554432
```

Already set at runtime (but NOT in GRUB — won't survive reboot):
- `numa_balancing=0`

Not yet applied (defaults):
- `amdgpu.vm_update_mode=-1` (default)
- `amdgpu.vm_fragment_size=-1` (default)
- `amdgpu.cwsr_enable=1` (default)
- GPU power level: `auto`
- `tuned`: not installed, `power-profiles-daemon` active
- `tcmalloc`: not installed

---

## Phase 1: Immediate Changes (No Reboot Required)

### 1.1 — Disable NUMA Balancing (Runtime)

Already set at runtime to 0, but let's make it explicit:

```bash
echo 0 | sudo tee /proc/sys/kernel/numa_balancing
```

**Verify:**
```bash
cat /proc/sys/kernel/numa_balancing
# Expected: 0
```

### 1.2 — Install `tuned` and Apply Accelerator-Performance Profile

```bash
sudo systemctl disable --now power-profiles-daemon
sudo apt install tuned -y
sudo systemctl enable --now tuned
sudo tuned-adm profile accelerator-performance
```

**Verify:**
```bash
tuned-adm active
# Expected: Current active profile: accelerator-performance
```

**Risk:** Conflicts with `power-profiles-daemon` — that's why we disable it first. If thermal issues arise, revert with `sudo tuned-adm profile balanced` or uninstall tuned and re-enable PPD.

### 1.3 — GPU Performance State Lock (udev Rule)

```bash
echo 'ACTION=="add", SUBSYSTEM=="drm", DRIVERS=="amdgpu", ATTR{device/power_dpm_force_performance_level}="high"' | sudo tee /etc/udev/rules.d/99-amdgpu-low-latency.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Verify:**
```bash
cat /sys/class/drm/card*/device/power_dpm_force_performance_level
# Expected: high
```

**Risk:** Slightly higher idle power draw. If thermals become an issue, remove the rule:
```bash
sudo rm /etc/udev/rules.d/99-amdgpu-low-latency.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### 1.4 — Install tcmalloc for vLLM

```bash
sudo apt install libgoogle-perftools4 libtcmalloc-minimal4 -y
```

**Verify:**
```bash
ls /usr/lib/x86_64-linux-gnu/libtcmalloc_minimal.so.4
# Should exist
```

### 1.5 — Set vLLM Environment Variables

Add these to the DiffusionGemma startup script (`~/trinity-models/start-diffusiongemma.sh`):

```bash
export HIP_FORCE_DEV_KERNARG=1
export LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libtcmalloc_minimal.so.4
```

**Verify:** Restart vLLM/podman with the new env vars and check it starts cleanly:
```bash
# After restarting the vLLM container
curl http://127.0.0.1:8000/v1/models
# Should return model list
```

### 1.6 — Check dmesg for gttsize Deprecation

```bash
dmesg | grep -i gttsize
dmesg | grep -i "deprecat"
```

**If no warnings:** Leave `amdgpu.gttsize=126976` as-is.
**If warnings appear:** Plan to remove `amdgpu.gttsize=126976` from GRUB in Phase 2 and rely solely on TTM page limits.

---

## Phase 2: GRUB Kernel Parameters (Requires Reboot)

### 2.1 — Update GRUB

Edit `/etc/default/grub` and append to `GRUB_CMDLINE_LINUX_DEFAULT`:

```
numa_balancing=disable amdgpu.vm_update_mode=3 amdgpu.vm_fragment_size=9
```

**Full expected GRUB_CMDLINE_LINUX_DEFAULT** (your existing params + new ones):
```
quiet splash iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432 ttm.page_pool_size=33554432 numa_balancing=disable amdgpu.vm_update_mode=3 amdgpu.vm_fragment_size=9
```

**Optional (test first — add only if Phase 2.2 goes well):**
```
pci=realloc=off amdgpu.cwsr_enable=0
```

### 2.2 — Apply and Reboot

```bash
sudo update-grub
sudo reboot
```

### 2.3 — Post-Reboot Verification

```bash
# Verify all kernel params took effect
cat /proc/cmdline
# Should contain all new params

# Check amdgpu module params
cat /sys/module/amdgpu/parameters/vm_update_mode
# Expected: 3

cat /sys/module/amdgpu/parameters/vm_fragment_size
# Expected: 9

cat /sys/module/amdgpu/parameters/cwsr_enable
# Expected: 0 (if you added it)

# Check NUMA balancing
cat /proc/sys/kernel/numa_balancing
# Expected: 0

# Check GPU performance lock survived reboot
cat /sys/class/drm/card*/device/power_dpm_force_performance_level
# Expected: high

# Check tuned profile survived reboot
tuned-adm active
# Expected: accelerator-performance

# Check dmesg for any new warnings
dmesg | grep -iE "amdgpu|ttm|gttsize|deprecat|error" | head -20
```

### 2.4 — Stability Test

Run these workloads and verify no crashes or NaN:

1. **ComfyUI pipeline:** Generate an image with Janus-Pro-7B. Check output for corruption.
2. **vLLM inference:** Send a test prompt to DiffusionGemma. Check response quality.
3. **If Hermes is running:** Send a test prompt. Check for NaN in output.
4. **Monitor thermals:**
```bash
rocm-smi --showtemp --showpower
# Watch for thermal throttling (>90°C) or power spikes (>120W)
```

---

## Phase 3: llama.cpp / Hermes Optimization

### 3.1 — Verify Flash Attention and ubatch

Check the Hermes 70B launch command includes:

```bash
-fa 1 --ubatch-size 1024
```

**Full recommended launch for H (Hermes 4 70B Q3_K_M):**
```bash
llama-server \
  -m /path/to/Hermes-4-70B-Q3_K_M.gguf \
  --host 127.0.0.1 --port 8002 \
  -ngl 99 \
  -c 32768 \
  --cache-type q8_0 \
  -fa 1 \
  --ubatch-size 1024 \
  --no-mmap \
  -t 8
```

**Env vars for ROCm backend:**
```bash
export GGML_HIP_ROCWMMA_FATTN=1
export ROCBLAS_USE_HIPBLASLT=1
export HSA_OVERRIDE_GFX_VERSION=11.5.1
```

### 3.2 — Check llama.cpp Build Freshness

```bash
# If llama.cpp is installed from source, check current build
cd /path/to/llama.cpp
git log --oneline -1
git fetch origin
git log --oneline HEAD..origin/main | wc -l
# If >50 commits behind, rebuild
```

**Build with gfx1151 support:**
```bash
git pull origin main
mkdir -p build && cd build
cmake .. -DGGML_HIP=ON -DAMDGPU_TARGETS=gfx1151 -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
```

### 3.3 — Test Vulkan RADV Backend for Hermes (HIGH IMPACT)

This is the highest-impact optimization to test. Research shows 25-32% token generation improvement on gfx1151.

**Build llama.cpp with Vulkan backend:**
```bash
cd /path/to/llama.cpp
mkdir -p build-vulkan && cd build-vulkan
cmake .. -DGGML_VULKAN=ON -DGGML_VULKAN_RUN_TESTS=ON -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
```

**Benchmark ROCm vs Vulkan:**
```bash
# ROCm baseline (current)
./build/bin/llama-bench -m /path/to/Hermes-4-70B-Q3_K_M.gguf -p 512 -n 128 -ngl 99

# Vulkan
./build-vulkan/bin/llama-bench -m /path/to/Hermes-4-70B-Q3_K_M.gguf -p 512 -n 128 -ngl 99
```

**Compare `tg128` (token generation) values.** If Vulkan is >15% faster:
1. Switch H to use the Vulkan build
2. Add `RADV_PERFMODE=nogttspill` to the env (critical for models approaching 64GB)
3. Update `[hotel.H]` config to use the Vulkan binary path

**Vulkan env vars:**
```bash
export RADV_PERFMODE=nogttspill
# No HSA_OVERRIDE needed — Vulkan doesn't use ROCm runtime
```

### 3.4 — Check gfx1151 Prefill Patch (Issue #21284)

```bash
# Check if the MMQ fix has been merged upstream
cd /path/to/llama.cpp
git log --oneline --all --grep="gfx1151" | head -10
git log --oneline --all --grep="mmq" | head -10
git log --oneline --all --grep="21284" | head -10
```

If NOT merged, the patch from the issue provides ~20% prefill uplift. See `ACTIONABLE_RESEARCH_STRIX_HALO.md` §6 for the specific changes (mmq_x=48, mmq_y=64, nwarps=4, intrinsic replacements).

### 3.5 — LLVM Unroll Threshold (ROCm Only)

If staying on ROCm backend for any model, add to vLLM/llama.cpp env:

```bash
export PYTORCH_HIP_ALLOC_CONF="max_split_size_mb:256"
# For ROCm compile flags:
# -mllvm --amdgpu-unroll-threshold-local=600
```

**Note:** This is a compile-time flag for ROCm, not a runtime env var. It needs to be set when building ROCm-dependent code or passed via `PYTORCH_ROCM_ARCH` overrides. Test first — measure before and after.

---

## Phase 4: Config File Updates (Trinity Codebase)

### 4.1 — Update `configs/hardware/strix_halo.toml`

The `model.path` still points to `Nemotron-3-Super-120B` (stale). Update to reflect the actual resident models:

```toml
[model]
# P (DiffusionGemma) is managed by vLLM/podman, not this config
# H (Hermes 4 70B) is managed by Hotel Manager
path = "/home/joshua/trinity_models/Hermes-4-70B-Q3_K_M.gguf"
n_gpu_layers = 999
context_size = 32768
batch_size = 1024
n_threads = 8
```

### 4.2 — Add `[hotel.H]` Section to `configs/runtime/default.toml`

Add Hermes as a hotel resident. This is the resident-but-paused architecture:

```toml
[hotel.H]
name = "H (Hermes — Planning)"
port = 8002
binary = "/path/to/llama-server"
args = [
    "-m", "/home/joshua/trinity_models/Hermes-4-70B-Q3_K_M.gguf",
    "--host", "127.0.0.1",
    "--port", "8002",
    "-ngl", "99",
    "-c", "32768",
    "--cache-type", "q8_0",
    "-fa", "1",
    "--ubatch-size", "1024",
    "--no-mmap",
    "-t", "8",
]
working_dir = "/home/joshua/llama.cpp"
health_path = "/health"
health_timeout_secs = 120
cpu_threads = "8-15"
vram_budget_mb = 49152
always_resident = true

[hotel.H.env]
HSA_OVERRIDE_GFX_VERSION = "11.5.1"
GGML_HIP_ROCWMMA_FATTN = "1"
ROCBLAS_USE_HIPBLASLT = "1"
# If using Vulkan backend instead of ROCm:
# RADV_PERFMODE = "nogttspill"
```

### 4.3 — Update `[inference.backends]` for H

Add Hermes as a backend in `configs/runtime/default.toml`:

```toml
[inference.backends.hermes]
url = "http://127.0.0.1:8002"
supports_tools = true
supports_vision = false
party_role = "H"
always_resident = true
```

### 4.4 — Remove Stale R/T Backend Configs

The R (research) and T (tempo-music) backends are no longer used. Remove or comment out:
- `[inference.backends.research]`
- `[inference.backends.tempo-music]`

---

## Phase 5: Ongoing Monitoring

### 5.1 — AOTriton bf16 NaN Watch

Monitor ComfyUI logs for NaN in attention outputs. If detected:

```bash
# Check if AOTriton fix from PR #70 is in current ROCm
/opt/rocm/bin/rocm-smi --showversion
# Check AOTriton version
```

**If NaN appears:**
1. Temporarily disable: `unset TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL`
2. Or force fp32 accumulators in custom Triton kernels
3. Restart ComfyUI

### 5.2 — ROCm Large Matmul NaN Watch

Monitor TRELLIS/Hunyuan3D output for corruption. If NaN appears in large tensor operations:

Apply the chunking workaround from `ACTIONABLE_RESEARCH_STRIX_HALO.md` §3a with `chunk_size=100_000`.

### 5.3 — Monthly llama.cpp Update Check

```bash
# Add to monthly calendar
cd /path/to/llama.cpp
git fetch origin
git log --oneline HEAD..origin/main | wc -l
# If >50 commits behind, rebuild and benchmark
```

---

## Phase 6: Future Reference (Not Actionable Now)

### 6.1 — NPU XDNA 2 Toolchain

**When actionable:** When building FACES-Embed (INT8 DistilBERT) for NPU offload.

**What's needed:**
- Build XRT driver (DKMS, v2.23.0+)
- Install IRON + MLIR-AIE + Peano toolchain
- Set `ulimit -l unlimited`
- Verify firmware at `/usr/lib/firmware/amdnpu/17f0_11/`

**See:** `ACTIONABLE_RESEARCH_STRIX_HALO.md` §7 for full details.

### 6.2 — SpatialClaw Architecture

**When actionable:** Phase 12 (Daydream XR test) and beyond.

**See:** `ACTIONABLE_RESEARCH_STRIX_HALO.md` §12 for integration path.

### 6.3 — Godot/StereoKit/3DGS Rendering Concepts

**When actionable:** When implementing Bevy XR rendering.

**See:** `ACTIONABLE_RESEARCH_STRIX_HALO.md` §5 for transferable concepts.

---

## Verification Checklist

After completing all phases, verify:

- [ ] `numa_balancing=disable` in `/proc/cmdline` (survives reboot)
- [ ] `amdgpu.vm_update_mode=3` in `/sys/module/amdgpu/parameters/vm_update_mode`
- [ ] `amdgpu.vm_fragment_size=9` in `/sys/module/amdgpu/parameters/vm_fragment_size`
- [ ] GPU power level: `high` at `/sys/class/drm/card*/device/power_dpm_force_performance_level`
- [ ] `tuned-adm active` shows `accelerator-performance`
- [ ] `tcmalloc` installed at `/usr/lib/x86_64-linux-gnu/libtcmalloc_minimal.so.4`
- [ ] `HIP_FORCE_DEV_KERNARG=1` in vLLM startup script
- [ ] `LD_PRELOAD=libtcmalloc_minimal.so.4` in vLLM startup script
- [ ] No dmesg warnings for gttsize deprecation
- [ ] Hermes launch includes `-fa 1 --ubatch-size 1024 --no-mmap -ngl 99`
- [ ] llama.cpp build is current (<50 commits behind main)
- [ ] Vulkan RADV benchmarked against ROCm for Hermes (decision documented)
- [ ] `configs/hardware/strix_halo.toml` model path updated
- [ ] `configs/runtime/default.toml` has `[hotel.H]` section
- [ ] Stale R/T backend configs removed or commented out
- [ ] ComfyUI runs without NaN (TRELLIS, HunyuanVideo tested)
- [ ] vLLM runs without errors after env var changes
- [ ] Thermals stable under load (`rocm-smi --showtemp --showpower`)

---

## Rollback Plan

If any change causes instability:

### Rollback Phase 1 (no reboot needed):
```bash
# Revert tuned
sudo tuned-adm profile balanced
sudo systemctl enable --now power-profiles-daemon

# Revert GPU lock
sudo rm /etc/udev/rules.d/99-amdgpu-low-latency.rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Revert vLLM env vars — edit startup script, remove HIP_FORCE_DEV_KERNARG and LD_PRELOAD
```

### Rollback Phase 2 (requires reboot):
```bash
sudo cp /etc/default/grub.bak /etc/default/grub
sudo update-grub
sudo reboot
```

### Rollback Phase 3 (llama.cpp):
```bash
# Switch back to ROCm build if Vulkan was unstable
# Remove -fa 1 if it causes issues (unlikely but possible with very old builds)
```

### Rollback Phase 4 (config files):
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS
git checkout configs/hardware/strix_halo.toml configs/runtime/default.toml
```
