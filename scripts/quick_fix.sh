#!/bin/bash

# Quick Fix Script for Trinity Core Compilation Errors

echo "🔧 Trinity Core - Quick Fixes"
echo "============================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "📋 Fixing critical compilation errors..."
echo ""

# Fix 1: Remove classifier from TaskRouter
echo -e "${YELLOW}1. Removing TaskClassifier references...${NC}"
sed -i 's|// classifier: TaskClassifier,||g' crates/trinity-kernel/src/task_router.rs
sed -i '/classifier: TaskClassifier::new(), \/\/ TODO/d' crates/trinity-kernel/src/task_router.rs
echo -e "${GREEN}✅ TaskRouter classifier removed${NC}"

# Fix 2: Fix borrow in UnifiedMemory
echo -e "${YELLOW}2. Fixing UnifiedMemory borrow issue...${NC}"
sed -i 's|match (state.preferred_complexity, &result.complexity) {|match (state.preferred_complexity.clone(), &result.complexity) {|g' crates/trinity-kernel/src/unified_memory.rs
echo -e "${GREEN}✅ UnifiedMemory borrow fixed${NC}"

# Fix 3: Add explicit type to ProductionBrain
echo -e "${YELLOW}3. Adding explicit type to ProductionBrain...${NC}"
sed -i 's|let full_prompt = self.prompt_builder.build_memory_aware_prompt(prompt).await?;|let full_prompt: String = self.prompt_builder.build_memory_aware_prompt(prompt).await?;|g' crates/trinity-kernel/src/production_brain.rs
echo -e "${GREEN}✅ ProductionBrain type fixed${NC}"

# Fix 4: Remove unused variable prefixes
echo -e "${YELLOW}4. Fixing unused variables...${NC}"

# Fix communication_bus.rs
sed -i 's|let (tx, mut rx) = mpsc::channel(1);|let (_tx, mut rx) = mpsc::channel(1);|g' crates/trinity-kernel/src/communication_bus.rs
sed -i 's|let mut temp_sub = self.subscribe|let _temp_sub = self.subscribe|g' crates/trinity-kernel/src/communication_bus.rs

# Fix workflow_recorder.rs
sed -i 's|template_id: Option<String>|_template_id: Option<String>|g' crates/trinity-kernel/src/workflow_recorder.rs
sed -i 's|let active = self.active_workflows|let _active = self.active_workflows|g' crates/trinity-kernel/src/workflow_recorder.rs
sed -i 's|let workflow_data = self.export_workflow_data|let _workflow_data = self.export_workflow_data|g' crates/trinity-kernel/src/workflow_recorder.rs

# Fix other files
sed -i 's|language } =>|language: _ } =>|g' crates/trinity-kernel/src/subagent_brain.rs
sed -i 's|let current_pid = process::id();|let _current_pid = process::id();|g' crates/trinity-kernel/src/system_reaper.rs
sed -i 's|agent_id: String) {|_agent_id: String) {|g' crates/trinity-kernel/src/agent_manager.rs
sed -i 's|tool: Box<dyn MCPTool>) {|_tool: Box<dyn MCPTool>) {|g' crates/trinity-kernel/src/trinity_mcp_server.rs
sed -i 's|params: serde_json::Value) -> Result|_params: serde_json::Value) -> Result|g' crates/trinity-kernel/src/trinity_mcp_server.rs

echo -e "${GREEN}✅ Unused variables fixed${NC}"

echo ""
echo "🧪 Testing compilation..."
echo ""

# Test compilation
if cargo check --package trinity-kernel > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Trinity Core compiles successfully!${NC}"
else
    echo -e "${RED}❌ Still has compilation errors:${NC}"
    cargo check --package trinity-kernel 2>&1 | grep "error" | head -10
fi

echo ""
echo "🧹 Running clippy for auto-fixable issues..."
echo ""

# Auto-fix clippy issues
cargo clippy --package trinity-kernel --fix --allow-dirty --allow-staged 2>/dev/null

echo ""
echo "📊 Final status:"
echo "==============="
echo "Run these commands to verify:"
echo "1. cargo check --package trinity-kernel"
echo "2. cargo clippy --package trinity-kernel"
echo "3. cargo test --package trinity-kernel"
echo ""
echo "If still errors remain, check TODO_TRINITY_CORE.md for manual fixes"
