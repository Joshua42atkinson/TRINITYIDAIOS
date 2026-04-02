#!/bin/bash
# Trinity Workspace Analysis Tool
# Assesses code organization, documentation, and compilation status

echo "🔍 TRINITY WORKSPACE ANALYSIS"
echo "=============================="
echo ""

# 1. Code Statistics
echo "📊 CODE STATISTICS"
echo "------------------"
echo "Total Rust files: $(find crates -name "*.rs" | wc -l)"
echo "Total lines of code: $(find crates -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "Total crates: $(ls -d crates/*/ | wc -l)"
echo ""

# 2. Documentation Status
echo "📚 DOCUMENTATION STATUS"
echo "----------------------"
echo "Markdown files:"
find crates -name "*.md" | while read file; do
    echo "  - $file ($(wc -l < $file) lines)"
done
echo ""

# 3. Compilation Status
echo "🔨 COMPILATION STATUS"
echo "--------------------"
echo "Checking core components..."
components=("trinity-core" "trinity-body" "trinity-brain" "trinity-protocol")
for comp in "${components[@]}"; do
    echo -n "$comp: "
    if cargo check -p $comp 2>/dev/null | grep -q "Finished"; then
        echo "✅ Compiles"
    else
        echo "❌ Errors"
    fi
done

echo ""
echo "Checking subagents..."
for agent in crates/trinity-agent-*; do
    if [ -d "$agent" ]; then
        agent_name=$(basename "$agent")
        echo -n "$agent_name: "
        if cargo check -p $agent_name 2>/dev/null | grep -q "Finished"; then
            echo "✅ Compiles"
        else
            echo "❌ Errors"
        fi
    fi
done
echo ""

# 4. Avatar System Assessment
echo "🎭 AVATAR SYSTEM ASSESSMENT"
echo "---------------------------"
echo "Files related to avatars:"
find crates -name "*avatar*" -type f | while read file; do
    echo "  - $file"
done
echo ""
echo "Ball/orbital avatar references:"
grep -r "ball\|orbital" crates/trinity-body/src/ --include="*.rs" | wc -l | xargs echo "  References found:"
echo ""

# 5. TODO/FIXME Count
echo "📝 OUTSTANDING TASKS"
echo "-------------------"
echo "TODO comments: $(grep -r "TODO" crates/ --include="*.rs" | wc -l)"
echo "FIXME comments: $(grep -r "FIXME" crates/ --include="*.rs" | wc -l)"
echo ""

# 6. Dependencies Analysis
echo "📦 DEPENDENCIES ANALYSIS"
echo "----------------------"
echo "Heavy dependencies (>1MB):"
grep -r "version.*=.*\".*[0-9][0-9][0-9]" Cargo.toml | grep -E "(bevy|tokio|serde)" | while read line; do
    echo "  $line"
done
echo ""

# 7. Test Coverage
echo "🧪 TEST COVERAGE"
echo "---------------"
echo "Test files: $(find crates -name "*test*.rs" | wc -l)"
echo "Integration tests: $(find crates -name "tests" -type d | wc -l)"
echo ""

echo "✨ Analysis complete!"
