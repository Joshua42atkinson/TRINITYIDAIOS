# The Iron Road API & Workflow Map
## Trinity ID AI OS - System Responsibility Matrix

As we finalize the systems and transition towards testing, this document maps out the clear lines of workflow, API responsibilities, and data flows within the **Iron Road**. This ensures every module has a bounded context and we can accurately measure, evaluate, and test the results.

---

## 🚂 1. The Core Loop: The Iron Road

The **Iron Road** is the central nervous system of Trinity. It connects the user's cognitive load (extrapolated from hardware telemetry) to the narrative LitRPG elements and the actual ADDIE instructional design progress.

### 1.1 Actor Responsibilities
| Actor / Module | Primary Responsibility | API / Comm Protocol | Bound Context |
|----------------|------------------------|---------------------|---------------|
| **ConductorLeader** | Orchestrates the party. Assigns quests based on ADDIE phase. | Internal Rust Channels / `tarpc` | Cannot generate code or art directly. Only plans and routes. |
| **Ask Pete (NPU)** | Socratic Voice Companion. Front-line SME interview. | `ort` (ONNX) Local + Axum API | Runs purely on the NPU (Lobby). Always listening. |
| **Great Recycler** | Observes progress. Translates raw actions into the "Iron Road Book" narrative. | Axum REST (`/api/narrative`) | Reads system events, outputs Markdown/HTML to Layer 2. |
| **Engineer (GPU)** | Writes Rust/Bevy code. Executes development quests. | `llama-server` (Vulkan) port 8081 | Loaded on demand. Heavy lifting logic. |
| **Artist (GPU)** | Generates sprites and visuals using SDXL/ComfyUI. | ComfyUI REST API | Loaded on demand. Output to `/assets/`. |

---

## 🛤️ 2. API Data Flow by System

### 2.1 The Weigh Station (VaaM - Vocabulary Acquisition and Mastery)
* **Goal**: Determine the *Intrinsic Load* (Mass) of a concept being taught.
* **Flow**:
  1. User introduces a concept/word.
  2. `WeighRequest` is dispatched in Bevy (`trinity-body/src/game/weigh_station.rs`).
  3. HTTP POST to local `llama-server` (port 8080/8081).
  4. Engine parses JSON schema -> Returns `WordPhysics` (Tier, Mass, Tags).
  5. Concept is physically spawned in the 3D Bevy space with its assigned mass.

### 2.2 Telemetry to Friction
* **Goal**: Enforce the Doctrine of Isomorphism (Hardware Load = Cognitive Load).
* **Flow**:
  1. `system_reaper` polls RAM, VRAM, and CPU usage.
  2. Stats are published to `MagicalSystemState`.
  3. "Track Friction" increases as memory fills (approaching the 128GB Unified Memory limit).
  4. UI dynamically updates; narrative slows down. If memory maxes out, Conductor forces a "Rest" (Model Offload).

### 2.3 The 12 Stations (ADDIECRAPEYE) State Machine
* **Goal**: Track instructional design progress.
* **Flow**:
  1. `AddieWorkflowState` tracks current phase (e.g., `Analysis` -> `Design`).
  2. Actions in UI trigger `RequestAiAnalysisEvent`.
  3. Event handled by `ConductorLeader`, delegated to specific `Sidecar`.
  4. Artifacts generated (code, text, art).
  5. Great Recycler verifies QM (Quality Matters) rubric. Phase advances.

---

## 🧪 3. Testing and Measurement Strategy

To ensure we are not creating redundant features and can measure actual results:

1. **VaaM Measurement**: We can measure the success of the Weigh Station by logging the `tokens/sec` during the `WordPhysics` JSON generation and validating the strict adherence to the output schema.
2. **Hardware Friction Measurement**: We will monitor the Vulkan flags (`-fa`, `-ctk q4_0`, `--mmap 1`) during a 65K context window conversation to measure if the UMA trap is avoided and memory usage remains stable without OOM panics.
3. **Audio NPU Pipeline Test**: We can measure latency from `NpuAudioEngine::speech_to_text` using the newly integrated `ort` Vitis AI execution provider vs the legacy dummy mocks.
4. **Quest Chaining Evaluation**: Testing will simulate a user going from Station 1 (Awakening/Analysis) to Station 5 (Evaluation) to verify that `ConductorLeader` successfully hot-swaps the `Engineer` and `Evaluator` without dropping the `SharedContext`.

---

## 🧹 4. Final Cleanup Status
* **NPU Audio**: `mock_speech_to_text` and encoding functions replaced with actual `ort::Session` inferences over Vitis AI.
* **NPU Engine**: Removed dummy float payloads; dynamically sizes based on model architecture.
* **Weigh Station**: Fallback mocks deleted. Strict HTTP local inference enforced to prevent silent offline failures.
* **Diffusion / Graphics**: Legacy `diffusion_asset.rs` (C++ mock bindings) removed entirely in favor of the active `comfyui` and `llama-server` mmproj integrations.
