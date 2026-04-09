# Open Notebook Sidecar — Document Ingestion Study
## Evaluating Patterns for the Yard's Document Management Feature

> **Status:** RESEARCH / FUTURE PILE  
> **Integration Point:** Yardmaster UI → `rag.rs` pipeline  
> **Depends On:** RAG infrastructure (SQLite + ONNX embeddings), Ring 3 (Rolling Context)

---

## 1. Current State

### What We Have

Trinity's existing document ingestion pipeline (`rag.rs`):

| Component | Implementation | Status |
|-----------|---------------|--------|
| **Chunking** | Paragraph-boundary splitting (~500 words/chunk) | ✅ Working |
| **Text Search** | SQLite `LIKE` fallback | ✅ Working |
| **Semantic Search** | ONNX cosine similarity (in-memory) | ✅ Working |
| **Embedding** | longcat-sglang `/v1/embeddings` → hash fallback | ✅ Working |
| **Auto-Ingest** | 7 key docs ingested at server startup | ✅ Working |
| **User Upload** | ❌ Not implemented | 🔴 Missing |
| **Document CRUD** | ❌ No delete/update UI | 🔴 Missing |
| **Format Support** | Markdown only | 🟡 Limited |

### What We Need

The "Open Notebook" concept: users should be able to **drag and drop** their existing teaching materials into the Yard, and Pete should be able to reference them during the Iron Road.

Target formats:
- **PDF** (syllabi, textbooks, handouts)
- **DOCX** (lesson plans, rubrics)
- **PPTX** (slide decks)
- **Images** (whiteboard photos, diagrams)
- **Videos** (lecture recordings — transcript extraction)

---

## 2. Ingestion Architecture Evaluation

### Pattern A: Direct Server Processing

```
User Upload → POST /api/yard/upload
                    │
                    ▼
            ┌──────────────┐
            │ Format Parser │   (pdf-extract, docx-rs, etc.)
            │     Rust      │
            └──────┬───────┘
                   │
                   ▼
            ┌──────────────┐
            │  RAG Pipeline │   (chunk → embed → store)
            │   rag.rs      │
            └──────────────┘
```

**Pros:** Simple, single-process, no new dependencies  
**Cons:** PDF parsing in Rust is immature. `lopdf` handles text but not tables/images. DOCX/PPTX support is limited.

### Pattern B: Python Sidecar (Recommended)

```
User Upload → POST /api/yard/upload
                    │
                    ▼
            ┌──────────────┐
            │  File Store   │   (save to workspace/yard/uploads/)
            └──────┬───────┘
                   │
                   ▼
            ┌──────────────────────┐
            │  Document Sidecar    │   Python process on port 8095
            │  ├── PyMuPDF (PDF)   │
            │  ├── python-docx     │
            │  ├── python-pptx     │
            │  ├── Whisper (video) │
            │  └── Tesseract (OCR) │
            └──────────┬───────────┘
                       │ POST /process
                       ▼
            ┌──────────────┐
            │  RAG Pipeline │   (chunk → embed → store)
            │   rag.rs      │
            └──────────────┘
```

**Pros:** Mature Python libraries for every format. Whisper for video transcription. Easy to extend.  
**Cons:** Adds a Python process. But we already have the sidecar pattern (ComfyUI, ACE-Step, Trellis).

### Pattern C: Browser-Side Processing

```
User Upload → Browser JavaScript
                    │
                    ▼
            ┌──────────────────────┐
            │  pdf.js (PDF→text)   │
            │  mammoth (DOCX→MD)   │
            └──────────┬───────────┘
                       │ POST /api/yard/ingest
                       ▼
            ┌──────────────┐
            │  RAG Pipeline │
            └──────────────┘
```

**Pros:** No server-side dependency. Works offline.  
**Cons:** Limited quality. No OCR. No video. Can't handle large files.

### Recommendation: **Pattern B** (Python Sidecar)

It fits the existing P-ART-Y architecture. The document sidecar would be a 6th party member:

| Role | Port | Purpose |
|------|------|---------|
| Engineer | 8090 | Code generation |
| Evaluator | 8090 | QM audits |
| Artist | 8090 | Creative assets |
| Brakeman | 8090 | QA/Security |
| Pete | 8090 | Socratic dialogue |
| **Librarian** | **8095** | **Document ingestion** |

---

## 3. Chunking Strategy Improvements

The current chunker (`chunk_text()` in `rag.rs`) splits on paragraph boundaries. For documents, we need smarter strategies:

### 3.1 Structural Chunking

```
PDF with Headings:
  Chapter 1: Introduction
    Section 1.1: Background
      [paragraph]
      [paragraph]
    Section 1.2: Objectives
      [paragraph]

Chunks:
  [Chapter 1 > Section 1.1 > paragraph 1]  — metadata: {heading: "Introduction > Background"}
  [Chapter 1 > Section 1.1 > paragraph 2]
  [Chapter 1 > Section 1.2 > paragraph 1]
```

