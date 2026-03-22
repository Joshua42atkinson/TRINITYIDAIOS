#!/bin/bash

# Update all subagent lib.rs files to use real brains

SUBAGENTS=("draftsman" "engineer" "yardmaster" "brakeman" "diffusion" "nitrogen" "omni" "conductor")

for subagent in "${SUBAGENTS[@]}"; do
    echo "Updating $subagent..."
    
    # Update lib.rs to import and use real brain
    cat > "crates/trinity-subagents/trinity-$subagent/src/lib.rs" << EOF
//! Trinity ${subagent^} - Specialized Subagent

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tracing::{info, debug};

use trinity_kernel::{
    brain::{Brain, StreamToken, GrammarSpec},
    subagent_brain::{
        SubagentBrain, TaskType, TaskContext, TaskResult, SubagentConfig,
        MemoryRequirement, SubagentMetrics, MetricsStorage
    },
};

mod real_brain;
pub use real_brain::Real${subagent^}Brain;

/// ${subagent^} subagent
pub struct ${subagent^}Brain {
    /// The underlying real brain
    brain: Real${subagent^}Brain,
    /// Subagent configuration
    config: SubagentConfig,
    /// Performance metrics
    metrics: MetricsStorage,
}

impl ${subagent^}Brain {
    /// Create a new ${subagent} brain
    pub async fn new() -> Result<Self> {
        info!("🔧 Initializing ${subagent^}Brain");
        
        let brain = Real${subagent^}Brain::new().await?;
        
        let config = SubagentConfig {
            id: "$subagent-specialist".to_string(),
            name: "${subagent^} (Specialist)".to_string(),
            description: "Specialized subagent for ${subagent} tasks".to_string(),
            supported_tasks: vec![
                // Add appropriate task types based on subagent
$(case $subagent in
    "engineer")
        echo "                TaskType::ThreeDModel," ;;
    "yardmaster")
        echo "                TaskType::QualityCheck," ;;
    "brakeman")
        echo "                TaskType::SecurityTest," ;;
    "diffusion")
        echo "                TaskType::ImageGeneration," ;;
    "nitrogen")
        echo "                TaskType::AudioGeneration," ;;
    "omni")
        echo "                TaskType::MultimodalProcessing," ;;
    *)
        echo "                TaskType::InstructionalDesign," ;;
esac
            ],
            memory_requirement: MemoryRequirement {
                min_ram_gb: 8.0,
                min_vram_gb: 6.0,
                recommended_ram_gb: 16.0,
                recommended_vram_gb: 12.0,
            },
            priority: 3,
            max_concurrent_tasks: 3,
            parameters: {
                let mut params = HashMap::new();
                params.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
                params.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(4096)));
                params.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.9).unwrap()));
                params
            },
        };
        
        Ok(Self {
            brain,
            config,
            metrics: MetricsStorage::new(),
        })
    }
}

#[async_trait]
impl Brain for ${subagent^}Brain {
    async fn think(&self, prompt: &str) -> Result<String> {
        let start = std::time::Instant::now();
        
        let result = self.brain.think(prompt).await;
        
        let duration = start.elapsed().as_millis() as u64;
        self.metrics.update(duration, result.is_ok());
        
        result
    }
    
    async fn think_with_grammar(&self, prompt: &str, grammar: GrammarSpec) -> Result<String> {
        let start = std::time::Instant::now();
        
        let result = self.brain.think_with_grammar(prompt, grammar).await;
        
        let duration = start.elapsed().as_millis() as u64;
        self.metrics.update(duration, result.is_ok());
        
        result
    }
    
    async fn think_stream(
        &self,
        prompt: &str,
        token_tx: mpsc::Sender<StreamToken>,
    ) -> Result<String> {
        let start = std::time::Instant::now();
        
        let result = self.brain.think_stream(prompt, token_tx).await;
        
        let duration = start.elapsed().as_millis() as u64;
        self.metrics.update(duration, result.is_ok());
        
        result
    }
    
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        self.brain.embed(text).await
    }
    
    fn is_ready(&self) -> bool {
        self.brain.is_ready()
    }
    
    fn name(&self) -> &'static str {
        "${subagent^}Brain"
    }
    
    fn count_tokens(&self, text: &str) -> usize {
        self.brain.count_tokens(text)
    }
    
    fn get_batch_limit(&self) -> u32 {
        self.brain.get_batch_limit()
    }
}

#[async_trait]
impl SubagentBrain for ${subagent^}Brain {
    fn subagent_id(&self) -> &str {
        &self.config.id
    }
    
    fn get_config(&self) -> &SubagentConfig {
        &self.config
    }
    
    fn get_metrics(&self) -> SubagentMetrics {
        self.metrics.get()
    }
    
    fn update_metrics(&self, task_duration_ms: u64, success: bool) {
        self.metrics.update(task_duration_ms, success);
    }
    
    fn get_system_prompt(&self) -> &str {
        // The real brain handles its own system prompt
        ""
    }
    
    fn build_prompt(&self, task_type: &TaskType, prompt: &str, context: &TaskContext) -> String {
        format!("Task: {:?}\n\nContext: {}\n\nRequest: {}", 
            task_type, context.context_info, prompt)
    }
}

/// Factory for creating ${subagent^}Brain instances
pub struct ${subagent^}BrainFactory;

impl ${subagent^}BrainFactory {
    /// Create a new ${subagent^}Brain instance
    pub async fn create() -> Result<Arc<RwLock<Box<dyn SubagentBrain>>>> {
        let brain = ${subagent^}Brain::new().await?;
        Ok(Arc::new(RwLock::new(Box::new(brain))))
    }
    
    /// Create a real ${subagent} brain directly
    pub async fn create_real() -> Result<Real${subagent^}Brain> {
        Real${subagent^}Brain::new().await
    }
}
EOF
    
    echo "Updated $subagent lib.rs"
done

echo "All lib.rs files updated!"
