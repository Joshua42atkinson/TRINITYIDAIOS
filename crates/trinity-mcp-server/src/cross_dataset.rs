//! Cross-dataset semantic search and integration
//!
//! Enables unified search across Edu-ConvoKit, Blooms, and RICO datasets

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use tracing::{debug, info};

use super::embeddings::{EmbeddingModel, CrossDatasetSearch, CrossDatasetFilters, CrossDatasetResults};
use super::pedagogical_schema::{EduConversationResult, BloomsConceptResult, RicoScreenResult};

/// Unified search result across all datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSearchResult {
    pub dataset_type: DatasetType,
    pub score: f64,
    pub content: UnifiedContent,
    pub metadata: SearchMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatasetType {
    Conversation,
    BloomsConcept,
    UIScreen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedContent {
    Conversation {
        id: String,
        speaker: String,
        text: String,
        bloom_level: Option<String>,
        strategy: Option<String>,
    },
    BloomsConcept {
        id: String,
        concept: String,
        level: String,
        domain: String,
        definition: String,
        verbs: Vec<String>,
    },
    UIScreen {
        id: String,
        app: String,
        screen_type: String,
        elements: Vec<String>,
        accessibility: Option<f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub query_embedding_hash: String,
    pub search_context: Option<String>,
}

/// Context-aware query routing
pub struct QueryRouter {
    keyword_mappings: HashMap<String, DatasetType>,
    bloom_keywords: Vec<String>,
    ui_keywords: Vec<String>,
    conversation_keywords: Vec<String>,
}

impl QueryRouter {
    pub fn new() -> Self {
        let mut keyword_mappings = HashMap::new();

        // Blooms taxonomy keywords
        let bloom_keywords = vec![
            "remember", "understand", "apply", "analyze", "evaluate", "create",
            "learning", "objective", "assessment", "cognitive", "knowledge",
            "comprehension", "synthesis", "evaluation"
        ];

        // UI/UX keywords
        let ui_keywords = vec![
            "interface", "design", "ui", "ux", "screen", "layout", "button",
            "navigation", "accessibility", "wcag", "responsive", "mobile",
            "dashboard", "form", "menu", "user experience"
        ];

        // Conversation keywords
        let conversation_keywords = vec![
            "teaching", "learning", "pedagogy", "instruction", "education",
            "course", "lesson", "student", "teacher", "classroom", "curriculum",
            "sme", "expert", "interview", "discussion", "dialogue"
        ];

        // Map keywords to dataset types
        for keyword in bloom_keywords.iter() {
            keyword_mappings.insert(keyword.to_lowercase(), DatasetType::BloomsConcept);
        }

        for keyword in ui_keywords.iter() {
            keyword_mappings.insert(keyword.to_lowercase(), DatasetType::UIScreen);
        }

        for keyword in conversation_keywords.iter() {
            keyword_mappings.insert(keyword.to_lowercase(), DatasetType::Conversation);
        }

        Self {
            keyword_mappings,
            bloom_keywords,
            ui_keywords,
            conversation_keywords,
        }
    }

    /// Route query to most relevant datasets
    pub fn route_query(&self, query: &str) -> Vec<DatasetType> {
        let query_lower = query.to_lowercase();
        let mut dataset_scores: HashMap<DatasetType, u32> = HashMap::new();

        // Score each dataset based on keyword matches
        for word in query_lower.split_whitespace() {
            if let Some(dataset_type) = self.keyword_mappings.get(word) {
                *dataset_scores.entry(dataset_type.clone()).or_insert(0) += 1;
            }
        }

        // Always include all datasets if no strong matches
        if dataset_scores.is_empty() {
            return vec![
                DatasetType::Conversation,
                DatasetType::BloomsConcept,
                DatasetType::UIScreen,
            ];
        }

        // Return datasets sorted by score
        let mut datasets: Vec<_> = dataset_scores.into_iter().collect();
        datasets.sort_by(|a, b| b.1.cmp(&a.1));

        datasets.into_iter().map(|(dt, _)| dt).collect()
    }

    /// Extract search context from query
    pub fn extract_context(&self, query: &str) -> Option<String> {
        let query_lower = query.to_lowercase();

        // Check for specific contexts
        if query_lower.contains("assessment") || query_lower.contains("evaluation") {
            Some("assessment".to_string())
        } else if query_lower.contains("accessibility") || query_lower.contains("wcag") {
            Some("accessibility".to_string())
        } else if query_lower.contains("beginner") || query_lower.contains("introductory") {
            Some("beginner".to_string())
        } else if query_lower.contains("advanced") || query_lower.contains("expert") {
            Some("advanced".to_string())
        } else {
            None
        }
    }
}

/// Performance-optimized cross-dataset search
pub struct OptimizedCrossSearch {
    inner: CrossDatasetSearch,
    router: QueryRouter,
    query_cache: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, UnifiedSearchResult>>>,
}

impl OptimizedCrossSearch {
    pub fn new(embedding_model: std::sync::Arc<dyn EmbeddingModel>, db_pool: SqlitePool) -> Self {
        Self {
            inner: CrossDatasetSearch::new(embedding_model, db_pool),
            router: QueryRouter::new(),
            query_cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Perform optimized cross-dataset search
    pub async fn search(
        &self,
        query: &str,
        limit: u32,
        filters: Option<CrossDatasetFilters>,
    ) -> Result<Vec<UnifiedSearchResult>> {
        info!("Performing cross-dataset search for: {}", query);

        // Route query to relevant datasets
        let target_datasets = self.router.route_query(query);
        let context = self.router.extract_context(query);

        // Build filters based on routing
        let mut optimized_filters = filters.unwrap_or_default();

        if !target_datasets.contains(&DatasetType::Conversation) {
            optimized_filters.include_conversations = false;
        }
        if !target_datasets.contains(&DatasetType::BloomsConcept) {
            optimized_filters.include_blooms = false;
        }
        if !target_datasets.contains(&DatasetType::UIScreen) {
            optimized_filters.include_ui_screens = false;
        }

        // Perform search
        let results = self.inner.search_all(query, limit, Some(optimized_filters)).await?;

        // Convert to unified results
        let mut unified_results = Vec::new();

        // Add conversation results
        for conv in results.conversations {
            unified_results.push(UnifiedSearchResult {
                dataset_type: DatasetType::Conversation,
                score: conv.similarity,
                content: UnifiedContent::Conversation {
                    id: conv.conversation_id,
                    speaker: conv.speaker,
                    text: conv.text,
                    bloom_level: conv.bloom_level,
                    strategy: conv.pedagogical_strategy,
                },
                metadata: SearchMetadata {
                    timestamp: chrono::Utc::now(),
                    query_embedding_hash: format!("{:x}", md5::compute(query.as_bytes())),
                    search_context: context.clone(),
                },
            });
        }

        // Add Blooms results
        for concept in results.blooms_concepts {
            unified_results.push(UnifiedSearchResult {
                dataset_type: DatasetType::BloomsConcept,
                score: concept.similarity,
                content: UnifiedContent::BloomsConcept {
                    id: concept.concept_id,
                    concept: concept.concept,
                    level: concept.bloom_level,
                    domain: concept.domain,
                    definition: concept.definition,
                    verbs: serde_json::from_value(concept.example_verbs).unwrap_or_default(),
                },
                metadata: SearchMetadata {
                    timestamp: chrono::Utc::now(),
                    query_embedding_hash: format!("{:x}", md5::compute(query.as_bytes())),
                    search_context: context.clone(),
                },
            });
        }

        // Add UI screen results
        for screen in results.ui_screens {
            unified_results.push(UnifiedSearchResult {
                dataset_type: DatasetType::UIScreen,
                score: screen.similarity,
                content: UnifiedContent::UIScreen {
                    id: screen.ui_id,
                    app: screen.app_name,
                    screen_type: screen.screen_type,
                    elements: serde_json::from_value(screen.ui_elements).unwrap_or_default(),
                    accessibility: screen.accessibility_score,
                },
                metadata: SearchMetadata {
                    timestamp: chrono::Utc::now(),
                    query_embedding_hash: format!("{:x}", md5::compute(query.as_bytes())),
                    search_context: context.clone(),
                },
            });
        }

        // Sort by score
        unified_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        Ok(unified_results)
    }

    /// Get search suggestions based on query
    pub async fn get_suggestions(&self, query: &str) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();

        // Add dataset-specific suggestions
        let target_datasets = self.router.route_query(query);

        if target_datasets.contains(&DatasetType::BloomsConcept) {
            suggestions.extend(vec![
                "learning objectives for beginners".to_string(),
                "assessment strategies for evaluation".to_string(),
                "cognitive complexity analysis".to_string(),
            ]);
        }

        if target_datasets.contains(&DatasetType::UIScreen) {
            suggestions.extend(vec![
                "accessible interface design".to_string(),
                "mobile responsive layouts".to_string(),
                "WCAG compliance guidelines".to_string(),
            ]);
        }

        if target_datasets.contains(&DatasetType::Conversation) {
            suggestions.extend(vec![
                "expert teaching strategies".to_string(),
                "student engagement techniques".to_string(),
                "pedagogical best practices".to_string(),
            ]);
        }

        Ok(suggestions)
    }
}