### 3.2 Sliding Window with Overlap

```
Current: |chunk 1|chunk 2|chunk 3|  (no overlap — boundary artifacts)
Improved: |chunk 1  |
              |chunk 2  |
                  |chunk 3  |         (50-word overlap)
```

### 3.3 Table Extraction

Tables in PDFs/DOCX should be extracted as structured data, not flat text:

```json
{
  "type": "table",
  "headers": ["Term", "Definition", "Example"],
  "rows": [
    ["Photosynthesis", "Process by which plants...", "Leaf absorbs sunlight"]
  ]
}
```

---

## 4. Yard UI — Document Management

### 4.1 Upload Zone

```
┌─────────────────────────────────────────┐
│  📂 THE YARD — Document Management      │
│                                          │
│  ┌─────────────────────────────────┐    │
│  │  Drop files here or click to    │    │
│  │  upload your teaching materials  │    │
│  │                                  │    │
│  │  📄 PDF  📝 DOCX  📊 PPTX      │    │
│  │  🖼️ Images  🎥 Video           │    │
│  └─────────────────────────────────┘    │
│                                          │
│  ── DOCUMENT LIBRARY ──                  │
│  ┌────────────────────┬─────┬───────┐   │
│  │ Syllabus F24.pdf   │ 23  │ ✓ RAG │   │
│  │ Lab Safety.docx    │ 8   │ ✓ RAG │   │
│  │ Lecture 3.pptx     │ 45  │ ⟳ ...  │   │
│  │ Whiteboard.jpg     │ 1   │ ✓ OCR │   │
│  └────────────────────┴─────┴───────┘   │
│                                          │
│  Total: 77 chunks, 4 documents           │
└─────────────────────────────────────────┘
```

### 4.2 Pete Integration

When the user uploads documents, Pete can reference them:

```
PETE: "I see you uploaded your F24 Syllabus. Looking at Week 3, 
you cover 'Cellular Respiration' — that's a natural companion to 
your Photosynthesis lesson. Want me to build a bridge activity 
connecting the two?"
```

This is powered by the RAG search pipeline — Pete's system prompt includes relevant chunks from uploaded documents.

---

## 5. Implementation Estimate

| Component | Effort | Priority |
|-----------|--------|----------|
| Upload API endpoint (`POST /api/yard/upload`) | 2 hours | P1 |
| File storage (filesystem + DB metadata) | 1 hour | P1 |
| Python sidecar (PDF + DOCX parsing) | 4 hours | P1 |
| Improved chunking (structural + overlap) | 3 hours | P2 |
| Yard UI (upload zone + document list) | 3 hours | P2 |
| OCR for images (Tesseract) | 2 hours | P3 |
| Video transcription (Whisper) | 4 hours | P3 |
| PPTX parsing | 2 hours | P3 |

**Total P1 estimate: ~7 hours**

---

## 6. Security Considerations

- **File size limit:** 50MB per file, 500MB total per user
- **Format validation:** Check magic bytes, not just extension
- **Sandboxing:** Python sidecar runs in a container with no network access
- **Content scanning:** No executable content (macros in DOCX disabled)
- **Ring 5 integration:** Upload rate limiting (10 uploads/min)

---

## 7. References

- `crates/trinity/src/rag.rs` — Current RAG pipeline
- `crates/trinity/frontend/src/components/Yardmaster.jsx` — Existing Yard UI
- `crates/trinity-sidecar/src/api.rs` — Sidecar API pattern to follow
- PyMuPDF: https://pymupdf.readthedocs.io/
- python-docx: https://python-docx.readthedocs.io/

---

## 8. Competitive Analysis — NotebookLM & Antigravity IDE Patterns

### 8.1 What NotebookLM Does Right (and Where Trinity Wins)

NotebookLM (Dec 2025 / Mar 2026 updates) has established several best practices for document-backed AI:

