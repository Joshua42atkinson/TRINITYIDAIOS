//! Demonstration of Trinity's new tool-aware capabilities

use anyhow::Result;
use trinity_kernel::{
    production_brain::ProductionBrain,
    config::ModelConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the enhanced ProductionBrain
    println!("🚀 Initializing Trinity with Tool Awareness...");
    
    let brain = ProductionBrain::new_97b_optimized().await?;
    
    // Check if brain is ready
    if !brain.health_check().await? {
        println!("❌ Brain not ready");
        return Ok(());
    }
    
    println!("✅ Trinity 97B Model is ready and aware of its tools!\n");
    
    // Example 1: User asks about something already built
    println!("📝 Example 1: User asks 'Build asset generation pipeline'");
    let response = brain.think("Build asset generation pipeline").await?;
    println!("Trinity Response:\n{}\n", response);
    
    // Example 2: User asks for status
    println!("📝 Example 2: User asks 'What's the current project status?'");
    let response = brain.think("What's the current project status?").await?;
    println!("Trinity Response:\n{}\n", response);
    
    // Example 3: User asks to fix something
    println!("📝 Example 3: User asks 'Fix the compilation errors'");
    let response = brain.think("Fix the compilation errors in trinity-kernel").await?;
    println!("Trinity Response:\n{}\n", response);
    
    println!("✅ Demonstration complete!");
    println!("Trinity now knows about:");
    println!("  - MCP Server at localhost:8080");
    println!("  - Documentation database");
    println!("  - Implementation status tracking");
    println!("  - Self-work workflows");
    println!("  - When to use each tool!");
    
    Ok(())
}
