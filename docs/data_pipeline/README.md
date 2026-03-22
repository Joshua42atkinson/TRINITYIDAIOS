# Trinity Data Pipeline

## Overview

The Trinity Data Pipeline converts raw educational datasets into Parquet format for zero-copy memory mapping and ingests them into the MCP Memory system with pedagogical context.

## Architecture

```
Raw Datasets → Parquet Files → MCP Memory (PostgreSQL + pgvector)
     ↓              ↓                    ↓
  Edu-ConvoKit   Zero-copy           Vector Similarity
  Blooms Tax    Memory Map           Semantic Search
  RICO UI/UX    Arrow Format        Pedagogical Context
```

## Components

### 1. Data Pipeline (`crates/trinity-data-pipeline`)

Converts datasets to Parquet format using Polars and Arrow:

- **Edu-ConvoKit**: Professional educational conversations
- **Blooms Taxonomy**: Learning objectives and cognitive domains
- **RICO**: UI/UX screens with accessibility metrics

### 2. Pedagogical Schema (memory / pedagogical service layer)

Extends MCP Memory with specialized tables:

- `edu_convokit`: Educational conversations with embeddings
- `blooms_concepts`: Bloom's taxonomy with example verbs
- `rico_screens`: UI screens with visual embeddings
- `pedagogical_context`: Learning objectives and outcomes
- `agent_data_access`: Query pattern tracking

## Usage

### Convert Datasets to Parquet

```bash
# Convert all datasets
cargo run --bin ingest_datasets -- --input datasets --output data/parquets

# Convert specific dataset
cargo run --bin ingest_datasets -- --input datasets --output data/parquets --dataset edu_convokit
```

### Ingest Parquet Files to MCP Memory

```bash
# Ingest all datasets
cargo run --bin ingest_to_mcp -- --parquet-dir data/parquets

# Ingest specific dataset
cargo run --bin ingest_to_mcp -- --parquet-dir data/parquets --dataset blooms

# Custom database URL
cargo run --bin ingest_to_mcp -- --parquet-dir data/parquets --db-url "postgresql://user:pass@host/db"
```

## Data Schemas

### Edu-ConvoKit Schema

| Field | Type | Description |
|-------|------|-------------|
| conversation_id | String | Unique conversation identifier |
| speaker | String | Speaker role (interviewer, sme, etc.) |
| text | String | Conversation text |
| timestamp | DateTime | When the utterance occurred |
| intent | String | Pedagogical intent |
| pedagogical_strategy | String | Teaching strategy used |
| bloom_level | String | Bloom's taxonomy level |
| embedding | vector(384) | Semantic embedding |

### Blooms Taxonomy Schema

| Field | Type | Description |
|-------|------|-------------|
| concept_id | String | Unique concept identifier |
| concept | String | Concept name |
| bloom_level | String | L1-L6 Bloom's level |
| domain | String | cognitive/affective/psychomotor |
| definition | String | Concept definition |
| example_verbs | JSON | List of example verbs |
| sample_question | String | Sample assessment question |
| embedding | vector(384) | Semantic embedding |

### RICO Schema

| Field | Type | Description |
|-------|------|-------------|
| ui_id | String | Unique screen identifier |
| app_name | String | Application name |
| screen_type | String | Screen type (dashboard, form, etc.) |
| ui_elements | JSON | List of UI elements |
| layout_description | String | Visual layout description |
| accessibility_score | Float | WCAG accessibility score |
| learnability_score | Float | Ease of learning score |
| aesthetic_score | Float | Visual design score |
| wcag_compliance | String | WCAG compliance level |
| visual_embedding | vector(384) | Visual similarity embedding |

## Vector Search

The system supports similarity search across all datasets:

```rust
// Search educational conversations
let results = ped_schema.search_edu_conversations(
    query_embedding,
    10, // limit
    Some(EduConversationFilters {
        speaker: Some("sme".to_string()),
        bloom_level: Some("analyze".to_string()),
        ..Default::default()
    })
).await?;

// Search Blooms concepts
let concepts = ped_schema.search_blooms_concepts(
    query_embedding,
    5,
    Some("L3") // bloom level filter
).await?;

// Search UI screens
let screens = ped_schema.search_rico_screens(
    query_embedding,
    20,
    Some(RicoScreenFilters {
        min_accessibility_score: Some(0.9),
        ..Default::default()
    })
).await?;
```

## Performance Considerations

### Zero-Copy Memory Mapping

- Parquet files use Arrow columnar format
- Zero-copy reads from memory-mapped files
- Efficient for large dataset scanning

### Vector Indexing

- IVFFlat indexes for similarity search
- 384-dimensional embeddings
- Cosine similarity distance metric
- Configurable list count for index tuning

### Agent Access Patterns

The system tracks query patterns to optimize:

```rust
// Track agent access
ped_schema.track_agent_access(
    "conductor",
    "edu_convokit",
    "semantic_search",
    Some(json!({"query_time_ms": 45}))
).await?;
```

## Integration with ADDIE Workflow

The data pipeline supports the ADDIE instructional design phases:

1. **Analysis**: Search Edu-ConvoKit for SME insights
2. **Design**: Use Blooms concepts for learning objectives
3. **Development**: Reference RICO for UI best practices
4. **Implementation**: Track with pedagogical context
5. **Evaluation**: Monitor via agent access patterns

## Future Enhancements

1. **Real Embeddings**: Replace hash-based with sentence transformers
2. **Incremental Updates**: Support for live data ingestion
3. **Cross-Dataset Search**: Unified semantic search across all data
4. **Performance Metrics**: Detailed query analytics dashboard
5. **Data Validation**: Schema validation and quality checks
