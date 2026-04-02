# Strix Halo Harmonization: Unifying ADDIECRAPEYE under vLLM

The transition from isolated `llama.cpp` child processes to a unified `vLLM` backbone fundamentally changes how the Trinity sidecars operate on the AMD Strix Halo architecture.

## The Bottleneck of Isolated Engines
Previously, the Conductor (Pete) and the Yardmaster (Dev) each spun up their own isolated `llama.cpp` inference engines.
1. **Memory Fragmentation:** 128GB of Unified Memory was being statically partitioned. The Yardmaster might reserve 40GB, Pete might reserve 30GB, leaving very little for Bevy or ComfyUI, even if one sidecar was idle.
2. **Sequential Blocking:** If Pete needed to evaluate a user's prompt before the Yardmaster could code, the Yardmaster's compute sat idle.

## The Solution: The Continuous Batching Quest System
By extracting the inference engine out of the individual sidecars and running a central `vLLM` server (with `vllm-omni` for Ming), we enable **Continuous Batching**.

### The ART Crate as the Batch Orchestrator
I have implemented `ArtQuestBatcher` in `crates/trinity-crap/src/quest_batcher.rs`. 
When the system needs to generate a massive amount of content (e.g., "Build 10 distinct neon signs for the Cyberpunk level"):

1. The ART sidecar builds a massive `Vec<VllmBatchRequest>`.
2. Half of the requests target the Conductor (`Mistral-Small-4`) to write the pedagogical text for the signs.
3. Half of the requests target the Yardmaster (`Ming-Flash-Omni-2.0`) to write the Rust/Bevy code and generate the structural visual mockups.
4. Using `tokio::join!`, it fires all requests simultaneously at the vLLM backbone.
5. vLLM's **PagedAttention** dynamically allocates memory across all 20 prompts, processing them in massive parallel batches utilizing the full compute width of the Strix Halo NPU/GPU, without hitting the OOM (Out Of Memory) limits that statically partitioned engines would hit.

## Dual Engine Rotation in DEV
The Yardmaster (`trinity-dev`) has also been updated to hold both engines conditionally:
* **The Flash Engine (Qwen-Coder via llama.cpp):** Remains active for instant, sub-second code generation when visual mockups are not needed.
* **The EYE Engine (Ming via vLLM-Omni):** Is toggled on when the user requests UI changes, 3D object placement, or visual evaluation, allowing Ming's DiT head to structurally validate the scene.

This allows the Yardmaster to drop compute to zero when idling, and instantly spool up the correct engine based on the task type.
