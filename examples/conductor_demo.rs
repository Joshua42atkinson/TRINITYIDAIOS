//! Demo of Conductor Agent orchestrating ADDIE workflow with dataset insights

use anyhow::Result;
use sqlx::PgPool;
use tracing_subscriber::fmt;
use trinity_kernel::{
    conductor_agent::{ConductorAgent, ConductorInterface, ProjectContext, AddiePhase},
    dataset_query::DatasetQueryImpl,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    fmt()
        .with_env_filter("info")
        .init();

    // Connect to database
    let db_url = "postgresql://trinity:6226@localhost:5432/trinity";
    let db_pool = PgPool::connect(db_url).await?;
    
    // Create dataset query interface
    let dataset_query = DatasetQueryImpl::new(db_pool.clone());
    
    // Create cross-search (placeholder for now)
    let embedding_model = std::sync::Arc::new(
        trinity_mcp_server::embeddings::SentenceTransformerModel::from_huggingface("all-MiniLM-L6-v2").await?
    );
    let cross_search = trinity_mcp_server::cross_dataset::OptimizedCrossSearch::new(
        embedding_model,
        db_pool,
    );
    
    // Create conductor agent
    let mut conductor = ConductorAgent::new(dataset_query, cross_search);
    
    // Define project context
    let project = ProjectContext {
        title: "Introduction to Machine Learning".to_string(),
        target_audience: "Undergraduate students with basic programming knowledge".to_string(),
        learning_goals: vec![
            "Understand ML fundamentals".to_string(),
            "Apply common algorithms".to_string(),
            "Evaluate model performance".to_string(),
        ],
        constraints: vec![
            "8-week timeline".to_string(),
            "No prior ML experience required".to_string(),
        ],
        timeline: Some("8 weeks".to_string()),
        resources: vec![
            "Python notebooks".to_string(),
            "Cloud computing credits".to_string(),
        ],
    };
    
    println!("=== Conductor Agent ADDIE Demo ===\n");
    
    // Start project
    println!("1. Starting project...");
    conductor.start_project(project).await?;
    
    // Show analysis results
    println!("\n2. Analysis Phase Results:");
    let analysis_insights = conductor.get_phase_insights(AddiePhase::Analysis);
    println!("   Insights gathered: {}", analysis_insights.len());
    for insight in analysis_insights.iter().take(3) {
        println!("   - {}", insight.insight);
    }
    
    let analysis_decisions = conductor.get_phase_decisions(AddiePhase::Analysis);
    for decision in analysis_decisions {
        println!("   Decision: {} (confidence: {:.2})", decision.decision, decision.confidence);
    }
    
    // Advance to design phase
    println!("\n3. Advancing to Design Phase...");
    conductor.advance_phase().await?;
    
    let design_decisions = conductor.get_phase_decisions(AddiePhase::Design);
    for decision in design_decisions {
        println!("   Decision: {} (confidence: {:.2})", decision.decision, decision.confidence);
    }
    
    // Advance to development phase
    println!("\n4. Advancing to Development Phase...");
    conductor.advance_phase().await?;
    
    let development_insights = conductor.get_phase_insights(AddiePhase::Development);
    println!("   UI insights gathered: {}", development_insights.len());
    for insight in development_insights.iter().take(3) {
        println!("   - {}", insight.insight);
    }
    
    // Advance to implementation phase
    println!("\n5. Advancing to Implementation Phase...");
    conductor.advance_phase().await?;
    
    // Advance to evaluation phase
    println!("\n6. Advancing to Evaluation Phase...");
    conductor.advance_phase().await?;
    
    // Generate final report
    println!("\n7. Generating Workflow Report...");
    let report = conductor.generate_report();
    
    println!("\n=== Workflow Summary ===");
    println!("{}", report.summary);
    println!("\nTotal Decisions: {}", report.decisions.len());
    println!("Total Insights: {}", report.insights.len());
    
    println!("\n=== Phase Breakdown ===");
    for phase in [AddiePhase::Analysis, AddiePhase::Design, AddiePhase::Development, 
                   AddiePhase::Implementation, AddiePhase::Evaluation] {
        let decisions = report.decisions.iter()
            .filter(|d| matches!(d.phase, phase))
            .count();
        let insights = report.insights.iter()
            .filter(|i| matches!(i.phase, phase))
            .count();
        
        println!("{:?}: {} decisions, {} insights", phase, decisions, insights);
    }
    
    // Save report to file
    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("conductor_report.json", report_json)?;
    println!("\nReport saved to: conductor_report.json");
    
    println!("\n=== Demo Complete ===");
    Ok(())
}
