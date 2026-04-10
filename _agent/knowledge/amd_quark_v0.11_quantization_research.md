# AMD Quark v0.11: Quantization Research & Architectural Logic Book

This document is the exclusive, unabridged research log tracking the extreme edge-case failures, tracebacks, engineering theories, and technical bypasses required to forcefully compile the 74 Billion parameter Longcat-Next multimodal framework down to an offline PyTorch-native 4-bit INT4 AWQ network via **AMD Quark**.

## 1. Trinity OS: Instructional Design & Engineering Context 

**The Pedagogical Goal:** Trinity AI OS is designed specifically for deep Socratic protocol, instructional design meaning-making, and agentic trajectory corrections via localized pedagogical methodologies. We explicitly rely on the enormous 74B parameter Mixture-of-Experts (MoE) scale to ensure the AI's "reasoner" has enough contextual volume to comprehend multi-layered educational nuances and redirect the user flawlessly locally without relying on Cloud API constraints.
**The Engineering Challenge:** A 74B parameter framework in raw formatting consumes upwards of `~145GB` of VRAM. To squeeze this onto an AMD Strix Halo consumer APU with 128GB of Unified RAM, we **must** quantize the active neural footprint down to `~38GB` (INT4 representations).
**Why AMD Quark?** Instead of using lazy, real-time quantization overhead (`bitsandbytes` `nf4` load-shifting), which severely cripples latency, we utilize Quark's AWQ (Activation-Aware Weight Quantization) module. Quark natively traces the model against actual localized calibration datasets (like WikiText), mathematically evaluating the error-loss profile of every single layer during matrix compression, permanently sealing an optimized hardware-native integer network.

---

## 2. Advanced Tracebacks, Bugs, and Architectural Fixes

### 2.1 The Native Monolithic Infrastructure Crash
Before Quark could initialize, the pure PyTorch execution layout instantly failed. We were trapped behind rigid Python virtual-environments dropping C++ compilation variables.

1.  **The Meta-Tensor Load Crash:** 
    *   `RuntimeError: Tensor.item() cannot be called on meta tensors`.
    *   *Engineering Perspective:* The Hugging Face `transformers` layout naturally maps heavy weights onto the "meta" device to prevent instant OOM crashes, building a geometric map first before streaming actual float weights. Longcat-Next relies on dynamically evaluated generation checkpoints. It forcefully evaluated parameter checks against "hollow" meta-tensors prior to initialization. 
    *   *The Fix:* We used `accelerate` hooks to mechanically scan the full repository space, overriding the `meta` initialization by forcing `accelerate.utils.set_module_tensor_to_device` to explicitly hydrate the GPU nodes synchronously prior to generation locks.
2.  **Flash Attention Shim Defect:**
    *   *Error:* Immediate Python process segmentation fault triggered on `import flash_attn`.
    *   *Engineering Perspective:* Deep learning repositories aggressively seek Nvidia CUDA binaries unconditionally. Because we are orchestrating ROCm (AMD), the model immediately violently segfaulted. 
    *   *The Fix:* We engineered a native PyTorch Scaled Dot Product Attention (SDPA) wrapper shim to permanently spoof the `flash_attn` initialization parameter requirement, silently routing backend calls manually.

---

### 2.2 AMD Quark V0.11 Native Compiler Failures
Once PyTorch stabilized, we booted the AMD Quark API to begin `AWQConfig` tracing. A proprietary multimodal MoE violently conflicted with Quark's generically-tuned LLM execution assumptions. The following five architectural bugs reflect the absolute bleeding edge of NPU integration.

#### A. The Structural Builder Assertion (Type Checking)
**The Raw Traceback:**
```python
File "/opt/quark-venv/lib/python3.12/site-packages/quark/torch/quantization/tensor_quantize.py", line 156, in get_fake_quantize
    if quant_spec.dtype in USING_NON_SCALED_QUANT:
AttributeError: 'Int4PerGroupSpec' object has no attribute 'dtype'
```
**Engineering Reality:** Starting in v0.11, Quark strictly tightened matrix encapsulation. The old `Int4PerGroupSpec` element wrapper used dynamically in scripts was instantly rejected because the compiler demands an explicit, fully evaluated tensor blueprint possessing the `dtype` property.
**The Fix:** We forcefully called the internal class builder sequence immediately inside the `QConfig` assignment block:
```python
# The builder method .to_quantization_spec() natively unpacks the matrix type constraints.
quant_schema = QLayerConfig(weight=[Int4PerGroupSpec(ch_axis=1, group_size=128).to_quantization_spec()])
```

