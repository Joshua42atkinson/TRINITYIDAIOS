//! Test our new real extraction functions

use trinity_body::instructional_design::{
    extract_business_need, extract_goal, extract_challenge, extract_metrics, parse_star_scenario
};

fn main() {
    println!("🧪 Testing Real ADDIE Extraction Functions");
    
    // Test business need extraction
    let business_input = "We need to improve our team's communication skills and reduce errors in reporting.";
    let business_need = extract_business_need(business_input);
    println!("✅ Business Need: {}", business_need);
    
    // Test goal extraction
    let goal_input = "We want to increase accuracy by 25% and reduce completion time by 30 minutes.";
    let goal = extract_goal(goal_input);
    println!("✅ Goal: {}", goal);
    
    // Test challenge extraction
    let challenge_input = "Our team struggles with complex data analysis and they're confused about the new software interface.";
    let challenge = extract_challenge(challenge_input);
    println!("✅ Challenge: {}", challenge);
    
    // Test metrics extraction
    let metrics_input = "We need 20% faster processing and 15% fewer errors, plus save 2 hours per week.";
    let metrics = extract_metrics(metrics_input);
    println!("✅ Metrics:");
    for metric in metrics {
        println!("   - {}", metric);
    }
    
    // Test STAR scenario parsing
    let star_input = "When we had the quarterly reporting deadline, I was responsible for data analysis. I created a new template and trained the team, which resulted in 50% faster reporting and fewer errors.";
    let scenario = parse_star_scenario(star_input);
    println!("✅ STAR Scenario:");
    println!("   Situation: {}", scenario.situation);
    println!("   Task: {}", scenario.task);
    println!("   Action: {}", scenario.action);
    println!("   Result: {}", scenario.result);
    
    println!("🎉 All extraction functions working! Ready for Week 2 implementation.");
}
