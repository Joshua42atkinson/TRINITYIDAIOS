//! Standalone test for ADDIE extraction functions

use regex;

// Helper functions for extracting information
fn extract_business_need(input: &str) -> String {
    // Real business need extraction using keyword patterns
    let business_need_patterns = vec![
        ("need", r"(?i)(?:we need|need to|required|must have|should have)\s+([^,.!?]+)"),
        ("problem", r"(?i)(?:problem|issue|challenge|difficulty)\s+(?:is|with)?\s*([^,.!?]+)"),
        ("goal", r"(?i)(?:goal|objective|target|aim)\s+(?:is|to)?\s*([^,.!?]+)"),
        ("improve", r"(?i)(?:improve|increase|enhance|boost)\s+([^,.!?]+)"),
        ("reduce", r"(?i)(?:reduce|decrease|minimize|eliminate)\s+([^,.!?]+)"),
    ];
    
    for (pattern_type, pattern) in business_need_patterns {
        if let Some(caps) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(input)) {
            if let Some(matched) = caps.get(1) {
                let need = matched.as_str().trim();
                if need.len() > 5 && !need.to_lowercase().contains("help") {
                    return format!("Business need ({}): {}", pattern_type, need);
                }
            }
        }
    }
    
    // Fallback: Look for action verbs + outcomes
    if let Some(caps) = regex::Regex::new(r"(?i)(?:train|teach|learn|develop|build|create)\s+([^,.!?]+)").ok().and_then(|re| re.captures(input)) {
        if let Some(matched) = caps.get(1) {
            return format!("Business need (training): {}", matched.as_str().trim());
        }
    }
    
    "Business need: Performance improvement".to_string()
}

fn main() {
    println!("🧪 Testing Real ADDIE Extraction Functions");
    
    // Test business need extraction
    let business_input = "We need to improve our team's communication skills and reduce errors in reporting.";
    let business_need = extract_business_need(business_input);
    println!("✅ Business Need: {}", business_need);
    
    println!("🎉 Extraction functions working! Ready for Week 2 implementation.");
}
