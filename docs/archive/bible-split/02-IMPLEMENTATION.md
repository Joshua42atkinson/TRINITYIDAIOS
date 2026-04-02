# Trinity Implementation Bible
**Document:** 02-IMPLEMENTATION.md  
**Purpose:** APIs, schemas, configs, code specifications  
**Lines:** ~600  
**Isomorphic to:** [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md), [03-OPERATIONS.md §2](03-OPERATIONS.md)

---

## 1. Configuration System

### 1.1 Runtime Configuration (TOML)

**Default config** (`configs/runtime/default.toml`):
```toml
[node]
type = "Auto"  # Auto-detect Brain/Body

[model]
model_dir = "~/.trinity/models"
default_model = "Llama-4-Scout-17B.safetensors"
n_gpu_layers = -1  # -1 = full offload, 0 = CPU only
context_size = 32768
batch_size = 512
n_threads = 16

[memory]
vector_store_path = "~/.trinity/vectors"
embedding_dim = 384  # MiniLM dimension
max_recall = 20

[network]
listen_addr = "0.0.0.0:9000"
brain_addr = "localhost:9000"

[hardware]
auto_detect = true
```

### 1.2 TrinityConfig Struct

**Source:** `crates/trinity-kernel/src/config.rs`

```rust
/// Main configuration struct — loaded from TOML or env vars
pub struct TrinityConfig {
    pub node_type: NodeType,      // Brain | Body | Combined | Auto
    pub model: ModelConfig,
    pub memory: MemoryConfig,
    pub network: NetworkConfig,
}

pub struct ModelConfig {
    pub model_path: PathBuf,
    pub n_gpu_layers: u32,        // 999 = full offload (Strix Halo)
    pub context_size: usize,      // 32768 default, 128000 for Nemotron
    pub batch_size: usize,        // 512 default, 2048 for code generation
    pub n_threads: usize,         // 8-16 for Strix Halo, 4 for laptop
    pub kv_cache_type: Option<String>, // "q8_0", "q4_0", "f16"
    pub cache_v_quant: Option<bool>,   // true = quantize V cache
}

pub struct MemoryConfig {
    pub vector_store_path: PathBuf,   // ~/.trinity/vectors
    pub embedding_dim: usize,         // 384 (MiniLM-L6-v2)
    pub max_recall: usize,            // 10-20 memories per query
}

pub struct NetworkConfig {
    pub listen_addr: String,        // "0.0.0.0:9000"
    pub brain_addr: Option<String>, // Tailscale: 100.115.247.4:9000
    pub body_addr: Option<String>,  // Tailscale: 100.84.217.60:9000
}

pub enum NodeType {
    Brain,      // Runs models (desktop with GPU)
    Body,       // UI/client only (laptop, connects to Brain)
    Combined,   // Single machine does both (future)
    Auto,       // Detect based on hardware
}
```

### 1.3 Profile-Specific Configurations

| Profile | Model Path | Context | Batch | GPU Layers | Use Case |
|---------|-----------|---------|-------|------------|----------|
| **conductor** | `~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf` | 32768 | 512 | 99 | Main orchestrator |
| **engineer** | `~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf` | 32768 | 2048 | 99 | Code generation |
| **evaluator** | `~/trinity-models/gguf/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf` | 32768 | 512 | 99 | QM evaluation |
| **artist** | `~/trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf` | 32768 | 512 | 99 | Creative design |
| **researcher** | `~/trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf` | 32000 | 512 | 99 | Quick queries |
| **rust_coder** | Same as engineer | 32768 | 2048 | 99 | Alias for engineer |

### 1.4 Environment Variables

| Variable | Purpose | Example | Default |
|----------|---------|---------|---------|
| `TRINITY_CONFIG` | Config file path | `configs/runtime/dev.toml` | `configs/runtime/default.toml` |
| `TRINITY_PROFILE` | Load profile | `conductor`, `engineer` | `rust_coder` |
| `HOME` | Vector store base | `/home/joshua` | System default |
| `DATABASE_URL` | PostgreSQL connection | `postgres://trinity:trinity@localhost/trinity` | `postgres://postgres:postgres@localhost:5432/trinity` |
| `LLAMA_URL` | llama-server endpoint | `http://localhost:8080` | `http://127.0.0.1:8080` |

### 1.5 Hardware-Specific Configs

