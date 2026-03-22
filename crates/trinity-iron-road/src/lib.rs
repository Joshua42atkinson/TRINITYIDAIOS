// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-iron-road
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        lib.rs
// PURPOSE:     Iron Road narrative system, book writing, Great Recycler
//
// ARCHITECTURE:
//   • ADDIE Sidecar — Book of the Bible generation via NPU Great Recycler
//   • LitRPG prose chapter writing from quest updates
//   • Append-only ledger of user's Hero's Journey
//   • Markdown persistence: docs/books_of_the_bible/*.md
//   • Real-time SSE broadcast to /book.html
//
// DEPENDENCIES:
//   - trinity-protocol — HeroStage, QuestState
//   - trinity-quest — Player progress
//   - pulldown-cmark — Markdown processing
//   - tokio — Async runtime
//
// CHANGES:
//   2026-03-16  Cascade  Created as dedicated Iron Road crate as part of 12-crate restructure
//
// ═══════════════════════════════════════════════════════════════════════════════

pub mod book;
pub mod great_recycler;
pub mod narrative;

pub use book::*;
pub use great_recycler::*;
pub use narrative::*;
pub mod game_loop;
pub mod vaam;
pub use game_loop::*;
pub use vaam::*;
pub mod pete_core;
