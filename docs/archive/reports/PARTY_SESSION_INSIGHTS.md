# P-ART-Y Architecture Session Insights

**Date:** March 16, 2026  
**Purpose:** Lock in the True P-ART-Y Alignment and stop planning spirals

---

## The P-ART-Y Mnemonic (Locked)

The AI sidecar system now strictly aligns to 5 gears:

| Letter | Role | Model | Purpose |
|--------|------|-------|---------|
| **P** | **Pete** (Conductor) | Nemotron-120B Boss | Large hardware sidecar. Handles Yield and Execution. |
| **A** | **Aesthetics** (The Looks) | Mini-Trinity Swarm | ComfyUI, Image/3D/Video, automated visual workflows. Design & Extension. |
| **R** | **Research** (The Smarts) | Mini-Trinity Swarm | Testing sandbox for Yardman's code. Safe environment + Nvidia ACE. Analysis & Planning. |
| **T** | **Timing** (The Rhythm) | Mini-Trinity Swarm | Creative studio. PersonaPlex voice, brainstorming, music, vibe. Rhythm, sound, voice. |
| **Y** | **Yardman** (The Integrator) | Step-Fun Flash 121B | Massive hardware sidecar. Final code implementation at 40 t/s. Development, Implementation, Evaluation, Correction. |

---

## Mini-Trinity Swarm Composition

For the three swarm roles (A, R, T), the sidecar runs **3 models concurrently**:

1. **Blueprint Designer (Crow):** `Crow-9B-HERETIC` (Opus Distilled) - reasoning, planning, blueprint design
2. **Little Rust Builder:** `Qwen3-Coder-REAP-25B-A3B-Rust` - live coding, API integration
3. **Brakeman (OmniCoder):** `OmniCoder-9B` - localized quality gate, tests code within swarm

---

## Cruising Mode Batching

To solve the "2-minute sidecar swap" problem:

| ADDIECRAPEYE Phase | P-ART-Y Gear |
|-------------------|--------------|
| Analysis, Design, Development, Implementation, Evaluation, Correction | **YARDMAN** (locked for full D-I-E-C loop) |
| Yield | **PETE** (Conductor approval) |

The Conductor stays in Yardman gear for the entire Development→Implementation→Evaluation→Correction cycle to avoid VRAM thrashing.

---

## Immediate Focus: Pete & Yardman Only

**Locked In Scope:**
- Pete (Conductor) - Nemotron 120B
- Yardman (Integrator) - Step-Fun Flash 121B

**Deferred:**
- ART sidecars (Aesthetics, Researcher, Timing)
- Bevy Egui UI (marked as scope creep)
- Cruising Mode automation

---

## UI Focus

Primary observability is the **Headless Server Web UI** at `http://localhost:3000/dev.html`.

Bevy Egui (`trinity-body`) is classified as **Scope Creep / Future Tech**.

---

## Key File Locations

- **Role definitions:** `crates/trinity-sidecar-engineer/src/roles.rs`
- **Conductor orchestration:** `crates/trinity-sidecar-engineer/src/conductor_leader.rs`
- **Sidecar start API:** `crates/trinity-server/src/tools.rs`
- **Dev UI:** `crates/trinity-server/static/dev.html`
- **Architecture doc:** `TRINITY_TECHNICAL_BIBLE.md`

---

## Session Commit

This alignment is committed and locked. No further architecture changes until Pete and Yardman are fully operational within the Dev UI.
