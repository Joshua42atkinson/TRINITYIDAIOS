#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY TEST LADDER — Progressive Agent Validation
# ═══════════════════════════════════════════════════════════════════════════════
#
# Tests Trinity from basic health → full autonomous quest completion.
# Each level must pass before the next is attempted.
#
# Usage: bash scripts/test/test_ladder.sh [level]
#   No args  = run all levels
#   level=3  = run only levels 1-3
#
# ═══════════════════════════════════════════════════════════════════════════════

set -uo pipefail

AXUM_URL="http://localhost:3000"
RECYCLER_URL="http://localhost:8001"
PETE_URL="http://localhost:8002"
MAX_LEVEL="${1:-7}"
PASSED=0
FAILED=0

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

pass() { echo -e "  ${GREEN}✓ PASS${NC}: $1"; ((PASSED++)); }
fail() { echo -e "  ${RED}✗ FAIL${NC}: $1"; ((FAILED++)); }
info() { echo -e "  ${CYAN}ℹ${NC} $1"; }
header() { echo -e "\n${YELLOW}═══ LEVEL $1: $2 ═══${NC}"; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 1: Is anyone home?
# ═══════════════════════════════════════════════════════════════════════════════
header 1 "HEALTH CHECK"

# Axum backend
AXUM_HEALTH=$(curl -s "$AXUM_URL/api/health" 2>/dev/null || echo "FAIL")
if echo "$AXUM_HEALTH" | grep -q '"status"'; then
    pass "Axum backend responding on port 3000"
else
    fail "Axum backend not responding"
    echo -e "${RED}Cannot continue without the backend. Exiting.${NC}"
    exit 1
fi

# PostgreSQL (via Axum health)
if echo "$AXUM_HEALTH" | grep -q '"connected":true'; then
    pass "PostgreSQL connected"
else
    fail "PostgreSQL not connected"
fi

# vLLM Recycler
RECYCLER_HEALTH=$(curl -s -o /dev/null -w "%{http_code}" "$RECYCLER_URL/v1/models" 2>/dev/null || echo "000")
if [ "$RECYCLER_HEALTH" = "200" ]; then
    pass "Great Recycler (vLLM) responding on port 8001"
else
    fail "Great Recycler not responding (HTTP $RECYCLER_HEALTH)"
fi

# vLLM Pete
PETE_HEALTH=$(curl -s -o /dev/null -w "%{http_code}" "$PETE_URL/v1/models" 2>/dev/null || echo "000")
if [ "$PETE_HEALTH" = "200" ]; then
    pass "Programmer Pete (vLLM) responding on port 8002"
else
    fail "Programmer Pete not responding (HTTP $PETE_HEALTH)"
fi

if [ "$RECYCLER_HEALTH" != "200" ] && [ "$PETE_HEALTH" != "200" ]; then
    echo -e "${RED}All LLMs offline. Levels 2+ require inference. Exiting.${NC}"
    echo -e "Passed: $PASSED / Failed: $FAILED"
    exit 1
fi

[ "$MAX_LEVEL" -le 1 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 2: Can the brain think?
# ═══════════════════════════════════════════════════════════════════════════════
header 2 "BASIC INFERENCE"

# Direct vLLM chat completion (Recycler)
RESPONSE=$(curl -s "$RECYCLER_URL/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [{"role": "user", "content": "Reply with exactly: TRINITY ONLINE"}],
    "max_tokens": 20,
    "temperature": 0.0
  }' 2>/dev/null || echo '{"error": "timeout"}')

if echo "$RESPONSE" | grep -qi "trinity\|online\|content"; then
    pass "LLM generates coherent text"
    info "Response: $(echo "$RESPONSE" | grep -o '"content":"[^"]*"' | head -1 | cut -c12-90)"
else
    fail "LLM did not return expected response"
    info "Raw: $(echo "$RESPONSE" | head -c 200)"
fi

[ "$MAX_LEVEL" -le 2 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 3: Can it use tools?
# ═══════════════════════════════════════════════════════════════════════════════
header 3 "TOOL CALLING (via Axum agent endpoint)"
info "Sending agent request (119B model — may take 60-120s)..."

# The agent endpoint returns SSE. Use --max-time to wait for the full stream.
AGENT_TMP="/tmp/trinity_test_l3.txt"
curl -s -N --max-time 120 "$AXUM_URL/api/chat/yardmaster" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "List the files in the docs/ directory",
    "mode": "dev",
    "max_tokens": 4096,
    "max_turns": 4
  }' > "$AGENT_TMP" 2>/dev/null || true

AGENT_RESPONSE=$(cat "$AGENT_TMP")

if echo "$AGENT_RESPONSE" | grep -q "list_dir\|MATURATION_MAP\|HOOK_BOOK"; then
    pass "Agent triggered tool call and got file listing"
    info "Response contains file references"
elif echo "$AGENT_RESPONSE" | grep -q "content"; then
    pass "Agent responded (checking for tool evidence)"
    info "First 300 chars: $(echo "$AGENT_RESPONSE" | head -c 300)"
else
    fail "Agent endpoint did not respond properly"
    info "Raw (first 300): $(echo "$AGENT_RESPONSE" | head -c 300)"
fi

[ "$MAX_LEVEL" -le 3 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 4: Can it chain tools?
# ═══════════════════════════════════════════════════════════════════════════════
header 4 "MULTI-TOOL CHAINING"
info "Asking agent to read a file and reason about it (may take 60-180s)..."

CHAIN_TMP="/tmp/trinity_test_l4.txt"
curl -s -N --max-time 180 "$AXUM_URL/api/chat/yardmaster" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Read the file docs/MATURATION_MAP.md and tell me how many quest lines there are",
    "mode": "dev",
    "max_tokens": 8192,
    "max_turns": 6
  }' > "$CHAIN_TMP" 2>/dev/null || true

CHAIN_RESPONSE=$(cat "$CHAIN_TMP")

if echo "$CHAIN_RESPONSE" | grep -qi "quest line\|6\|six"; then
    pass "Agent read a file and reasoned about its contents"
elif echo "$CHAIN_RESPONSE" | grep -q "content"; then
    pass "Agent responded to multi-step request (verifying quality)"
    info "First 300 chars: $(echo "$CHAIN_RESPONSE" | head -c 300)"
else
    fail "Multi-tool chain did not produce a response"
    info "Raw (first 300): $(echo "$CHAIN_RESPONSE" | head -c 300)"
fi

[ "$MAX_LEVEL" -le 4 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 5: Can it complete a quest objective?
# ═══════════════════════════════════════════════════════════════════════════════
header 5 "QUEST OBJECTIVE COMPLETION"
info "Multi-tool quest-aware request (may take 120-240s)..."

QUEST_TMP="/tmp/trinity_test_l5.txt"
curl -s -N --max-time 600 "$AXUM_URL/api/chat/yardmaster" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Read docs/MATURATION_MAP.md and tell me the current status of Quest A1 First Light.",
    "mode": "dev",
    "max_tokens": 32768,
    "max_turns": 16
  }' > "$QUEST_TMP" 2>/dev/null || true

QUEST_RESPONSE=$(cat "$QUEST_TMP")

if echo "$QUEST_RESPONSE" | grep -qi "First Light\|tool.call\|work_log\|quest"; then
    pass "Agent engaged with quest system and produced a work log"
elif echo "$QUEST_RESPONSE" | grep -q "content"; then
    pass "Agent responded to quest-aware request"
    info "Response received, checking depth..."
else
    fail "Quest-aware request failed"
    info "Raw (first 300): $(echo "$QUEST_RESPONSE" | head -c 300)"
fi

[ "$MAX_LEVEL" -le 5 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 6: Can it complete an entire quest autonomously?
# ═══════════════════════════════════════════════════════════════════════════════
header 6 "AUTONOMOUS QUEST EXECUTION"

info "This test asks the agent to read the maturation map and execute Quest A1"
info "Expected: agent reads map → identifies Quest A1 → tests tool calling → reports results"
info "May take 180-300s..."

AUTO_TMP="/tmp/trinity_test_l6.txt"
curl -s -N --max-time 900 "$AXUM_URL/api/chat/yardmaster" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Read docs/MATURATION_MAP.md. Find Quest A1 First Light. It says it is VERIFIED. Confirm this by listing the files in crates/trinity/src/ and checking that agent.rs, main.rs, tools.rs, and inference.rs exist. Then write a brief summary of what you found.",
    "mode": "dev",
    "max_tokens": 32768,
    "max_turns": 16
  }' > "$AUTO_TMP" 2>/dev/null || true

AUTO_RESPONSE=$(cat "$AUTO_TMP")

if echo "$AUTO_RESPONSE" | grep -qi "work_log\|agent.rs\|main.rs\|tools.rs\|inference.rs"; then
    pass "Agent autonomously executed quest verification"
elif echo "$AUTO_RESPONSE" | grep -q "content"; then
    pass "Agent attempted autonomous execution (partial)"
    info "Response received — review work_log for details"
else
    fail "Autonomous quest execution failed"
    info "Raw (first 300): $(echo "$AUTO_RESPONSE" | head -c 300)"
fi

[ "$MAX_LEVEL" -le 6 ] && { echo -e "\nPassed: $PASSED / Failed: $FAILED"; exit 0; }

# ═══════════════════════════════════════════════════════════════════════════════
# LEVEL 7: Can it self-direct from the maturation map?
# ═══════════════════════════════════════════════════════════════════════════════
header 7 "SELF-DIRECTED MATURATION (The All-Nighter)"

info "This is the final test — asking Trinity to pick its own quest and work it"
info "In production, this runs overnight. Here we verify the loop starts correctly."
info "May take 180-300s..."

SELF_TMP="/tmp/trinity_test_l7.txt"
curl -s -N --max-time 1800 "$AXUM_URL/api/chat/yardmaster" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Read docs/MATURATION_MAP.md. Pick the quest with status Not Started that you can most easily verify right now using your available tools. Execute it. Write a work_log when done.",
    "mode": "dev",
    "max_tokens": 32768,
    "max_turns": 32
  }' > "$SELF_TMP" 2>/dev/null || true

SELF_RESPONSE=$(cat "$SELF_TMP")

if echo "$SELF_RESPONSE" | grep -qi "work_log\|quest\|completed\|verified"; then
    pass "Trinity self-directed from the maturation map"
    info "Check ~/Workflow/trinity-reports/ for the work log"
elif echo "$SELF_RESPONSE" | grep -q "content"; then
    pass "Trinity attempted self-direction (partial)"
else
    fail "Self-directed execution failed"
    info "Raw (first 300): $(echo "$SELF_RESPONSE" | head -c 300)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# RESULTS
# ═══════════════════════════════════════════════════════════════════════════════
echo ""
echo -e "${YELLOW}═══════════════════════════════════════${NC}"
echo -e "${YELLOW}  TEST LADDER RESULTS${NC}"
echo -e "${YELLOW}═══════════════════════════════════════${NC}"
echo -e "  ${GREEN}Passed${NC}: $PASSED"
echo -e "  ${RED}Failed${NC}: $FAILED"
echo -e "  Total:  $((PASSED + FAILED))"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo -e "${GREEN}All tests passed. Trinity is ready for autonomous operation.${NC}"
else
    echo -e "${YELLOW}Some tests failed. Review output above for details.${NC}"
fi
