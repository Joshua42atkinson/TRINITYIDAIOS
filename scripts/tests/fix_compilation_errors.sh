#!/bin/bash

# Fix Trinity Compilation Errors - Test-Driven Approach
echo "🔧 Fixing Trinity Compilation Errors..."

cd /home/joshua/Workflow/desktop_trinity/trinity-genesis

echo "1. Fixing quest_system.rs string type errors..."
# Fix line 499 and similar patterns
sed -i 's/example_verbs: vec!\["explain", "describe", "summarize", "interpret", "compare".to_string()\],/example_verbs: vec!\["explain".to_string(), "describe".to_string(), "summarize".to_string(), "interpret".to_string(), "compare".to_string()\],/g' crates/trinity-kernel/src/quest_system.rs

# Fix line 500 and similar patterns
sed -i 's/assessment_suggestions: vec!\["Short answer", "Explanation", "Classification".to_string()\],/assessment_suggestions: vec!\["Short answer".to_string(), "Explanation".to_string(), "Classification".to_string()\],/g' crates/trinity-kernel/src/quest_system.rs

# Fix line 507 and similar patterns
sed -i 's/example_verbs: vec!\["apply", "implement", "use", "execute", "demonstrate".to_string()\],/example_verbs: vec!\["apply".to_string(), "implement".to_string(), "use".to_string(), "execute".to_string(), "demonstrate".to_string()\],/g' crates/trinity-kernel/src/quest_system.rs

# Fix line 508 and similar patterns
sed -i 's/assessment_suggestions: vec!\["Practical exercises", "Case studies", "Simulations".to_string()\],/assessment_suggestions: vec!\["Practical exercises".to_string(), "Case studies".to_string(), "Simulations".to_string()\],/g' crates/trinity-kernel/src/quest_system.rs

echo "2. Removing unused imports..."
sed -i '/use crate::brain::{Brain, GrammarSpec};/d' crates/trinity-kernel/src/trinity_orchestrator.rs

echo "3. Testing compilation..."
cd crates/trinity-kernel
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Compilation errors fixed!"
    echo "🧪 Running orchestrator test..."
    cargo run --bin test_trinity_orchestrator
else
    echo "❌ Still have compilation errors"
    echo "🔍 Running cargo check for details..."
    cargo check
fi

echo "📊 Compilation fix complete!"
