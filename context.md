# TRINITY EXTREME DEPLOYMENT CONTEXT: LongCat-Next on vLLM

**Current Phase:** Quantization and Multi-Modal Deployment Preparation  
**Target:** AMD Strix Halo (APU), 128GB Unified Memory  
**Hardware Engine:** Radeon 8060S (`gfx1151`) / Ryzen AI Max+ 395  
**Current Driver Suite:** ROCm 7.2.1 (Mainline Stable Release - Nightlies Deprecated)

## 1. Where We Are Right Now (April 2026 Shift)
We have officially deprecated the Docker-based SGLang and vLLM SGLang container runtimes for end-user deployment. The Trinity AI OS target architecture now completely relies on **Tauri Sidecar Proxies** utilizing `uv` environment mapping. This abstracts `PyTorch` and `vLLM` locally on macOS/Windows/Linux endpoints natively without requiring hypervisors.

*   **Offline Quantization Failure:** The offline compression pipeline (`quantize_quark.py`) inside `quark-forge` failed during the GEMM scalar layer crunch due to breaking API changes in the AMD Quark v0.11 ROCm 7.2.1 library. 
    *   **The Bug:** Passing `QuantizationConfig(weight=Int4PerGroupSpec(...))` triggers `TypeError: 'Int4PerGroupSpec' object is not iterable` at runtime because the library changed its structural mapping requirements.

## 2. Next Session Plan: Quark API Investigation
Before any end-user deployment can finish, we must compile the compressed format correctly:
1.  **Resolve AMD Quark v0.11 API Bug:** The next session *must* investigate the precise schema `quark.torch.quantization.config` requires for defining `awq` weighting on matrices. You should construct a simple interactive `dir()` block query against the library inside the `quark-forge` distrobox, observing the source code of `QuantizationConfig`.
2.  **Compile & Cache AWQ:** Once the python bug is bypassed, allow the pipeline to compress `LongCat-Next-74B-MoE` into `~/trinity-models/vllm/LongCat-Next-AWQ-4bit`.
3.  **Validate `trinity-sidecar-boot.sh`:** Our zero-docker distribution logic runs fully offline on end-user machines. We need to verify that `uv` successfully sequesters the PyTorch and vLLM dependencies before launching Port 8010.

## 3. Essential Guardrails for the Next Agent
*   **DO NOT REVERT TO GGUF/LLAMA.CPP:** We explicitly investigated compiling LongCat to `.gguf`. While `llama.cpp` handles multimodal image *input*, it strips the DiNA discrete decoders responsible for **generating** images and audio tokens. We *must* use `vLLM` in our sidecar to preserve generation capabilities.
*   **DO NOT OVER-COMPLICATE TAURI:** Tauri triggers `trinity-sidecar-boot.sh` blindly. Do not try to inject Docker logic into the startup sequence.
*   **STAY ON ROCM 7.2.1:** Nightly drivers have been fully purged from the system loop.
