#!/bin/bash

# Fix workspace configuration issues

echo "🔧 Fixing workspace configuration..."

# Remove nested workspace from subagents
sed -i '/^\[workspace\]/,/^$/d' crates/trinity-subagents/Cargo.toml

# Add missing dependencies to subagents
for subagent in draftsman engineer yardmaster brakeman diffusion nitrogen omni; do
    echo "Updating $subagent dependencies..."
    sed -i 's/std::sync = { version = "1.0", features = \["arc_lock"\] }/std::sync = { version = "1.0" }/' "crates/trinity-subagents/trinity-$subagent/Cargo.toml"
done

echo "✅ Workspace configuration fixed!"
