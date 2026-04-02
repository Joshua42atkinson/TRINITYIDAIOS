# Conductor Agent Guide

## Overview
The Conductor Agent orchestrates the ADDIE (Analysis, Design, Development, Implementation, Evaluation) workflow using insights from educational datasets.

## Architecture

### Core Components
1. **ConductorAgent** - Main orchestrator
2. **WorkflowState** - Tracks project progress
3. **DatasetQuery** - Extracts insights from datasets
4. **CrossDatasetSearch** - Unified semantic search

### ADDIE Phases
1. **Analysis** - SME insights, learning objectives
2. **Design** - Curriculum structure, assessment strategy
3. **Development** - UI/UX recommendations, content creation
4. **Implementation** - Deployment strategies, rollout
5. **Evaluation** - Outcome measurement, improvement

## Usage Examples

### Basic Workflow
```rust
use trinity_kernel::{
    conductor_agent::{ConductorAgent, ProjectContext},
    dataset_query::DatasetQueryImpl,
};

// Create agent
let dataset_query = DatasetQueryImpl::new(db_pool);
let cross_search = OptimizedCrossSearch::new(embedding_model, db_pool);
let mut conductor = ConductorAgent::new(dataset_query, cross_search);

// Define project
let project = ProjectContext {
    title: "Data Science Fundamentals".to_string(),
    target_audience: "Business professionals".to_string(),
    learning_goals: vec![
        "Understand data analysis".to_string(),
        "Create visualizations".to_string(),
        "Make data-driven decisions".to_string(),
    ],
    constraints: vec!["6 weeks duration".to_string()],
    timeline: Some("6 weeks".to_string()),
    resources: vec!["Tableau".to_string(), "Python".to_string()],
};

// Start workflow
conductor.start_project(project).await?;

// Advance phases
conductor.advance_phase().await?; // Analysis → Design
conductor.advance_phase().await?; // Design → Development
conductor.advance_phase().await?; // Development → Implementation
conductor.advance_phase().await?; // Implementation → Evaluation
```

### Phase-Specific Operations
```rust
// Get analysis insights
let insights = conductor.get_phase_insights(AddiePhase::Analysis);
for insight in insights {
    println!("Analysis: {} (relevance: {:.2})", insight.insight, insight.relevance_score);
}

// Get design decisions
let decisions = conductor.get_phase_decisions(AddiePhase::Design);
for decision in decisions {
    println!("Decision: {} (confidence: {:.2})", decision.decision, decision.confidence);
}
```

### Generate Reports
```rust
let report = conductor.generate_report();

println!("Project: {}", report.project.title);
println!("Current Phase: {:?}", report.current_phase);
println!("Summary: {}", report.summary);

// Save report
let json = serde_json::to_string_pretty(&report)?;
std::fs::write("project_report.json", json)?;
```

## Data Sources

### Edu-ConvoKit
- **Used in**: Analysis, Implementation phases
- **Provides**: SME insights, teaching strategies
- **Query types**: "expert opinions", "teaching methods"

### Blooms Taxonomy
- **Used in**: Design, Evaluation phases
- **Provides**: Learning objectives, assessment methods
- **Query types**: "learning objectives", "assessment design"

### RICO UI/UX
- **Used in**: Development phase
- **Provides**: Interface best practices, accessibility guidelines
- **Query types**: "educational interface", "accessible design"

## Decision Making Process

### 1. Data Collection
```rust
// SME insights for analysis
let sme_insights = dataset_query.get_sme_insights(topic, Some(5)).await?;

// Learning objectives for design
let objectives = dataset_query.get_learning_objectives(goal, Some(levels)).await?;

// UI recommendations for development
let ui_recs = dataset_query.get_ui_recommendations(app_type, Some(accessibility)).await?;
```

### 2. Insight Synthesis
- Collect relevant insights from datasets
- Rank by relevance/confidence scores
- Identify patterns and recommendations

### 3. Decision Generation
- Synthesize insights into actionable decisions
- Include rationale and confidence scores
- Track data sources for transparency

## Customization

### Custom Project Context
```rust
let project = ProjectContext {
    title: "Custom Course".to_string(),
    target_audience: "Specific audience".to_string(),
    learning_goals: vec![
        "Goal 1".to_string(),
        "Goal 2".to_string(),
    ],
    constraints: vec![
        "Budget limit".to_string(),
        "Time constraint".to_string(),
    ],
    timeline: Some("Custom timeline".to_string()),
    resources: vec!["Resource 1".to_string()],
};
```

### Custom Query Logic
```rust
impl ConductorAgent {
    async fn custom_analysis(&mut self) -> Result<()> {
        // Custom analysis logic
        let custom_insights = self.dataset_query
            .search_conversations("custom query", Some(10), None)
            .await?;
        
        // Process insights
        for insight in custom_insights {
            // Custom processing
        }
        
        Ok(())
    }
}
```

## Best Practices

### 1. Clear Project Definition
- Specific learning goals
- Well-defined audience
- Realistic constraints

### 2. Iterative Approach
- Review insights at each phase
- Adjust decisions based on data
- Document rationale

### 3. Data-Driven Decisions
- Trust high-confidence insights
- Consider multiple data sources
- Validate with human expertise

### 4. Progress Tracking
- Monitor phase transitions
- Track decision quality
- Collect feedback

## Integration Examples

### With LLM Generation
```rust
// Use insights to prompt LLM
let insights = conductor.get_phase_insights(AddiePhase::Design);
let prompt = format!(
    "Generate course structure based on these insights: {}",
    insights.iter().map(|i| &i.insight).collect::<Vec<_>>().join("\n")
);

let llm_response = llm.generate(&prompt).await?;
```

### With External Tools
```rust
// Export decisions to project management
let decisions = conductor.get_phase_decisions(AddiePhase::Design);
for decision in decisions {
    project_tool.create_task(
        &decision.decision,
        &decision.rationale,
        decision.confidence
    ).await?;
}
```

## Monitoring

### Key Metrics
1. **Insight Quality**: Relevance scores, source diversity
2. **Decision Confidence**: Average confidence per phase
3. **Data Source Usage**: Which datasets contribute most
4. **Phase Duration**: Time spent in each ADDIE phase

### Logging
```rust
use tracing::{info, warn};

// Log important decisions
for decision in &conductor.get_workflow_state().decisions {
    info!(
        phase = ?decision.phase,
        confidence = decision.confidence,
        "Decision: {}",
        decision.decision
    );
}
```

## Troubleshooting

### Low Confidence Decisions
- Check data availability
- Verify query relevance
- Consider additional data sources

### Missing Insights
- Ensure datasets are populated
- Check query terms
- Verify database connectivity

### Phase Transition Issues
- Complete current phase requirements
- Check decision completeness
- Validate workflow state

## Future Enhancements

1. **Adaptive Workflows**: Custom phase sequences
2. **Multi-Agent Coordination**: Specialized agents per phase
3. **Real-time Adaptation**: Dynamic adjustment based on feedback
4. **Collaborative Features**: Multiple conductor instances
5. **Template Library**: Pre-defined project templates
