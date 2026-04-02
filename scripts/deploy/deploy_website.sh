#!/bin/bash
# ============================================================
# LDTAtkinson.com Deploy Script
# Run once, walk away. Everything happens automatically.
# ============================================================

set -e  # Stop on any error
echo "🚂 TRINITY DEPLOY — LDTAtkinson.com"
echo "======================================"

# Step 1: Build the portfolio
echo ""
echo "📦 Step 1/4: Building LDTAtkinson portfolio..."
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis/LDTAtkinson/client
npm run build 2>&1 | tail -5
echo "✅ Portfolio built!"

# Step 2: Copy Caddyfile
echo ""
echo "⚙️  Step 2/4: Installing Caddyfile..."
sudo mkdir -p /var/log/caddy
sudo cp /home/joshua/Workflow/desktop_trinity/trinity-genesis/configs/Caddyfile /etc/caddy/Caddyfile
echo "✅ Caddyfile installed!"

# Step 3: Validate Caddy config
echo ""
echo "🔍 Step 3/4: Validating Caddy configuration..."
sudo caddy validate --config /etc/caddy/Caddyfile 2>&1 || {
    echo "⚠️  Caddy config has issues — check above errors"
    exit 1
}
echo "✅ Config valid!"

# Step 4: Restart Caddy
echo ""
echo "🔄 Step 4/4: Restarting Caddy..."
sudo systemctl restart caddy
sleep 2
sudo systemctl status caddy --no-pager -l 2>&1 | head -15
echo ""

# Verify
echo "======================================"
echo "🎉 DEPLOY COMPLETE!"
echo ""
echo "Local test (run these to check):"
echo "  curl -s http://localhost | head -5"
echo "  curl -s http://localhost/trinity/api/health"
echo ""
echo "⚠️  REMAINING MANUAL STEPS:"
echo "  1. Router: Forward ports 80 & 443 → 192.168.0.69"
echo "     (Visit http://192.168.0.1 in browser)"
echo ""
echo "  2. Squarespace DNS: Add A records"
echo "     @ → 71.161.122.153"
echo "     www → 71.161.122.153"
echo "     (Visit https://account.squarespace.com/domains)"
echo ""
echo "After DNS propagates (~5-15 min):"
echo "  https://LDTAtkinson.com ← Portfolio"
echo "  https://LDTAtkinson.com/trinity/ ← Trinity ID AI OS"
echo "======================================"