**AMD Strix Halo (Desktop Brain):**
```rust
// From trinity-kernel/src/config.rs
pub fn strix_halo_brain() -> Self {
    Self {
        node_type: NodeType::Brain,
        model: ModelConfig {
            model_path: PathBuf::from("~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf"),
            n_gpu_layers: 999,        // Full offload
            context_size: 32768,
            batch_size: 2048,
            n_threads: 16,
            kv_cache_type: Some("q8_0".to_string()),
            cache_v_quant: Some(true),
        },
        memory: MemoryConfig {
            vector_store_path: PathBuf::from("/home/joshua/.trinity/vectors"),
            embedding_dim: 384,
            max_recall: 20,
        },
        network: NetworkConfig {
            listen_addr: "0.0.0.0:9000".to_string(),
            brain_addr: None,         // This IS the brain
            body_addr: Some("100.84.217.60:9000".to_string()), // Tailscale to laptop
        },
    }
}
```

**Laptop (Body Node):**
```rust
pub fn laptop_body() -> Self {
    Self {
        node_type: NodeType::Body,
        model: ModelConfig {
            model_path: PathBuf::new(), // No local model
            n_gpu_layers: 0,          // CPU-only (or no model)
            context_size: 0,
            batch_size: 0,
            n_threads: 0,
            kv_cache_type: None,
            cache_v_quant: None,
        },
        memory: MemoryConfig {
            vector_store_path: PathBuf::from("/home/joshua/.trinity/vectors"),
            embedding_dim: 384,
            max_recall: 10,           // Less local context
        },
        network: NetworkConfig {
            listen_addr: "0.0.0.0:9001".to_string(),
            brain_addr: Some("100.115.247.4:9000".to_string()), // Tailscale to desktop
            body_addr: None,
        },
    }
}
```

---

## 2. API Specification

### 2.1 Trinity Server API (Port 3000)

**Base URL:** `http://localhost:3000`

#### Health & Status

| Endpoint | Method | Request | Response | Purpose |
|----------|--------|---------|----------|---------|
| `/api/health` | GET | — | `{"status": "ok", "llama": true, "db": true}` | System health check |
| `/api/status` | GET | — | `{"models": [...], "memory_used": "42GB"}` | Detailed status |

#### Chat Endpoints

| Endpoint | Method | Request Body | Response | Purpose |
|----------|--------|--------------|----------|---------|
| `/api/chat` | POST | `{"message": "hi", "use_rag": true}` | `{"response": "...", "latency_ms": 1234}` | Standard chat |
| `/api/chat/stream` | POST | `{"message": "hi"}` | SSE stream | Streaming response |
| `/api/chat/agent` | POST | `{"message": "code", "tools": ["read_file"]}` | SSE stream | Agentic with tools |

**ChatRequest struct:**
```rust
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub use_rag: bool,           // Include RAG context
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,         // Default: 1024
}

fn default_max_tokens() -> u32 {
    1024
}
```

#### Quest System (ADDIE-C-R-A-P-E-Y-E)

| Endpoint | Method | Request | Response | Purpose |
|----------|--------|---------|----------|---------|
| `/api/quest` | GET | — | `GameState` | Current quest state |
| `/api/quest/complete` | POST | `{"objective_id": "ch1_a1"}` | `GameState` | Mark objective done |
| `/api/quest/advance` | POST | — | `GameState` | Manual phase advance |
| `/api/quest/party` | POST | `{"member_id": "engineer"}` | `GameState` | Toggle party member |
| `/api/quest/subject` | POST | `{"subject": "Biology"}` | `GameState` | Set core subject |

**GameState struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub quest: QuestState,
    pub stats: PlayerStats,
    pub party: Vec<PartyMember>,
    pub inventory: Vec<String>,
}

pub struct QuestState {
    pub quest_id: String,
    pub quest_title: String,
    pub hero_stage: HeroStage,      // Ch 1-12
    pub current_phase: Phase,       // ADDIE-C-R-A-P-E-Y-E
    pub phase_objectives: Vec<Objective>,
    pub completed_phases: Vec<Phase>,
    pub completed_chapters: Vec<u8>,
    pub xp_earned: u32,
    pub coal_used: f32,
    pub steam_generated: f32,
    pub subject: String,
    pub game_title: String,
}

