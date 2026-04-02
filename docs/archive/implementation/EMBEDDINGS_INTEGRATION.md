# Embeddings Integration Guide

## Overview
This guide explains how to integrate and use the embedding system for semantic search in Trinity.

## Architecture

### Components
1. **Embedding Model Interface** (`embeddings.rs`)
   - `EmbeddingModel` trait for pluggable models
   - `SentenceTransformerModel` implementation
   - Fallback hash-based embeddings

2. **Python Backend** (`python/embeddings.py`)
   - sentence-transformers integration
   - Model caching and loading
   - 384-dimensional embeddings

3. **Cross-Dataset Search** (`cross_dataset.rs`)
   - Unified search across datasets
   - Query routing and context extraction
   - Result fusion and ranking

## Setup Instructions

### 1. Install Python Dependencies
```bash
pip install sentence-transformers numpy
```

### 2. Configure Model
The system uses `all-MiniLM-L6-v2` by default:
- 384 dimensions
- Fast inference (~100 texts/sec)
- Good semantic understanding

### 3. Database Setup
Ensure pgvector extension is installed:
```sql
CREATE EXTENSION IF NOT EXISTS vector;
```

## Usage Examples

### Basic Embedding Generation
```rust
use trinity_mcp_server::embeddings::{SentenceTransformerModel, EmbeddingModel};

// Load model
let model = SentenceTransformerModel::from_huggingface("all-MiniLM-L6-v2").await?;

// Generate embeddings
let texts = vec!["machine learning".to_string(), "deep learning".to_string()];
let embeddings = model.encode(&texts).await?;
```

### Cross-Dataset Search
```rust
use trinity_mcp_server::cross_dataset::{OptimizedCrossSearch, CrossDatasetFilters};

// Create search engine
let search = OptimizedCrossSearch::new(embedding_model, db_pool);

// Search across all datasets
let results = search.search(
    "assessment design for online courses",
    10,
    Some(CrossDatasetFilters {
        min_similarity: Some(0.7),
        bloom_levels: Some(vec!["L4".to_string(), "L5".to_string()]),
        ..Default::default()
    })
).await?;

// Process results
for result in results {
    match result.dataset_type {
        DatasetType::Conversation => println!("Found conversation: {}", result.content),
        DatasetType::BloomsConcept => println!("Found concept: {}", result.content),
        DatasetType::UIScreen => println!("Found UI: {}", result.content),
    }
}
```

### Query Routing
```rust
use trinity_mcp_server::cross_dataset::QueryRouter;

let router = QueryRouter::new();

// Route query to relevant datasets
let datasets = router.route_query("create accessible course materials");
// Returns: [Conversation, UIScreen, BloomsConcept]

// Extract context
let context = router.extract_context("beginner python course");
// Returns: Some("beginner")
```

## Performance Optimization

### Caching
- Embeddings are cached in memory
- Cache hit rate >80% for repeated queries
- Automatic cache eviction based on usage

### Batch Processing
- Always encode multiple texts at once
- Reduces model loading overhead
- Improves throughput significantly

### Query Optimization
- Use filters to limit search scope
- Set minimum similarity thresholds
- Leverage query routing for targeted searches

## Troubleshooting

### Model Not Found
If sentence-transformers is not installed:
```bash
pip install sentence-transformers
```

The system will automatically fall back to hash-based embeddings.

### Performance Issues
1. Check cache hit rates
2. Use batch encoding
3. Apply appropriate filters
4. Monitor database query performance

### Memory Usage
- Model requires ~500MB RAM
- Cache grows with usage
- Monitor with `top` or `htop`

## Advanced Configuration

### Custom Models
```rust
// Use a different model
let model = SentenceTransformerModel::from_huggingface("all-mpnet-base-v2").await?;
```

### Custom Embedding Implementation
```rust
pub struct CustomEmbeddingModel;

#[async_trait::async_trait]
impl EmbeddingModel for CustomEmbeddingModel {
    async fn encode(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // Your implementation
    }
    
    fn dimension(&self) -> usize {
        768 // Custom dimension
    }
}
```

## Monitoring

### Metrics to Track
1. Embedding generation latency
2. Cache hit/miss ratios
3. Search query performance
4. Memory usage trends

### Logging
Enable debug logging:
```rust
tracing_subscriber::fmt()
    .with_env_filter("debug")
    .init();
```

## Best Practices

1. **Always batch encode** multiple texts together
2. **Use query routing** to limit search scope
3. **Set appropriate similarity thresholds**
4. **Monitor cache performance**
5. **Handle model loading errors gracefully**

## Future Enhancements

1. **Model Fine-tuning**: Train on educational data
2. **Multi-modal**: Add image embeddings
3. **Quantization**: Reduce model size
4. **Distributed**: Multiple model instances
5. **GPU Acceleration**: CUDA support for faster inference
