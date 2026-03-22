// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-iron-road/src/book.rs
// PURPOSE: Append-only chapter ledger for the Iron Road LitRPG narrative
// ═══════════════════════════════════════════════════════════════════════════════
//
// The Book of the Bible is the user's development story, written as a LitRPG
// lite-novel. Each chapter corresponds to a completed ADDIECRAPEYE cycle.
//
// ARCHITECTURE:
//   • Chapters stored as Markdown in docs/books_of_the_bible/[quest_id].md
//   • Append-only: chapters are never modified after writing
//   • SSE broadcast to /api/book/stream when new chapters arrive
//   • The Great Recycler (great_recycler.rs) synthesizes Journal entries
//     into prose chapters via the Conductor model
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::sync::broadcast;

/// A single chapter in the Book of the Bible
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Unique chapter ID (e.g. "ch-001")
    pub id: String,
    /// Chapter title (e.g. "The Awakening")
    pub title: String,
    /// The LitRPG prose content (Markdown)
    pub prose: String,
    /// Which quest produced this chapter
    pub quest_id: String,
    /// When the chapter was written
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Player's resonance level when this chapter was generated
    pub resonance_level: u32,
    /// ADDIECRAPEYE phase that completed to trigger this chapter
    pub phase: String,
}

/// The Book of the Bible — append-only ledger of the user's Iron Road journey
#[derive(Debug)]
pub struct BookOfTheBible {
    /// All chapters in order
    chapters: Vec<Chapter>,
    /// Directory to persist markdown files
    output_dir: PathBuf,
    /// Broadcast sender for SSE updates to connected clients
    update_tx: broadcast::Sender<String>,
}

impl BookOfTheBible {
    /// Create a new book with an output directory and SSE broadcast channel
    pub fn new(output_dir: PathBuf, update_tx: broadcast::Sender<String>) -> Self {
        // Ensure the output directory exists
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            tracing::warn!("Could not create book output dir {:?}: {}", output_dir, e);
        }

        Self {
            chapters: Vec::new(),
            output_dir,
            update_tx,
        }
    }

    /// Append a new chapter to the book
    /// This is the ONLY way to add content — chapters are never modified
    pub async fn append_chapter(&mut self, chapter: Chapter) -> anyhow::Result<()> {
        tracing::info!(
            "[Book] New chapter: {} — \"{}\" (quest: {}, resonance: {})",
            chapter.id,
            chapter.title,
            chapter.quest_id,
            chapter.resonance_level
        );

        // Persist to markdown file
        let md_path = self.persist_to_markdown(&chapter).await?;
        tracing::info!("[Book] Persisted to {:?}", md_path);

        // Broadcast SSE update
        let update_json = serde_json::to_string(&chapter).unwrap_or_default();
        let _ = self.update_tx.send(update_json);

        // Append to in-memory ledger
        self.chapters.push(chapter);

        Ok(())
    }

    /// Get a chapter by ID
    pub fn get_chapter(&self, id: &str) -> Option<&Chapter> {
        self.chapters.iter().find(|c| c.id == id)
    }

    /// Get the latest chapter
    pub fn latest_chapter(&self) -> Option<&Chapter> {
        self.chapters.last()
    }

    /// Get all chapters
    pub fn all_chapters(&self) -> &[Chapter] {
        &self.chapters
    }

    /// Total chapter count
    pub fn chapter_count(&self) -> usize {
        self.chapters.len()
    }

    /// Persist a chapter as a Markdown file
    async fn persist_to_markdown(&self, chapter: &Chapter) -> anyhow::Result<PathBuf> {
        let filename = format!("{}.md", chapter.id);
        let path = self.output_dir.join(&filename);

        let markdown = format!(
            "# {}\n\n\
             *Quest: {} | Resonance: {} | Phase: {} | {}*\n\n\
             ---\n\n\
             {}\n",
            chapter.title,
            chapter.quest_id,
            chapter.resonance_level,
            chapter.phase,
            chapter.timestamp.format("%Y-%m-%d %H:%M UTC"),
            chapter.prose,
        );

        tokio::fs::write(&path, markdown).await?;
        Ok(path)
    }

    /// Load existing chapters from the output directory (for restart recovery)
    pub async fn load_from_disk(
        output_dir: &Path,
        update_tx: broadcast::Sender<String>,
    ) -> anyhow::Result<Self> {
        let mut book = Self::new(output_dir.to_path_buf(), update_tx);

        if !output_dir.exists() {
            return Ok(book);
        }

        let mut entries: Vec<_> = std::fs::read_dir(output_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .collect();

        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let content = tokio::fs::read_to_string(entry.path()).await?;
            let filename = entry.file_name().to_string_lossy().replace(".md", "");

            // Parse minimal metadata from the markdown header
            let title = content
                .lines()
                .next()
                .unwrap_or("Untitled")
                .trim_start_matches("# ")
                .to_string();

            book.chapters.push(Chapter {
                id: filename,
                title,
                prose: content,
                quest_id: String::new(),
                timestamp: chrono::Utc::now(),
                resonance_level: 1,
                phase: String::new(),
            });
        }

        tracing::info!("[Book] Loaded {} chapters from disk", book.chapters.len());
        Ok(book)
    }
}