pub struct PlayerStats {
    pub traction: u32,          // d20 bonus for logistics
    pub velocity: u32,          // d20 bonus for inspiration  
    pub combustion: u32,        // d20 bonus for complexity
    pub coal_reserves: f32,     // Stamina 0-100+
    pub resonance: i32,         // Mastery level 1-12
    pub total_xp: u32,
    pub quests_completed: u32,
}
```

#### Agentic Tools

| Endpoint | Method | Response | Purpose |
|----------|--------|----------|---------|
| `/api/tools` | GET | `Vec<ToolInfo>` | List available tools |
| `/api/tools/execute` | POST | `ToolResponse` | Execute a tool |

**Tool definitions:**
```rust
pub async fn list_tools() -> Json<Vec<ToolInfo>> {
    Json(vec![
        ToolInfo {
            name: "read_file".to_string(),
            description: "Read a file's contents".to_string(),
            params: vec!["path".to_string()],
        },
        ToolInfo {
            name: "write_file".to_string(),
            description: "Write content to a file".to_string(),
            params: vec!["path".to_string(), "content".to_string()],
        },
        ToolInfo {
            name: "list_dir".to_string(),
            description: "List directory contents".to_string(),
            params: vec!["path".to_string()],
        },
        ToolInfo {
            name: "shell".to_string(),
            description: "Execute a shell command (sandboxed)".to_string(),
            params: vec!["command".to_string()],
        },
        ToolInfo {
            name: "search_files".to_string(),
            description: "Search for text in files using grep".to_string(),
            params: vec!["query".to_string(), "path".to_string()],
        },
        ToolInfo {
            name: "sidecar_status".to_string(),
            description: "Check status of AI sidecars".to_string(),
            params: vec![],
        },
        ToolInfo {
            name: "sidecar_start".to_string(),
            description: "Start a model sidecar".to_string(),
            params: vec!["model".to_string()],
        },
    ])
}
```

#### RAG & Documents

| Endpoint | Method | Request | Purpose |
|----------|--------|---------|---------|
| `/api/ingest` | POST | `{"path": "doc.md"}` | Ingest document |
| `/api/models` | GET | — | List available models |

#### Iron Road Book

| Endpoint | Method | Response | Purpose |
|----------|--------|----------|---------|
| `/api/book` | GET | `BookState` | Current book content |
| `/api/book/stream` | GET | SSE | Real-time updates |

#### Creative Sidecar

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/creative/status` | GET | Check ComfyUI, etc. |
| `/api/creative/image` | POST | Generate image |
| `/api/creative/video` | POST | Generate video |
| `/api/creative/music` | POST | Generate music |
| `/api/creative/settings` | GET/POST | Creative config |

#### Voice (PersonaPlex)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/voice/status` | GET | PersonaPlex status |
| `/api/voice/pete` | POST | Audio conversation |
| `/api/voice/pete/text` | POST | Text through voice pipeline |

### 2.2 Sidecar Engineer API (Port 8090)

**Base URL:** `http://localhost:8090`

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/status` | GET | Sidecar health, loaded model |
| `/quests` | GET | List all quests |
| `/quest/:id` | GET | Specific quest details |
| `/quest/claim` | POST | Claim a quest |
| `/quest/execute` | POST | Execute quest manually |
| `/autonomous/start` | POST | Start autonomous loop |
| `/autonomous/stop` | POST | Stop autonomous loop |
| `/think` | POST | Direct Opus prompt (Shield) |
| `/code` | POST | Direct REAP prompt (Sword) |
| `/shutdown` | POST | Graceful shutdown |

### 2.3 llama-server API (Port 8080, 8081, 8082)

**Base URL:** `http://localhost:8080/v1`

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Model health |
| `/v1/chat/completions` | POST | OpenAI-compatible chat |
| `/v1/models` | GET | List loaded models |

**OpenAI-compatible request:**
```json
{
  "messages": [
    {"role": "system", "content": "You are Trinity."},
    {"role": "user", "content": "Hello"}
  ],
  "max_tokens": 1024,
  "temperature": 0.7,
  "stream": true
}
```

---

## 3. Database Schema

### 3.1 Core RAG Tables

**document_embeddings** (from the memory/RAG service layer):
```sql
CREATE TABLE document_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    doc_path TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    embedding vector(384),          -- MiniLM-L6-v2 dimension
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(doc_path, chunk_index)
);

CREATE INDEX idx_doc_embeddings_path 
ON document_embeddings(doc_path);
```

