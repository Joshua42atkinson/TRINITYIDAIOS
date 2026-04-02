# Trinity Dataset Integration - Quick Start Guide

## Prerequisites
- PostgreSQL 16+ with pgvector extension
- Python 3.8+ with sentence-transformers
- Rust toolchain

## Setup

### 1. Database Setup
```bash
# Start PostgreSQL
sudo systemctl start postgresql

# Create database and user
sudo -u postgres psql
CREATE USER trinity WITH PASSWORD '6226';
CREATE DATABASE trinity OWNER trinity;
\c trinity
CREATE EXTENSION IF NOT EXISTS vector;
```

### 2. Python Dependencies
```bash
pip install sentence-transformers numpy
```

### 3. Rust Build
```bash
cargo build --release
```

## Basic Usage

### 1. Ingest Datasets
```bash
# Convert and ingest datasets
cargo run --bin ingest_datasets
cargo run --bin ingest_to_mcp -- --dataset edu_convokit
cargo run --bin ingest_to_mcp -- --dataset blooms
cargo run --bin ingest_to_mcp -- --dataset rico
```

### 2. Start MCP Server
```bash
cargo run --bin trinity-mcp-server
```

### 3. Query Datasets
```bash
# Run demo
cargo run --example dataset_query_demo
```

### 4. Conductor Agent Demo
```bash
cargo run --example conductor_demo
```

### 5. Real-time Ingestion
```bash
cargo run --bin real_time_ingest -- --watch-dir data/parquets
```

## API Examples

### Dataset Query
```rust
use trinity_kernel::dataset_query::{DatasetQueryImpl, DatasetQuery};

let query = DatasetQueryImpl::new(db_pool);

// Search conversations
let conversations = query.search_conversations(
    "machine learning",
    Some(10),
    None
).await?;

// Get learning objectives
let objectives = query.get_learning_objectives(
    "python programming",
    Some(vec!["L1", "L2", "L3"])
).await?;
```

### Cross-Dataset Search
```rust
use trinity_mcp_server::cross_dataset::OptimizedCrossSearch;

let search = OptimizedCrossSearch::new(embedding_model, db_pool);

let results = search.search(
    "accessible course design",
    10,
    None
).await?;
```

### Conductor Agent
```rust
use trinity_kernel::conductor_agent::{ConductorAgent, ProjectContext};

let mut conductor = ConductorAgent::new(dataset_query, cross_search);

// Start ADDIE workflow
conductor.start_project(project).await?;

// Advance phases
for _ in 0..5 {
    conductor.advance_phase().await?;
}

// Get report
let report = conductor.generate_report();
```

## Performance Tips

1. **Batch Operations**: Always encode multiple texts at once
2. **Use Filters**: Limit search scope for better performance
3. **Enable Caching**: Embeddings are automatically cached
4. **Monitor Memory**: Model requires ~500MB RAM

## Troubleshooting

### Embedding Model Issues
```bash
# Check Python installation
python3 -c "import sentence_transformers; print('OK')"

# Install if missing
pip install sentence-transformers
```

### Database Issues
```bash
# Check pgvector
psql -d trinity -c "SELECT extversion FROM pg_extension WHERE extname = 'vector';"

# Install if missing
psql -d trinity -c "CREATE EXTENSION IF NOT EXISTS vector;"
```

### Performance Issues
- Check database connections
- Monitor cache hit rates
- Verify index creation

## Next Steps

1. **Fine-tune Models**: Train on educational data
2. **Expand Datasets**: Add more educational content
3. **Multi-modal**: Add image embeddings
4. **Federation**: Cross-instance search

## Documentation

- [Embeddings Integration](EMBEDDINGS_INTEGRATION.md)
- [Conductor Agent Guide](CONDUCTOR_AGENT_GUIDE.md)
- [Dataset Integration Complete](../research_progress/DATASET_INTEGRATION_COMPLETE.md)
- [Next Steps Implemented](../research_progress/NEXT_STEPS_IMPLEMENTED.md)