| NotebookLM Pattern | How We Mirror It | Trinity's Edge |
|-------------------|-------------------|----------------|
| **Source, Study, Speak** framework — upload → analyze → generate | Yard upload → RAG embed → Pete references | We add ADDIECRAPEYE lens — sources are analyzed against pedagogical phases, not just content |
| **Source-labeled attribution** — every answer cites which doc it came from | RAG already returns `source_title` with each chunk | ✅ Already implemented in `rag.rs` |
| **Studio suite** — auto-generates flashcards, quizzes, study guides from docs | Our export system (Quiz, Adventure, GDD) already does this | We generate from the *quest progression*, not just raw docs — context-aware |
| **Google Classroom integration** (Dec 2025) — pull in assigned resources | Yard upload + filesystem watch | We're local-first — no cloud dependency. Privacy win. |
| **Mind Map generation** (Mar 2025) | Not yet — but Ring 6 Perspective Engine serves a similar role | Future: auto-generate concept maps from uploaded curriculum |
| **Deep Research** (Mar 2026) — multi-step investigation | Pete's Socratic protocol already does iterative questioning | We're *guided* research (ADDIECRAPEYE phases), not open-ended |
| **50 sources / 500K words per source** | No limit (local SQLite) | **Unlimited** — only bounded by disk space |
| **Continuous review loops** ("What did you miss?") | Ring 6 Perspective Engine (Devil's Advocate lens) | Automated, not manual |

### 8.2 What Antigravity IDE Does Right (Artifact System)

Antigravity's artifact generation model is directly relevant to how Trinity should handle document quality:

| Antigravity Pattern | Trinity Equivalent | Application |
|--------------------|--------------------|-------------|
| **Artifacts** — verifiable deliverables (plans, screenshots, recordings) | Quest objectives + GDD export | Each completed phase produces a tangible artifact |
| **Agent Manager View** — dashboard for supervising multiple agents | Our 3-column layout (Rail + Workspace + HUD) | Same architecture, education-focused |
| **Browser-based testing** — agent opens browser, verifies output | Golden Path E2E script | Headless verification of quest progression |
| **Implementation plans** as artifacts | PEARL Vision + quest objectives | Both create structured plans before execution |

### 8.3 Trinity's Killer Feature: The Quality Scorecard

**This is the big win.** Neither NotebookLM nor Antigravity quantifies the *pedagogical quality* of uploaded materials. They ingest and synthesize, but they don't *evaluate*.

Trinity should score every uploaded document on a **Quality Scorecard**:

```
┌─────────────────────────────────────────────────┐
│  📊 QUALITY SCORECARD — Syllabus F24.pdf         │
│                                                   │
│  Bloom's Coverage     ████████░░  82%            │
│  ADDIE Alignment      ██████░░░░  62%            │
│  Accessibility        ███████░░░  71%            │
│  Student Engagement   █████░░░░░  55%            │
│  Assessment Clarity   █████████░  90%            │
│                                                   │
│  Overall:  72/100  (B — "Solid foundation,        │
│           needs engagement hooks")                │
│                                                   │
│  ── RECOMMENDATIONS ──                            │
│  • Add a hook activity for Week 1 (Analysis gap)  │
│  • Week 5 jumps from Remember to Create (skip     │
│    Apply) — add a practice activity                │
│  • No rubric found for final project — generate?  │
└─────────────────────────────────────────────────┘
```

#### Scoring Dimensions

| Dimension | What It Measures | How We Score It |
|-----------|-----------------|-----------------|
| **Bloom's Coverage** | Are all 6 levels represented? Does the sequence build correctly? | Parse verbs from objectives → map to Bloom's → check coverage distribution |
| **ADDIE Alignment** | Does the material follow Analysis → Design → Development → Implementation → Evaluation? | Check for sections that map to each phase |
| **Accessibility** | Readability level, alt text presence, document structure | Flesch-Kincaid score, heading hierarchy, image count vs. alt text count |
| **Student Engagement** | Hooks, interactive elements, variety of activities | Scan for question marks, activity verbs ("create", "build", "explore"), media references |
| **Assessment Clarity** | Clear rubrics, measurable objectives, success criteria | Look for rubric tables, verbs matching Bloom's, point values |

#### Implementation

```rust
/// Quality scorecard for an uploaded document
pub struct QualityScorecard {
    pub document_id: i64,
    pub blooms_coverage: f32,      // 0.0-1.0
    pub addie_alignment: f32,
    pub accessibility: f32,
    pub engagement: f32,
    pub assessment_clarity: f32,
    pub overall: f32,
    pub recommendations: Vec<String>,
    pub scored_at: DateTime<Utc>,
}

/// Score is generated by:
/// 1. Document Sidecar extracts text + structure
/// 2. RAG pipeline chunks and embeds
/// 3. Scoring engine (LLM + heuristics) evaluates each dimension
/// 4. Results stored in SQLite, shown in Yard UI
```

This is what makes Trinity's Open Notebook **superior to NotebookLM for educators**: NotebookLM will summarize your syllabus. Trinity will tell you it's missing a practice activity in Week 5 and that your Bloom's progression skips Apply.

### 8.4 Competitive Positioning

```
NotebookLM:      Upload → Summarize → Answer questions about it
Antigravity:     Upload → Agent reads it → Agent builds with it
Trinity:         Upload → Score it → Tell you what's missing →
                 Help you fix it with ADDIECRAPEYE → Export the improved version
```

The quality scorecard transforms Trinity from "another document AI" into a **pedagogical quality assurance tool**. That's the differentiator for the presentation.
