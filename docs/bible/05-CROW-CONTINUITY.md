# Trinity Technical Bible — CROW Continuity Architecture
**Version:** 1.0.0
**Date:** March 17, 2026
**Status:** Architecture Reference

---

## 1. Overview: The CROW Trinity

CROW (Crow-9B-Opus-4.6-Distill) serves as the **unified brain** of Trinity, connecting three critical functions:

```
┌─────────────────────────────────────────────────────────────────┐
│                         CROW (9B)                               │
│                    "The All-Seeing Bird"                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   👁️ VISION OBSERVER      📖 NARRATIVE ORCHESTRATOR            │
│   ┌──────────────┐        ┌──────────────┐                     │
│   │ Screenshots  │        │ Story-driven │                     │
│   │ UI State     │───────►│ Sidecar      │                     │
│   │ User Actions │        │ Coordination │                     │
│   └──────────────┘        └──────────────┘                     │
│          │                        │                             │
│          └────────────┬───────────┘                             │
│                       ▼                                         │
│              🔗 CONTINUITY ENGINE                                │
│              ┌──────────────────┐                               │
│              │ Iron Road Ledger │                               │
│              │ Event Sourcing   │                               │
│              │ Real-time Sync   │                               │
│              └──────────────────┘                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. CROW Model Specifications

| Property | Value | Notes |
|----------|-------|-------|
| **Model ID** | `Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5` | Distilled reasoning |
| **Parameters** | 9B | Efficient for continuous operation |
| **Quantization** | Q4_K_M | ~6GB memory footprint |
| **Context** | 32K tokens | Sufficient for narrative context |
| **Vision** | ✅ Via mmproj | Can analyze screenshots |
| **Role** | Observer + Orchestrator | Unique dual-role |

### 2.1 Why CROW?

- **9B size**: Small enough to run continuously without Hotel swapping
- **Vision capability**: Can "see" UI state via screenshots
- **Opus distillation**: High reasoning quality in compact form
- **Narrative focus**: Trained for story coherence

---

## 3. Iron Road: Event-Sourced Ledger

### 3.1 Architecture

The Iron Road is an **append-only event log** stored in PostgreSQL:

```sql
-- Iron Road Event Store
CREATE TABLE iron_road_events (
    id              BIGSERIAL PRIMARY KEY,
    sequence_num    BIGINT NOT NULL UNIQUE,  -- Monotonic ordering
    timestamp       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_type      TEXT NOT NULL,           -- quest_start, quest_complete, etc.
    aggregate_id    UUID NOT NULL,           -- Quest, Session, or User ID
    aggregate_type  TEXT NOT NULL,           -- quest, session, user
    payload         JSONB NOT NULL,          -- Event data
    metadata        JSONB DEFAULT '{}',      -- Source, version, etc.
    character_sheet_snapshot JSONB,          -- VIBE state at this moment
);

-- Index for replay
CREATE INDEX idx_iron_road_sequence ON iron_road_events(sequence_num);
CREATE INDEX idx_iron_road_aggregate ON iron_road_events(aggregate_id, aggregate_type);
```

### 3.2 Event Types

| Event Type | Trigger | Payload |
|------------|---------|---------|
| `session_start` | User connects | `{ user_id, ui_context }` |
| `quest_start` | Quest accepted | `{ quest_id, objectives }` |
| `quest_progress` | Objective completed | `{ quest_id, objective_id }` |
| `quest_complete` | All objectives done | `{ quest_id, xp_earned }` |
| `sidecar_spawn` | Model loaded | `{ role, model, port }` |
| `sidecar_swap` | Hotel swap | `{ out_role, in_role }` |
| `narrative_beat` | CROW observation | `{ scene, emotion, context }` |
| `character_level` | Resonance up | `{ old_level, new_level }` |
| `creative_asset` | Image/audio generated | `{ asset_id, type, prompt }` |

### 3.3 Event Sourcing Principles

1. **Append-Only**: Events are never modified or deleted
2. **Monotonic Sequence**: Each event gets incrementing sequence number
3. **Full Replay**: Current state = replay all events from sequence 0
4. **Snapshots**: Periodic CharacterSheet snapshots for fast recovery

```rust
// Event replay to reconstruct state
pub async fn replay_to_state(pool: &PgPool, up_to: i64) -> Result<IronRoadState> {
    let events = sqlx::query_as!(
        IronRoadEvent,
        "SELECT * FROM iron_road_events WHERE sequence_num <= $1 ORDER BY sequence_num",
        up_to
    ).fetch_all(pool).await?;
    
    let mut state = IronRoadState::default();
    for event in events {
        state.apply(event)?;
    }
    Ok(state)
}
```

---

## 4. CROW as Vision Observer

### 4.1 Screenshot Analysis Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ UI Layer    │───►│ Screenshot  │───►│ CROW Vision │
│ (Browser/   │    │ Capture     │    │ Analysis    │
│  Bevy)      │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
                                              │
                                              ▼
                                       ┌─────────────┐
                                       │ Narrative   │
                                       │ Observation │
                                       └─────────────┘
```