#### B. The Null Decoder Heuristic
**The Raw Traceback:**
```python
File "/opt/quark-venv/lib/python3.12/site-packages/quark/torch/algorithm/awq/awq.py"
    AttributeError: 'LongcatNextForCausalLM' object has no attribute ''
```
**Engineering Reality:** Quark natively uses hardcoded dictionaries to try and "guess" where your model stores its neural sequences (e.g. `LlamaDecoderLayer`, `GemmaLayer`). Because we are compiling a completely custom architecture, the internal pointer loop immediately defaulted to a blank string `""`, passing a completely null parameter space down the compilation loop and instantly destroying the hook.
**The Fix:** We hardcoded the pointer allocation into the algorithm dictionary logic:
```python
quant_config = QConfig(algo_config=[AWQConfig(model_decoder_layers="model.layers")])
```

#### C. The Multimodal "Dummy Tracer" Ghost Crash
**The Raw Traceback:**
```python
File "~/.cache/huggingface/modules/transformers_modules/LongCat_hyphen_Next/modeling_longcat_next.py", line 352, in forward
    if multimodal_generation_status.mode == "visual" and ...
AttributeError: 'NoneType' object has no attribute 'mode'
```
**Engineering Reality:** AWQ is "Activation-Aware." It calculates scaling error metrics by tracing mathematical dummy texts randomly fed down the tensor line to trace the exact numerical activations. Longcat-Next relies fiercely upon highly dynamic pipeline states (switching conditionally between checking `audio_head` and `visual_head` checkpoints utilizing the `multimodal_generation_status` flag) mid-sequence. Because Quark fires raw primitive tensors natively blindly into `Model.forward(...)`, the `multimodal_generation_status` context is never initialized, defaulting to `None`, instantly fracturing the entire conditional PyTorch logic tree natively.
**The Fix:** We permanently patched the model's physical GitHub cache file directly natively to forcefully instantiate a generic textual dictionary object wrapper. 
```python
if multimodal_generation_status is None:
    class DummyStatus:
        mode = "text"
        is_audio_start = False
    multimodal_generation_status = DummyStatus()
```

#### D. The R.O.P.E. Coordinate Geometry Mismatch
**The Raw Traceback:**
```python
  File "/opt/quark-venv/lib/python3.12/site-packages/transformers/models/longcat_flash/modeling_longcat_flash.py", line 325, in apply_rotary_pos_emb_interleave
    q_embed = (q * cos) + (rotate_half(q) * sin)
RuntimeError: The size of tensor a (8) must match the size of tensor b (94) at non-singleton dimension 2
```
**Engineering Reality:** This is highly fundamental. When simulating calibrations, the Hugging Face `tokenizer` yields entirely heterogeneous arrays corresponding strictly to text length (i.e. Sample A yields exactly 94 tokens; Sample B yields exactly 8 tokens). Native AWQ logic inherently traces mathematical constraints (namely the `position_embeddings` tuple parameter caching generated `cos`/`sin` coordinate waves) implicitly using the very setup defined by **the very first dataset trace batch**. 
Because it caches `kwargs`, when it dynamically fires the 8-token matrix through the layer using a 94-length `cos` geometry wrapper array, traversing across sequence dimension `2`, the array boundary dimensions violently overflow and crash. 
**The Fix:** Standardize strictly the fundamental dataset mapping length dynamically across native `<eos>` tensor padding formats. 
```python
if tokenizer.pad_token is None: tokenizer.pad_token = tokenizer.eos_token
inputs = tokenizer(example["text"], max_length=512, truncation=True, padding="max_length")
```

