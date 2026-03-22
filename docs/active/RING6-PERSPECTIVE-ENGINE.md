# Ring 6 — The Perspective Engine
## Multi-Perspective AI Output Evaluation

> **Ring Status:** DESIGN  
> **Author:** Trinity AI OS  
> **Depends On:** Ring 2 (Persona Clearance), Ring 3 (Rolling Context), Ring 5 (Rate Limiting)  
> **Integration Point:** `crates/trinity/src/agent.rs` → `evaluate_perspectives()`

---

## 1. Problem Statement

Currently, Pete generates a **single response** to each user message. The user either accepts it or redirects with follow-up questions. This creates two failure modes:

1. **Confirmation Bias** — Pete's response anchors the user's thinking. A first-year teacher asking about lesson structure gets *one* view. There's no cognitive friction.
2. **Missing Expertise** — Pete is a generalist. For a physics question, an actual physics teacher would frame the answer differently than an instructional designer.

The **Perspective Engine** addresses this by generating **multiple evaluative lenses** on Pete's output before the user sees it, then surfacing the most valuable contrast.

---

## 2. Architecture

```
User Message
     │
     ▼
┌─────────────┐
│  Pete (P-1)  │  ← Primary response (Slot 1, Ring 2 persona)
│  "The answer" │
└─────┬───────┘
      │
      ▼
┌─────────────────────────────────────────────────┐
│          PERSPECTIVE ENGINE (Ring 6)              │
│                                                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │ Lens A   │  │ Lens B   │  │ Lens C   │       │
│  │ "Bloom's │  │ "Pract-  │  │ "Devil's │       │
│  │  Check"  │  │  itioner"│  │ Advocate" │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
│       │              │              │              │
│       ▼              ▼              ▼              │
│  ┌──────────────────────────────────────────┐    │
│  │       Perspective Synthesis               │    │
│  │  "Pete's answer is solid, but consider:" │    │
│  └──────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
      │
      ▼
  User sees:  Pete's response + Perspective sidebar
```

### 2.1 The Three Default Lenses

| Lens | Name | Evaluates | Example Output |
|------|------|-----------|----------------|
| **A** | *Bloom's Check* | Does Pete's response match the current phase's Bloom's verb? If we're in "Analysis" (Remember), Pete shouldn't be asking users to "Create." | "⚠ Pete is asking you to design (Apply) but you're still at the Analysis station (Remember). Consider first listing what you already know." |
| **B** | *Practitioner* | Would an experienced teacher in this subject agree? Cross-references the user's stated experience level from Session Zero. | "A 10-year K-12 veteran might already know this. Consider shortcuts: 'What assessment techniques have you already tried?'" |
| **C** | *Devil's Advocate* | What's the strongest counterargument? What assumption is Pete making that could be wrong? | "Pete assumes your students struggle with engagement. But the real bottleneck might be prerequisite knowledge, not motivation." |

### 2.2 Dynamic Lens Selection

Not every message needs all three lenses. The Perspective Engine uses a **relevance heuristic**:

```rust
/// Decide which lenses to activate based on context
fn select_lenses(phase: &str, message_type: MessageType, session_zero: &CharacterSheet) -> Vec<Lens> {
    let mut lenses = vec![Lens::BloomsCheck]; // Always active — it's the pedagogical spine
    
    // Add Practitioner lens when the user's experience level is known
    if session_zero.experience.is_some() {
        lenses.push(Lens::Practitioner);
    }
    
    // Add Devil's Advocate only on substantive responses (not greetings/transitions)
    if message_type == MessageType::Substantive && phase != "Analysis" {
        lenses.push(Lens::DevilsAdvocate);
    }
    
    lenses
}
```

### 2.3 Evaluation Cost

Each lens requires an LLM call. To avoid 4x latency:

| Strategy | Description |
|----------|-------------|
| **Parallel Execution** | All lenses fire concurrently using `tokio::join!` |
| **Short Context** | Each lens gets only Pete's response + the current phase + a 100-word system prompt. No full history. |
| **Token Budget** | Each lens is capped at 100 tokens output (2-3 sentences max) |
| **Lazy Evaluation** | On first message of a phase, skip perspectives. Only activate after the 2nd+ exchange. |