**implementation_status**:
```sql
CREATE TABLE implementation_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feature_name TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'in_progress', 'completed', 'blocked')),
    description TEXT,
    last_checked TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    evidence JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### 3.2 Quest System Tables

**quest_state** (from `crates/trinity/src/quests.rs`):
```sql
CREATE TABLE IF NOT EXISTS quest_state (
    id SERIAL PRIMARY KEY,
    player_id TEXT NOT NULL DEFAULT 'default',
    chapter INT NOT NULL DEFAULT 1,           -- Hero's Journey chapter 1-12
    phase TEXT NOT NULL DEFAULT 'analysis',   -- ADDIE-C-R-A-P-E-Y-E phase
    xp INT NOT NULL DEFAULT 0,
    coal REAL NOT NULL DEFAULT 87.0,          -- Stamina (0-100)
    steam REAL NOT NULL DEFAULT 0.0,          -- Progress momentum
    resonance INT NOT NULL DEFAULT 1,         -- Mastery level
    stats JSONB NOT NULL DEFAULT '{"traction":3,"velocity":2,"combustion":1,"coal_reserves":87.0,"resonance":1,"total_xp":0,"quests_completed":0}'::jsonb,
    inventory JSONB NOT NULL DEFAULT '["📐 ADDIE Framework","🌸 Bloom\'s Taxonomy","🧠 Cognitive Load Theory"]'::jsonb,
    subject TEXT NOT NULL DEFAULT '',         -- Core subject area
    game_title TEXT NOT NULL DEFAULT '',    -- User's game name
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(player_id)
);

CREATE INDEX idx_quest_state_player ON quest_state(player_id);
```

**quest_history**:
```sql
CREATE TABLE IF NOT EXISTS quest_history (
    id SERIAL PRIMARY KEY,
    player_id TEXT NOT NULL DEFAULT 'default',
    quest_id TEXT NOT NULL,
    quest_title TEXT NOT NULL,
    status TEXT NOT NULL,                     -- active, completed, abandoned
    xp_earned INT NOT NULL DEFAULT 0,
    duration_secs INT,                        -- Time spent
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    results JSONB                             -- Completion data
);

CREATE INDEX idx_quest_history_player ON quest_history(player_id);
CREATE INDEX idx_quest_history_quest ON quest_history(quest_id);
```

### 3.3 Pedagogical Schema

**edu_convokit** (professional interview dataset):
```sql
CREATE TABLE edu_convokit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id TEXT NOT NULL,
    speaker TEXT NOT NULL,
    text TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE,
    intent TEXT,
    pedagogical_strategy TEXT,
    bloom_level TEXT,                         -- remember, understand, apply, etc.
    embedding vector(384),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX edu_convokit_embedding_idx
ON edu_convokit USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
```

**blooms_concepts**:
```sql
CREATE TABLE blooms_concepts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    concept_id TEXT UNIQUE NOT NULL,
    concept TEXT NOT NULL,
    bloom_level TEXT NOT NULL,                -- remember, understand, apply, analyze, evaluate, create
    domain TEXT NOT NULL,
    definition TEXT NOT NULL,
    example_verbs JSONB NOT NULL,             -- ["define", "list", "describe"]
    sample_question TEXT,
    embedding vector(384),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX blooms_concepts_embedding_idx
ON blooms_concepts USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
```

**rico_screens** (UI/UX dataset):
```sql
CREATE TABLE rico_screens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ui_id TEXT UNIQUE NOT NULL,
    app_name TEXT NOT NULL,
    screen_type TEXT NOT NULL,
    ui_elements JSONB NOT NULL,
    layout_description TEXT,
    accessibility_score FLOAT,
    learnability_score FLOAT,
    aesthetic_score FLOAT,
    wcag_compliance TEXT,
    visual_embedding vector(384),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX rico_screens_visual_embedding_idx
ON rico_screens USING ivfflat (visual_embedding vector_cosine_ops) WITH (lists = 100);
```

### 3.4 Vector Search Queries

**Similarity search (RAG):**
```sql
-- Find top 10 most similar chunks
SELECT doc_path, chunk_index, content,
       embedding <=> query_embedding AS distance
FROM document_embeddings
ORDER BY distance ASC
LIMIT 10;
```

**Hybrid search (text + vector):**
```sql
-- Full-text + vector similarity
SELECT 
    d.id,
    d.content,
    ts_rank(to_tsvector('english', d.content), plainto_tsquery('english', $1)) as text_rank,
    1 - (d.embedding <=> $2) as vector_similarity
FROM document_embeddings d
WHERE to_tsvector('english', d.content) @@ plainto_tsquery('english', $1)
ORDER BY text_rank * 0.3 + vector_similarity * 0.7 DESC
LIMIT 10;
```

---

## 4. Model Launch Parameters

### 4.1 llama-server Commands

**Conductor (Mistral Small 4):**
```bash
./llama.cpp/build/bin/llama-server \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 99 \                          # GPU layers (99 = all)
  -c 32768 \                         # Context size
  -b 512 \                           # Batch size
  --port 8080 \
  --host 0.0.0.0
