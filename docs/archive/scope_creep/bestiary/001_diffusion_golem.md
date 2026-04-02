# The Diffusion Golem
*A manifestation of an in-engine ComfyUI Bevy 3D Studio*

**HP (Tech Debt):** 80 (Heavy C++ wrapper dependencies, requires custom `diffusion.cpp` bindings that diverge from the main Iron Road.)
**Mana Cost:** 16GB VRAM (Minimum for Stable Diffusion 1.5), 24GB for SDXL.
**Taming Requirement:** **Design & Integration (Development Phase)**
**Git Artifacts:** `docs/scope_creep/dumpster_universe/diffusion_asset.rs.bak`

### Lore (The Origin Story)
During the early chaotic days of Trinity's genesis, the ambition was to have the game engine itself (Bevy) directly call a local C++ library (`diffusion.cpp`) to generate 3D textures, UI elements, and sprites on the fly without any external server. 

The daydream was a fully self-contained "Studio in a Box" where the Bevy ECS spawned generation tasks. However, the reality of maintaining C++ build bindings in a pure Rust workspace, combined with the fact that we already have a dedicated `ComfyUI` sidecar accessible via REST, turned this feature into a massive, tangled Scope Creep. It was banished to the Dumpster Universe to keep the core Iron Road API clean.

### Integration Strategy (The Taming Plan)
To safely merge the Diffusion Golem back into the Iron Road:
1. **Analysis:** Verify if `diffusion.cpp` offers a clear performance advantage over local `ComfyUI` REST calls for *our specific asset generation needs*.
2. **Design:** Create an asynchronous worker pool in Bevy that does *not* block the main render thread when the C++ FFI is invoked.
3. **Development:** Rewrite the mock `diffusion_asset.rs` to actually link against the `diffusion.cpp` static library using `bindgen`.
4. **Evaluation:** Ensure the memory footprint of the loaded diffusion model does not clash with the `llama-server` KV cache (The UMA Trap).
5. **Yield:** If it passes the evaluation, it becomes an equippable "Artifact Generator" for the Artist sidecar.
