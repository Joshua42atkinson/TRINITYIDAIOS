// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-iron-road/src/great_recycler.rs
// PURPOSE: Background task that synthesizes Journal entries into Book chapters
// ═══════════════════════════════════════════════════════════════════════════════
//
// The Great Recycler watches for quest events and synthesizes them into
// LitRPG prose chapters via the narrative engine. Raw journal entries (scrap)
// are recycled into polished narrative (the book).
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::RwLock;
use trinity_protocol::Genre;

use crate::book::{BookOfTheBible, Chapter};
use crate::narrative::{NarrativeContext, NarrativeEngine};

/// Event that triggers the Great Recycler to generate a chapter
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecyclerEvent {
    /// Quest that produced this event
    pub quest_id: String,
    /// ADDIECRAPEYE phase
    pub phase: String,
    /// Human-readable description of what happened
    pub description: String,
    pub alias: String,
    pub appearance: Option<String>,
    pub backstory: Option<String>,
    pub alignment: Option<String>,
    pub current_quest_flavor: Option<String>,
    /// Current game stats
    pub coal: f32,
    pub steam: u32,
    pub xp: u64,
    pub resonance_level: u32,
    /// Genre for narrative tone
    pub genre: Genre,
}

/// The Great Recycler — background process that turns quest events into book chapters
pub struct GreatRecycler {
    /// The Book of the Bible (shared, locked)
    book: Arc<RwLock<BookOfTheBible>>,
    /// Narrative engine for prose generation
    narrative: NarrativeEngine,
    /// Counter for generating chapter IDs
    chapter_counter: u32,
}

impl GreatRecycler {
    pub fn new(book: Arc<RwLock<BookOfTheBible>>, inference_url: &str) -> Self {
        Self {
            book,
            narrative: NarrativeEngine::new(inference_url),
            chapter_counter: 0,
        }
    }

    /// Process a single event and generate a chapter
    pub async fn recycle(&mut self, event: RecyclerEvent) -> anyhow::Result<()> {
        self.chapter_counter += 1;
        let chapter_id = format!("ch-{:03}", self.chapter_counter);

        let ctx = NarrativeContext {
            genre: event.genre,
            phase: event.phase.clone(),
            last_action: event.description.clone(),
            coal: event.coal,
            steam: event.steam,
            xp: event.xp,
            resonance_level: event.resonance_level,
            alias: event.alias.clone(),
            appearance: event.appearance.clone(),
            backstory: event.backstory.clone(),
            alignment: event.alignment.clone(),
            current_quest_flavor: event.current_quest_flavor.clone(),
        };

        // Generate prose via the narrative engine (calls LLM backend or uses fallback)
        let prose = self
            .narrative
            .generate_prose(&ctx, &event.description)
            .await?;

        // Build chapter title from the phase
        let title = format!("Station {}: {}", self.chapter_counter, event.phase);

        let chapter = Chapter {
            id: chapter_id,
            title,
            prose,
            quest_id: event.quest_id,
            timestamp: chrono::Utc::now(),
            resonance_level: event.resonance_level,
            phase: event.phase,
        };

        // Append to the book (persists to markdown + broadcasts SSE)
        let mut book = self.book.write().await;
        book.append_chapter(chapter).await?;

        Ok(())
    }

    /// Run as a background task, consuming events from a channel
    pub async fn run(mut self, mut event_rx: tokio::sync::mpsc::Receiver<RecyclerEvent>) {
        tracing::info!("[Great Recycler] Online. Listening for quest events...");

        while let Some(event) = event_rx.recv().await {
            tracing::info!(
                "[Great Recycler] Processing event: {} (quest: {}, phase: {})",
                event.description,
                event.quest_id,
                event.phase
            );

            if let Err(e) = self.recycle(event).await {
                tracing::error!("[Great Recycler] Failed to generate chapter: {}", e);
            }
        }

        tracing::info!("[Great Recycler] Channel closed. Shutting down.");
    }
}