**Estimated overhead:** 0.5-1.5 seconds (parallel), ~300 tokens total input per perspective set.

---

## 3. User Interface

### 3.1 Perspective Sidebar

The perspectives appear as **marginalia** in the book view — like handwritten notes in the margins of a manuscript.

```
┌──────────────────────────────────────────────┐
│ Pete's Response                               │
│                                               │     ┌─────────────────┐
│ "Start by identifying your learning           │     │ 🔍 BLOOM'S      │
│  objectives. What should students be able     │     │ ✓ Matches phase  │
│  to DO after this lesson? Use a verb from     │     │   (Remember →    │
│  Bloom's taxonomy..."                         │     │    identify)     │
│                                               │     │                 │
│                                               │     │ 👤 PRACTITIONER │
│                                               │     │ For K-12: add   │
│                                               │     │ "age-appropriate │
│                                               │     │  verb examples"  │
│                                               │     │                 │
│                                               │     │ 😈 ADVOCATE     │
│                                               │     │ Assumption: user │
│                                               │     │ knows Bloom's.   │
│                                               │     │ Consider explain-│
│                                               │     │ ing the taxonomy │
│                                               │     │ first.           │
│                                               │     └─────────────────┘
└──────────────────────────────────────────────┘
```

### 3.2 Collapsible

- Default: **collapsed** to a single "🔮 3 perspectives" badge
- Click to expand the marginalia
- Each perspective has a 👍/👎 reaction → trains the lens relevance over time

---

## 4. Implementation Plan

### Phase 1 — Core Engine (Rust)

```
crates/trinity/src/perspective.rs  (NEW)
├── struct Lens { name, system_prompt, max_tokens }
├── struct Perspective { lens, content, relevance_score }
├── async fn evaluate(pete_response, phase, lenses) -> Vec<Perspective>
└── fn select_lenses(phase, msg_type, character) -> Vec<Lens>
```

Wire into `agent.rs`:
```rust
// After Pete responds:
let perspectives = perspective::evaluate(&pete_response, &current_phase, &lenses).await;
// Attach to SSE response as a sidecar field
```

### Phase 2 — Frontend

```
components/PerspectiveSidebar.jsx  (NEW)
├── Collapsed badge: "🔮 3 perspectives"
├── Expanded: marginalia-style cards
└── Reaction buttons (👍/👎) → POST /api/perspective/feedback
```

### Phase 3 — Learning

- Track which perspectives users find useful (👍/👎 ratio per lens)
- Adjust lens selection weights over time
- Add custom lenses defined by the user: "Always check for accessibility"

---

## 5. Ring Integration

| Ring | Interaction |
|------|------------|
| **Ring 2** (Persona) | Perspectives respect persona clearance. If Pete is in "Recycler" slot, the Practitioner lens adapts. |
| **Ring 3** (Context) | The Rolling Context Digest is NOT passed to lenses — they only see Pete's response. This keeps them fast and unbiased. |
| **Ring 5** (Rate Limit) | Perspective calls count toward the 60/min rate limit. Each set of lenses = 1-3 calls. |
| **Ring 6** (This) | Self-contained. Does not modify Pete's output — only annotates it. |

---

## 6. Success Criteria

1. **Bloom's alignment accuracy** ≥ 85% (validates phase-verb match correctly)
2. **User engagement** — perspectives are expanded on ≥ 30% of messages
3. **Latency** — perspective generation adds < 2 seconds to response time
4. **Zero prompt injection** — lenses cannot modify Pete's actual response

---

## 7. Future Extensions

- **Expert Lenses** — GPT-4 or Opus for domain-specific evaluation ("What would a physicist say?")
- **Student Voice Lens** — "How would a 7th grader interpret this instruction?"
- **Accessibility Lens** — "Does this work for students with dyslexia / color blindness?"
- **Cultural Lens** — "Are there cultural assumptions in this framing?"
