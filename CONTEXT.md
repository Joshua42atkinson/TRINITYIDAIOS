# Trinity ID AI OS — Research Bible & Session Context
## March 27, 2026 — Production Prototype v17.0.0 (Native Rust Voice + ID·AI·OS Layout)

---

> **CRITICAL NOMENCLATURE ANCHOR & SYSTEM DIRECTIVE FOR ALL AI AGENTS:**
> The capitalization and spelling of the core pedagogical frameworks must NEVER drift. We use words as systems management:
> 1. **ADDIECRAPEYE**: *Always* fully capitalized, exactly as written. It is the 12-station instructional design lifecycle. Do not use variations.
> 2. **PEARL**: *Always* fully capitalized (Perspective, Engineering, Aesthetic, Research, Layout). It is the pedagogical focusing lens. Do not use variations.

## 1. WHAT TRINITY IS

Trinity ID AI OS is a gamified instructional design system that helps K-12 teachers build educational games. It uses AI agents orchestrated through ADDIECRAPEYE to autonomously create games, lesson plans, and educational media.

**TRINITY = ID + AI + OS:**
- **ID = ADDIE** — Instructional Design backbone
- **AI = CRAP** — Visual Design principles
- **OS = EYE** — Observer/Metacognition 

**THE TRINITY DELIVERABLES & WEB UI MAPPING (LEARNING + FUN + WORK):**
- **LEARNING (The IRON ROAD Tab)**: Structured application over rote memorization, built directly into the dev process and demonstrated in the portfolio. The page opens to `IRON ROAD`.
- **FUN (The ART Tab)**: The full edutainment "lite novel" **DAYDREAM** in the Bevy window. It represents the gamified adventure of the development process. The page opens to `ART Aesthetic Research Temp`.
- **WORK (The YARDMASTER Tab)**: The actual product you are on the Iron Road to build. The page opens to `Yard IDE`.

**THE THREE UX SYSTEMS:**
- **AUDIO** — Telephone line / Voice interfaces.
- **WEB** — React frontend (Iron Road / ART page / About).
- **BEVY** — **DAYDREAM** (The immersive game system inside the ART page where players watch and experience full lite-novel edutainment with music, pictures, and emotionally specific narration).

**The P-ART-Y Framework (Who operates Trinity):**
- **P = Pete** — The ONLY AI personality (Mistral Small 4 119B)
- **A = Aesthetics** — CRAP visual design, ComfyUI assets 
- **R = Research** — QM audits, tests, CI/CD
- **T = Tempo** — Code gen, Bevy scaffolding
- **Y = You** — The Yardmaster. Executive core.

---

## 2. THE "MEANING MAKING" TRACE (Isomorphic Alignment)

1. **AI Attention**: Sacred Circuitry (15 nodes)
2. **User Preference**: VAAM Bridge (Profile + Word Weights)
3. **Methodology**: ADDIECRAPEYE (12 stations) lifecycle
4. **Identity**: Character Sheet (Moved to "About" — directs the portfolio build, the project, and the actual user profile. Acts as the permanent "agreement between the computer and the user")
5. **Academic Progress**: LDT Portfolio (12 portfolio artifacts mapped to ADDIECRAPEYE phases, QM scoring)

**Functional Flow:**
User Message → VAAM Bridge → Pete Orchestration → Quest Objective Complete → Station Advance → **Portfolio Artifact Vaulted** → QM/Competency Recalculation → **Character Sheet Updated**.

---

## 3. RUNTIME ARCHITECTURE (Strix Halo 128GB)

- **vLLM (:8000) — PRIMARY** (~80GB): Mistral Small 4 119B safetensors + P-EAGLE. bitsandbytes INT4.
- **llama-server (:8080) — FALLBACK** (~80GB): MS4 Q4_K_M GGUF.
- **Voice (Embedded ORT)**: Supertonic-2 TTS (~280MB) & Whisper Base STT (~278MB).
- **Voxtral-4B (:8100)**: Fancy TTS (vLLM).
- **ComfyUI SDXL Turbo (:8188)**: Image generation.

---

## 4. CURRENT SYSTEM STATE (Purdue Presentation Ready)
- **Code Textbook Autopoiesis (Phase 8)**: LIVE. The core `crates/trinity/src/` engine is documented with Actionable P-ART-Y headers. `rag.rs` autonomously background-indexes these files into Qdrant on startup so Pete and the Recycler can read the engine's source semantics.
- **UI Deliverables Triad**: LIVE. The interface is mapped to Work (Yard IDE), Fun (ART/Daydream), and Learning (Iron Road).
- **0 Compile Errors**, **100% Tests Passing** (286 framework tests).
- **Portfolio & Character Sheet Unification**: LIVE. `PortfolioView` merges directly below the `CharacterSheet` within the 👤 Author tab natively without CSS flexbox clipping.
- **Agentic Parity (Pete)**: LIVE. The `task_queue` parameter parser safely accepts both String and Number JSON representations, formally unlocking Mistral Small 4's ability to act autonomously within the Yardmaster without entering an infinite loop.
- **Purdue Institutional Readiness (LDTAtkinson)**: LIVE. The landing page explicitly pitches the VAAM Engine Hook (Vulnerability, Autonomy, Authority, Mastery) as the pedagogical physics engine driving the student's intrinsic motivation.
- **Edge Guard Security**: LIVE. Safe tunnel access vs unrestricted local Dev access.
- **Sidecar Vaulting**: LIVE. ComfyUI creations map straight to the LDT Portfolio.

---

## 5. PENDING UI / EVIDENCE UPDATES (POST-vLLM MIGRATION)
*To be executed after vLLM system stability is achieved:*
1. **Remove Multilingual Pitch**: The system is not natively multilingual right now; remove the multilingual localization matrices from the Evidence page.
2. **"Super Textbook" Branding**: Shift the verbiage from "College in a Box" to "Super Textbook".
3. **Living Textbook Acronym**: Develop/integrate an acronym aligning with Trinity branding to describe a "living textbook that grows with the user."
4. **Lines of Code (LOC) Audit**: Recalculate the actual current Lines of Code for the core system vs the full project, and update the "194K" metric on the Evidence page and Professor Profile accordingly.