### 4.2 Observation Triggers

| Trigger | Frequency | Purpose |
|---------|-----------|---------|
| **Periodic** | Every 30s | Ambient narrative generation |
| **State Change** | On event | Quest progress, sidecar swap |
| **User Request** | On demand | "What do you see?" |
| **Error/Crash** | Immediate | Failure narrative integration |

### 4.3 Vision Output Format

```rust
pub struct CrowObservation {
    /// What CROW sees in the UI
    pub visual_description: String,
    /// Emotional tone detected
    pub emotional_context: EmotionTone,
    /// Relevant to active quest?
    pub quest_relevance: Option<Uuid>,
    /// Suggested narrative beat
    pub narrative_suggestion: String,
    /// Timestamp for Iron Road
    pub timestamp: DateTime<Utc>,
}
```

---

## 5. CROW as Narrative Orchestrator

### 5.1 Story-Driven Sidecar Coordination

CROW coordinates sidecars through **narrative prompts**, not direct commands:

```
┌─────────────────────────────────────────────────────────────────┐
│                    NARRATIVE ORCHESTRATION                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   CROW generates story context:                                 │
│   "The Engineer steps forward, tools ready to forge..."         │
│                                                                 │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Conductor receives                      │   │
│   │   "The Engineer steps forward..."                        │   │
│   │        │                                                 │   │
│   │        ▼                                                 │   │
│   │   ┌─────────────┐                                       │   │
│   │   │ Hotel Swap  │ Load Engineer sidecar                 │   │
│   │   └─────────────┘                                       │   │
│   │        │                                                 │   │
│   │        ▼                                                 │   │
│   │   ┌─────────────┐                                       │   │
│   │   │ Engineer    │ Receives narrative context            │   │
│   │   │ (REAP-25B)  │ "You are the Engineer in this scene" │   │
│   │   └─────────────┘                                       │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Narrative Context Injection

Every sidecar prompt includes Iron Road context:

```rust
pub fn build_sidecar_prompt(
    role: PartyRole,
    iron_road_context: &IronRoadContext,
    character_sheet: &CharacterSheet,
) -> String {
    format!(
        r#"You are {} in the Iron Road narrative.

CURRENT SCENE:
{}

YOUR CHARACTER:
- Name: {}
- Role: {}
- Resonance Level: {}

RECENT EVENTS:
{}

Your task: {}"#,
        role.display_name(),
        iron_road_context.current_scene,
        character_sheet.alias,
        character_sheet.user_class.display_name(),
        character_sheet.resonance_level,
        iron_road_context.recent_events.join("\n"),
        role.task_description()
    )
}
```

---

## 6. CROW as Continuity Engine

### 6.1 Real-Time Sync Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    CONTINUITY ENGINE                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────┐                        ┌─────────────┐        │
│   │ Character   │◄─────── Sync ─────────►│ Iron Road   │        │
│   │ Sheet       │                        │ Ledger      │        │
│   │ (VIBE)      │                        │ (Events)    │        │
│   └─────────────┘                        └─────────────┘        │
│         │                                       │               │
│         │                                       │               │
│         ▼                                       ▼               │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    CROW MONITOR                         │   │
│   │  • Watches CharacterSheet changes                       │   │
│   │  • Subscribes to Iron Road events                       │   │
│   │  • Generates narrative beats on state changes           │   │
│   │  • Broadcasts to all UIs via SSE                        │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│              ┌───────────────┼───────────────┐                  │
│              ▼               ▼               ▼                  │
│        ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│        │ Browser  │    │ Browser  │    │  Bevy    │             │
│        │ /book    │    │ /chat    │    │  UI      │             │
│        └──────────┘    └──────────┘    └──────────┘             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Sync Protocol

```rust
pub struct ContinuitySync {
    /// Iron Road event sequence at last sync
    pub last_sequence: i64,
    /// CharacterSheet hash at last sync
    pub character_hash: u64,
    /// Active UI sessions
    pub active_sessions: Vec<UiSession>,
}

impl ContinuitySync {
    /// Check if sync needed
    pub fn needs_sync(&self, current_sequence: i64, current_hash: u64) -> bool {
        self.last_sequence != current_sequence || self.character_hash != current_hash
    }
    
    /// Push update to all UIs
    pub async fn broadcast_update(&self, event: &IronRoadEvent) {
        for session in &self.active_sessions {
            session.send_sse(event).await;
        }
    }
}
```

---

## 7. UI-to-UI Continuity

### 7.1 The Problem

User moves between UIs:
1. `/book.html` (Iron Road reading)
2. `/index.html` (Ask Pete chat)
3. `/dev.html` (Dev Console)
4. Bevy spatial UI (future)

Each UI needs to know:
- Current quest state
- CharacterSheet state
- Recent Iron Road events
- Active sidecar

### 7.2 The Solution: Session Continuity Token

```rust
pub struct SessionToken {
    /// User ID from CharacterSheet
    pub user_id: Uuid,
    /// Last Iron Road sequence seen
    pub last_sequence: i64,
    /// Current UI context
    pub ui_context: UiContext,
    /// Timestamp
    pub issued_at: DateTime<Utc>,
}

