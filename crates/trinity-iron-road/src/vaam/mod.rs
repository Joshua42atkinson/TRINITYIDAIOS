// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Iron Road / VAAM Subsystem
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         vaam/mod.rs
// BIBLE CAR:    Car 4 — IMPLEMENT (Iron Road Game Mechanics)
// HOOK SCHOOL:  🏫 Pedagogy — VAAM Engine
// PURPOSE:      Module entry point for the Vocabulary Acquisition Autonomy Mastery
//               (VAAM) subsystem. Re-exports cognitive_load (Flesch-Kincaid
//               scoring), litrpg (handbook generation), and madlibs (lesson
//               template filling). VAAM is the pedagogical core of the Iron
//               Road — every word the user encounters is tracked, scored, and
//               integrated into the learning journey.
//
// ═══════════════════════════════════════════════════════════════════════════════

pub mod cognitive_load;
pub mod litrpg;
pub mod madlibs;

pub use cognitive_load::*;
pub use litrpg::*;
pub use madlibs::*;
