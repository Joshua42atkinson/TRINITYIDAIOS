//! Test Trinity's Memory-Aware Capabilities

use anyhow::Result;
use trinity_kernel::{
    production_brain::ProductionBrain,
    trinity_mcp_client::McpClient,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Testing Trinity Memory-Aware Responses...\n");
    
    // Initialize the enhanced ProductionBrain
    let brain = ProductionBrain::new_97b_optimized().await?;
    
    // Test scenarios
    let test_cases = vec![
        ("Build MCP server", "Should report it's already built"),
        ("Fix compilation errors", "Should use existing solutions"),
        ("Create ADDIE workflow", "Should report it's implemented"),
        ("Optimize performance", "Should check current status"),
    ];
    
    for (i, (request, expected)) in test_cases.iter().enumerate() {
        println!("📝 Test Case {}: {}", i + 1, request);
        println!("   Expected: {}", expected);
        
        // Use memory-aware thinking
        let response = brain.think_with_memory(request).await?;
        
        println!("   Response: {}\n", response);
        
        // Check if response mentions memory checks
        let memory_indicators = vec![
            "implementation status",
            "already built",
            "already implemented",
            "documentation shows",
            "existing solution",
        ];
        
        let found_memory_check = memory_indicators.iter()
            .any(|indicator| response.to_lowercase().contains(indicator));
            
        if found_memory_check {
            println!("   ✅ Memory check detected in response");
        } else {
            println!("   ⚠️ No obvious memory check in response");
        }
        
        println!("   {}\n", "─".repeat(50));
    }
    
    println!("✅ Memory-aware testing complete!");
    Ok(())
}