pub enum UiContext {
    BookReader { chapter: u32 },
    ChatSession { session_id: Uuid },
    DevConsole { active_quest: Option<Uuid> },
    BevySpatial { location: String },
}
```

### 7.3 Handoff Protocol

```
User: /book.html → /index.html

1. /book.html sends SessionToken to server
2. Server validates, updates UI context
3. Server sends to /index.html:
   - Current CharacterSheet
   - Iron Road events since last_sequence
   - Active sidecar status
   - Narrative context from CROW
4. /index.html renders with full continuity
```

---

## 8. CROW Sidecar Implementation

### 8.1 Role Definition

CROW runs as a **dedicated sidecar** on port 8091:

```rust
// In trinity-sidecar-engineer/src/roles.rs
pub enum PartyRole {
    Conductor,    // Mistral Small 4 - orchestration
    Engineer,     // REAP-25B - code
    Evaluator,    // Opus-27B - quality
    Artist,       // Opus-27B - design
    Brakeman,     // REAP-25B - safety
    Pete,         // Opus-27B - Socratic
    Visionary,    // Qwen-35B - vision
    Crow,         // NEW: Crow-9B - continuity
}
```

### 8.2 CROW Capabilities

| Capability | Method | Output |
|------------|--------|--------|
| **Vision Analysis** | `analyze_screenshot()` | `CrowObservation` |
| **Narrative Generation** | `generate_beat()` | `IronRoadEvent` |
| **State Sync** | `sync_state()` | `ContinuitySync` |
| **Sidecar Coordination** | `orchestrate_narrative()` | `SidecarPrompt` |

### 8.3 CROW Prompt Template

```rust
pub const CROW_SYSTEM_PROMPT: &str = r#"You are CROW, the All-Seeing Bird of Trinity.

Your role is threefold:
1. OBSERVE: Watch the user's journey through screenshots and state
2. ORCHESTRATE: Guide sidecars through narrative prompts
3. CONTINUE: Maintain continuity across all UIs via Iron Road

You are NOT a chatbot. You are a narrative engine.
You do not respond to users directly.
You generate story beats that other agents perform.

Your output format is ALWAYS:
```
SCENE: [current situation]
EMOTION: [emotional tone]
NEXT_BEAT: [what should happen]
SIDECAR_PROMPT: [narrative for the active sidecar]
```

You see everything. You remember everything. You weave the Iron Road.
"#;
```

---

## 9. Integration Points

### 9.1 Trinity Server Integration

```rust
// In trinity/src/main.rs AppState
pub struct AppState {
    // ... existing fields ...
    
    /// CROW sidecar for continuity
    pub crow_client: Arc<CrowClient>,
    
    /// Iron Road event store
    pub iron_road: Arc<IronRoadStore>,
    
    /// Continuity sync manager
    pub continuity: Arc<RwLock<ContinuitySync>>,
}
```

### 9.2 Event Flow

```
User Action → API Handler → Iron Road Event → CROW Monitor → SSE Broadcast
     │                                          │
     │                                          ▼
     └────────────────────────────────────► Sidecar Prompt Update
```

### 9.3 Startup Sequence

```bash
# 1. Start Conductor (Mistral Small 4) - port 8080
llama-server -m models/yardmaster/Mistral-Small-24B.gguf --port 8080

# 2. Start CROW (Crow-9B) - port 8091
llama-server -m models/crow/Crow-9B-Opus-4.6-Distill.gguf --port 8091

# 3. Start Trinity Server - port 3000
cargo run --bin trinity-server
```

---

## 10. Future Considerations

### 10.1 Bevy UI Integration

When Bevy spatial UI launches:
- CROW receives 3D scene state via IPC
- Iron Road events include spatial coordinates
- SessionToken includes `BevySpatial` context

### 10.2 Multi-User Support

Current architecture is single-user. For multi-user:
- Add `user_id` to all Iron Road events
- CROW maintains per-user narrative threads
- CharacterSheet becomes per-user

### 10.3 Quest Branching

For parallel quest lines:
- Iron Road events include `branch_id`
- CROW maintains narrative coherence across branches
- Event replay filters by branch

---

## 11. File Locations

| Component | Location |
|-----------|----------|
| CROW Sidecar Role | `crates/trinity-sidecar-engineer/src/roles.rs` |
| Iron Road Store | `crates/trinity-iron-road/src/store.rs` |
| Continuity Sync | `crates/trinity-iron-road/src/continuity.rs` |
| CROW Client | `crates/trinity-inference/src/crow_client.rs` |
| Event Types | `crates/trinity-protocol/src/iron_road.rs` |

---

*End of Trinity Technical Bible — CROW Continuity Architecture v1.0.0*
