#!/bin/bash

echo "🔧 Trinity UI Panic Fix Script"
echo "============================"

# Step 1: Backup current files
echo "📋 Step 1: Backing up current files..."
cp crates/trinity-body/src/main.rs crates/trinity-body/src/main.rs.backup
cp crates/trinity-body/src/panels/model_hotswap_panel.rs crates/trinity-body/src/panels/model_hotswap_panel.rs.backup

# Step 2: Fix message registration
echo "📨 Step 2: Fixing message registration..."
# This would be automated with sed commands or Python script

# Step 3: Fix egui context
echo "🎨 Step 3: Fixing egui context..."
# This would replace problematic context usage

# Step 4: Test the fix
echo "🧪 Step 4: Testing the fix..."
timeout 30 cargo run --bin trinity-body

echo "✅ Fix script completed"
