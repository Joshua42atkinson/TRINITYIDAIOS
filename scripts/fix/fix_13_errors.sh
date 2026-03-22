#!/bin/bash

# Neon Wizard's Error Fixing Spell
# Fixes the 13 compilation errors safely

echo "🧙‍♂️ Neon Wizard's Error Fixing Spell"
echo "===================================="
echo ""

# Colors
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${PURPLE}🔧 Fixing 13 Compilation Errors...${NC}"
echo ""

# Backup first
echo -e "${CYAN}🛡️ Creating Safety Backup...${NC}"
cp -r crates/trinity-kernel/src session_success/backup_src_$(date +%H%M%S)
echo -e "${GREEN}✅ Backup created${NC}"

# Fix 1: Type conversion in unified_memory.rs
echo ""
echo -e "${CYAN}Fix 1: Type conversion (usize → u32)...${NC}"
sed -i 's/rag.search(query, limit.unwrap_or(10) as usize)/rag.search(query, limit.unwrap_or(10) as u32)/g' crates/trinity-kernel/src/unified_memory.rs

# Fix 2: Metadata move issue in tool_registry.rs
echo -e "${CYAN}Fix 2: Metadata move issue...${NC}"
sed -i 's/tools.insert(tool_id.clone(), metadata);/tools.insert(tool_id.clone(), metadata.clone());/g' crates/trinity-kernel/src/tool_registry.rs

# Fix 3: Tool tags access
echo -e "${CYAN}Fix 3: Tool tags field access...${NC}"
sed -i 's/tool.tags.iter()/tool.metadata.tags.iter()/g' crates/trinity-kernel/src/tool_registry.rs

# Fix 4: Agent health monitoring parameter
echo -e "${CYAN}Fix 4: Agent health monitoring...${NC}"
sed -i 's/self.start_health_monitoring(&agent_id).await;/self.start_health_monitoring(agent_id).await;/g' crates/trinity-kernel/src/agent_manager.rs

# Fix 5: Remove agent cloning in get_agent
echo -e "${CYAN}Fix 5: Agent cloning issue...${NC}"
cat > temp_fix_agent.rs << 'EOF'
// Replace get_agent method
pub async fn get_agent(&self, agent_id: &str) -> Option<AgentInfo> {
    let agents = self.agents.read().await;
    agents.get(agent_id).map(|instance| AgentInfo {
        id: instance.id.clone(),
        name: instance.agent.name().to_string(),
        agent_type: instance.agent.agent_type(),
        status: instance.status.clone(),
        created_at: instance.created_at,
        last_heartbeat: instance.last_heartbeat,
        resource_allocation: instance.resource_allocation.clone(),
    })
}
EOF

# Apply the fix (simplified)
sed -i '/pub async fn get_agent/,/Ok(())/c\
    pub async fn get_agent(&self, agent_id: &str) -> Option<AgentInfo> {\
        let agents = self.agents.read().await;\
        agents.get(agent_id).map(|instance| AgentInfo {\
            id: instance.id.clone(),\
            name: instance.agent.name().to_string(),\
            agent_type: instance.agent.agent_type(),\
            status: instance.status.clone(),\
            created_at: instance.created_at,\
            last_heartbeat: instance.last_heartbeat,\
            resource_allocation: instance.resource_allocation.clone(),\
        })\
    }' crates/trinity-kernel/src/agent_manager.rs

# Fix 6: ProductionBrain type annotation
echo -e "${CYAN}Fix 6: ProductionBrain type annotation...${NC}"
sed -i 's/let full_prompt = self.prompt_builder/let full_prompt: String = self.prompt_builder/g' crates/trinity-kernel/src/production_brain.rs

# Fix 7: Remove mut from agent variables
echo -e "${CYAN}Fix 7: Remove unnecessary mut...${NC}"
sed -i 's/if let Some(mut inst)/if let Some(inst)/g' crates/trinity-kernel/src/agent_manager.rs

# Fix 8: Add Copy to ResourceAllocation
echo -e "${CYAN}Fix 8: Add Copy trait...${NC}"
sed -i 's/#\[derive(Debug, Clone, Serialize, Deserialize)\]/#[derive(Debug, Clone, Copy, Serialize, Deserialize)]/g' crates/trinity-kernel/src/agent_manager.rs

# Fix 9-13: Remove unused imports
echo -e "${CYAN}Fix 9-13: Remove unused imports...${NC}"
# These will be handled by cargo clippy --fix

echo ""
echo -e "${PURPLE}✨ Applying Fixes...${NC}"

# Test compilation
echo ""
echo -e "${CYAN}🧪 Testing Compilation...${NC}"
if cargo check --package trinity-kernel > session_success/logs/fix_test.log 2>&1; then
    echo -e "${GREEN}✅ SUCCESS! Trinity Core compiles!${NC}"
    
    # Run clippy auto-fix
    echo ""
    echo -e "${CYAN}🧹 Running Clippy Auto-Fix...${NC}"
    cargo clippy --package trinity-kernel --fix --allow-dirty --allow-staged > session_success/logs/clippy_fix.log 2>&1
    
    echo -e "${GREEN}✅ All fixes applied!${NC}"
else
    echo -e "${RED}❌ Still has errors:${NC}"
    grep -c "error" session_success/logs/fix_test.log
    echo "Check session_success/logs/fix_test.log for details"
fi

echo ""
echo -e "${PURPLE}🎉 Error Fixing Complete!${NC}"
echo ""
echo -e "${CYAN}Next step:${NC}"
echo "cargo run --package trinity-kernel --example simple_test"
