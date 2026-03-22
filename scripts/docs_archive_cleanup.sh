#!/bin/bash
# docs_archive_cleanup.sh
# Run this when the system is not under memory pressure
# Moves outdated docs to docs/archive/pre-march20/

set -e

DOCS=/home/joshua/Workflow/desktop_trinity/trinity-genesis/docs
ARCHIVE=$DOCS/archive/pre-march20

mkdir -p "$ARCHIVE"

echo "=== Archiving stale root docs ==="
# 12-crate map (architecture collapsed to fewer crates)
mv -v "$DOCS/TRINITY_12_CRATE_MAP.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"
# Orchestration map (references GPT-OSS-20B, PersonaPlex, old models)
mv -v "$DOCS/TRINITY_AI_ORCHESTRATION_MAP.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"

echo "=== Archiving stale research docs ==="
for f in \
  vllm_sidecar_architecture.md \
  vllm_sidecar_architecture_final.md \
  vllm_omni_yardmaster_setup.md \
  vllm_diffusion_art_analysis.md \
  blender_bridge_verification.md \
  sidecar_evolution_plan.md \
  ming_download_strategy.md \
  ming_dynamic_quantization.md \
  ming_world_evaluation.md \
  ming_yardmaster_integration.md \
  MING_INTEGRATION_LOG.md \
  MING_STRIX_HALO_IMPLEMENTATION.md \
  bevy_winit_patch_issue.md \
  bevy_winit_investigation.md \
  bevy_ui_strategy.md \
  ui_archive_analysis.md \
  ui_overhaul_summary.md \
  ui_usability_scan.md \
  ui_deployment_strategy.md \
  ring_mini_sparse_analysis.md \
  sglang_yardmaster_evaluation.md; do
  mv -v "$DOCS/research/$f" "$ARCHIVE/" 2>/dev/null || echo "  $f: already moved or missing"
done

echo "=== Archiving stale crate-manuals ==="
mv -v "$DOCS/crate-manuals/nemotron-3-super-guide.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"
mv -v "$DOCS/crate-manuals/trinity-adaptive-model-system.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"

echo "=== Archiving stale reference docs ==="
mv -v "$DOCS/reference/TRINITY_DAYDREAM_INTEGRATION.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"

echo "=== Archiving stale report docs ==="
mv -v "$DOCS/reports/PYTHON_BRIDGE_STRATEGY.md" "$ARCHIVE/" 2>/dev/null || echo "already moved"

echo ""
echo "=== Cleanup summary ==="
echo "Archived $(find "$ARCHIVE" -name "*.md" | wc -l) docs to $ARCHIVE"
echo "Remaining active docs: $(find "$DOCS" -name "*.md" -not -path "*/archive/*" | wc -l)"
echo ""
echo "Done! Docs directory is now clean."
