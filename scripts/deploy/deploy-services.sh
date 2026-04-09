#!/bin/bash
# ══════════════════════════════════════════════════════
# Trinity OS — One-Command Deploy
# Installs and activates the systemd services
# Run with: sudo bash scripts/deploy-services.sh
# ══════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "═══ Trinity Service Deployer ═══"

# 1. Stop any existing services
echo "[1/5] Stopping existing services..."
systemctl stop trinity-os trinity-llama 2>/dev/null || true

# 2. Kill any manual processes
echo "[2/5] Cleaning up manual processes..."
pkill -f "longcat-sglang" 2>/dev/null || true
pkill -f "target/release/trinity" 2>/dev/null || true
sleep 2

# 3. Copy service files
echo "[3/5] Installing service files..."
cp "$SCRIPT_DIR/trinity-llama.service" /etc/systemd/system/
cp "$SCRIPT_DIR/trinity-os.service" /etc/systemd/system/

# 4. Reload and enable
echo "[4/5] Enabling services..."
systemctl daemon-reload
systemctl enable trinity-llama trinity-os

# 5. Start everything
echo "[5/5] Starting Trinity stack..."
systemctl start trinity-llama
echo "  ↳ longcat-sglang starting (model loading ~60s)..."
systemctl start trinity-os
echo "  ↳ trinity-os waiting for llama health check..."

echo ""
echo "═══ Deploy Complete ═══"
echo "Monitor with: journalctl -u trinity-os -u trinity-llama -f"