#### E. The AWQ Native Sub-Function `group_size` Wraparound
**The Raw Traceback (Two Separate Code-Block Triggers):**
```python
  File "/opt/quark-venv/lib/python3.12/site-packages/quark/torch/algorithm/awq/awq.py", line 363, in _compute_best_clip
    tmp_group_size = named_linears._weight_quantizer.group_size
AttributeError: 'SequentialQuantize' object has no attribute 'group_size'
----
  File "/opt/quark-venv/lib/python3.12/site-packages/quark/torch/algorithm/awq/awq.py", line 460, in pseudo_quantize_tensor
    group_size = linear_layer._weight_quantizer.group_size
AttributeError: 'SequentialQuantize' object has no attribute 'group_size'
```
**Engineering Reality:** Quark v0.11 implicitly encapsulated standard integer configurations internally directly inside a specialized `SequentialQuantize` container list element. However, the AMD core engineering team drastically failed to mathematically translate their specific variable lookups inside the numerical sub-functions handling matrix scale optimization (`_compute_best_clip` and `pseudo_quantize_tensor`). The native AMD logic brutally hardcoded queries identically for `._weight_quantizer.group_size`, utterly crashing because the class parameter relies on an iterative list location fundamentally located physically inside `._weight_quantizer[0]`.
**The Fix:** Because the error spans globally across the underlying external dependency package structure independently of our workflow codebase, we designed and aggressively deployed a pure RegEx wrapper intercept algorithm globally. This parsed the literal text layout dynamically of the internal AMD deployment modules installed within our localized node structure securely, directly rewriting the explicit source dependencies prior to compiler runtime logically.
```python
import os, re
file_path = '/opt/quark-venv/lib/python3.12/site-packages/quark/torch/algorithm/awq/awq.py'
with open(file_path, 'r') as f: text = f.read()

pattern = re.compile(r'([a-zA-Z0-9_\[\]]+)\._weight_quantizer\.group_size')
def replacer(match):
    var = match.group(1)
    return f"({var}._weight_quantizer[0].group_size if hasattr({var}._weight_quantizer, '__len__') else {var}._weight_quantizer.group_size)"

if text != pattern.sub(replacer, text):
    with open(file_path, 'w') as f: f.write(pattern.sub(replacer, text))
```

---

## 3. Evaluation: Longcat Nano and the ONNX Trajectory Analysis

Through the rigorous investigation logs recorded dynamically over the preceding execution sequences, we explicitly identified a total failure in utilizing `ONNX Runtime` to trace `Longcat-Next` parameters unconditionally. 
Because Longcat-Next utilizes dynamic generation conditional boundaries structurally, any generic attempt mathematically to dynamically export its geometry unconditionally deletes structural audio and spatial arrays structurally out of the computational graph during tracing iterations locally. Shifting development exclusively to `ONNX Runtime` would structurally force our OS integration to fundamentally manually segment the PyTorch base entirely dynamically over heavy C++ code arrays. PyTorch + AMD Quark AWQ strictly remains the exclusive path forward for processing unbridled conditional architecture natively.

---

## 4. Current Execution Status: UNRESOLVED HARDWARE FAILURES

Despite mapping 5 native architecture overrides, we have physically **not** gotten the AMD Quark architecture to work securely. The native API is overwhelmingly unstable and bleeds across basic Python reference points natively. 

### 6. The Recursive SequentialQuantizer Crash (`.observer`)
**The Raw Traceback (Directly Following the Group-Size Patch):**
```python
  File "/opt/quark-venv/lib/python3.12/site-packages/quark/torch/algorithm/awq/awq.py", line 472, in pseudo_quantize_tensor
    linear_layer._weight_quantizer.observer.reset_state()
AttributeError: 'SequentialQuantize' object has no attribute 'observer'
```
**Engineering Reality:** After successfully regex-patching the `.group_size` wrapper bug globally, the model securely initialized the layer scale loops successfully, only to immediately crash exactly 12 lines later when the AMD developers predictably hardcoded `._weight_quantizer.observer`. 
The AMD team fundamentally failed to update *any* internal numerical sub-parameters inside `awq.py` to match their own Version 0.11 `SequentialQuantize` list container structures. 

**Conclusion for Next Session:** The native ecosystem is fundamentally broken from the inside. Continuing this pure AWQ route requires a massive granular teardown and dynamic recompilation of `awq.py` to intercept and forcefully dynamically evaluate every single sequential property reference (`.group_size`, `.observer`, `.scale`, etc.) locally because the AMD framework mathematically cannot parse its own configurations correctly currently. 
*Explicit instruction: Extensive raw rebuilding of the PyTorch AMD dependency library arrays will be required next session.*
