# Trinity ID AI OS: Scope Creep & Three-on-a-Tree Analysis

## The "Three on a Tree" Philosophy
Trinity operates like an old-school column shifter ("three on the tree"). You only use one gear at a time:
1. **First Gear (IRON ROAD / Pete):** The ID. Handles lesson planning, narrative state, VAAM, and learning objectives.
2. **Second Gear (DEV / Yardmaster / Ming):** The Architect. Writes Bevy code and streams structural mockups.
3. **Third Gear (ART / ComfyUI / Blender):** The Studio. Renders the final high-fidelity 2D/3D assets.

You can shift between them quickly, and they can pass data to each other, but the user is only "driving" in one context at a time to maintain focus and safety.

## Evaluating New Tech (Avoiding Scope Creep)

### 1. "OpenSandbox" by Alibaba
* **What it is:** Alibaba's Qwen-Agent framework includes a Python code execution sandbox, allowing the AI to safely run and test Python code in isolated environments.
* **Does Trinity need it?** **No.**
* **Why:** The Yardmaster sidecar is building *Rust* code for the *Bevy* engine. Python execution sandboxes are great for data analysis (like Jupyter), but they do not help us compile or run Bevy. If we need a sandbox, we should use a Rust-native Wasm sandbox (which `trinity-inference` already has a prototype for). Adding a Python sandbox creates unnecessary overhead.

### 2. GLM OCR (Zhipu AI) for `book.rs` / Academic Papers
* **What it is:** GLM-4V is highly optimized for complex document OCR (reading academic papers, extracting math formulas, parsing PDFs).
* **Does Trinity need it?** **Maybe later, but NOT right now.**
* **Why:** You mentioned your goal is to solve problems and not create scope creep. Right now, `book.rs` and the Iron Road narrative are functioning on standard text and user prompts. Integrating a massive VLM just for OCR requires standing up another API endpoint or memory allocation. Let Mistral/Pete handle text ingestion for now. If reading actual PDFs of academic papers becomes a strict bottleneck for the user, we can add it, but it violates the "UI first" priority right now.

### 3. Open Viking (Tiered Context Loading)
* **What it is:** A system for managing massive context windows by tiering vector database retrieval (summary vs full text).
* **Does Trinity need it?** **No, we already have something better.**
* **Why:** We are already implementing **Graph RAG via SurrealDB** combined with **vLLM's PagedAttention**. SurrealDB handles the exact hierarchical relationships of the Bevy ECS, and vLLM handles the memory paging perfectly. Adding another tiering system is redundant and complex.

## The Verdict: Stop Adding, Start Driving
Your instinct is exactly right: **"I have not used trinity yet, I am waiting on the UI to be usable."**

We have built an incredible backend:
* Mistral is ready to teach (Pete).
* Ming is ready to code and mockup (Yardmaster).
* The ART batcher is ready to build textures (ComfyUI).

**Recommendation:** Do not add GLM OCR, Alibaba Sandboxes, or Open Viking right now. We must adhere to the "Three on a Tree" philosophy. We need to build the steering wheel (the UI) so you can actually drive the car. 