```

**Engineer (Qwen3-Coder-25B):**
```bash
./llama.cpp/build/bin/llama-server \
  -m ~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf \
  -ngl 99 \
  -c 32768 \
  -b 2048 \                          # Larger batch for code
  --port 8082 \
  --host 0.0.0.0
```

**Swarm/Researcher (Crow-9B or OmniCoder-9B):**
```bash
./llama.cpp/build/bin/llama-server \
  -m ~/trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf \
  -ngl 99 \
  -c 32000 \
  --port 8081 \
  --host 0.0.0.0
```

### 4.2 Parameter Explanation

| Flag | Meaning | Recommended |
|------|---------|-------------|
| `-m` | Model path | Profile-specific |
| `-ngl` | GPU layers | 99 (full offload on Strix Halo) |
| `-c` | Context size | 32768 (128K for Nemotron-120B) |
| `-b` | Batch size | 512 (2048 for code) |
| `-t` | Threads | 16 (Strix Halo), 4 (laptop) |
| `--port` | Listen port | 8080, 8081, 8082 |
| `--host` | Bind address | 0.0.0.0 (all interfaces) |

---

## 5. File System Locations

| Component | Path | Document |
|-----------|------|----------|
| Master Bible | `TRINITY_TECHNICAL_BIBLE.md` | [00-MASTER](00-MASTER.md) |
| Architecture | `docs/bible/01-ARCHITECTURE.md` | [01-ARCHITECTURE](01-ARCHITECTURE.md) |
| Implementation | `docs/bible/02-IMPLEMENTATION.md` | This file |
| Operations | `docs/bible/03-OPERATIONS.md` | [03-OPERATIONS](03-OPERATIONS.md) |
| Models | `docs/bible/04-MODELS.md` | [04-MODELS](04-MODELS.md) |
| §17 Standard | `docs/bible/§17-STANDARD.md` | [§17-STANDARD](§17-STANDARD.md) |
| Configs | `configs/runtime/*.toml` | [§5.1](02-IMPLEMENTATION.md) |
| Hardware configs | `configs/hardware/*.toml` | [§5.1](02-IMPLEMENTATION.md) |
| Models | `~/.trinity/models/` | [04-MODELS.md](04-MODELS.md) |
| Vectors | `~/.trinity/vectors/` | [§3](02-IMPLEMENTATION.md) |
| Database | `postgres://localhost/trinity` | [§3](02-IMPLEMENTATION.md) |
| Character sheet | `~/.trinity/character_sheet.json` | [01-ARCHITECTURE.md §6](01-ARCHITECTURE.md) |
| Quests | `quests/board/*.json` | [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md) |
| Books | `docs/books_of_the_bible/*.md` | [01-ARCHITECTURE.md §6.3](01-ARCHITECTURE.md) |
| Logs | `~/.trinity/logs/` | [03-OPERATIONS.md](03-OPERATIONS.md) |

---

## 6. Port Allocation

| Port | Service | Model | Notes |
|------|---------|-------|-------|
| 3000 | Trinity Server | — | Main HTTP API, static files |
| 8080 | llama-server | Mistral Small 4 | Conductor, always on |
| 8081 | llama-server | Crow-9B, etc. | Swarm/Researcher |
| 8082 | llama-server | Qwen3-Coder-25B | Engineer (on-demand) |
| 8090 | Sidecar Engineer | — | Axum API for sidecar control |
| 8091 | Sidecar Artist | — | Creative generation |
| 8092 | Sidecar Evaluator | — | QM evaluation |
| 8093 | Sidecar Pete | — | Socratic dialogue |
| 8094 | Sidecar Visionary | — | Vision analysis |
| 9000 | Trinity Kernel | — | Brain node (desktop) |
| 9001 | Trinity Kernel | — | Body node (laptop) |

---

## 7. Cross-References

| Section | Reference |
|---------|-----------|
| ADDIE framework detail | [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md) |
| Launch commands | [03-OPERATIONS.md §2](03-OPERATIONS.md) |
| Troubleshooting | [03-OPERATIONS.md §6](03-OPERATIONS.md) |
| Model cards | [04-MODELS.md §2](04-MODELS.md) |
| Delegation matrix | [04-MODELS.md §3](04-MODELS.md) |

---

*End of 02-IMPLEMENTATION.md — APIs, Schemas, and Configs*
