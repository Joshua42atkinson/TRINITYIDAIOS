---
description: Phase 13 — Deployment packaging: systemd services, Caddy reverse proxy, desktop files, release build verification.
---

# Phase 13: Deployment Packaging

## Objective

Finalize deployment artifacts so Trinity can be installed and run as a system service on the Strix Halo machine.

## Prerequisites

- Phases 3-5 complete (clean architecture, security hardened)
- `cargo check` passes

## Existing Artifacts

- `configs/systemd/trinity-brain.service` — systemd unit for Trinity server
- `configs/systemd/llama-server.service` — systemd unit for LLM backend
- `configs/systemd/cloudflared.service` — Cloudflare tunnel
- `configs/Caddyfile` — Caddy reverse proxy config
- `packaging/trinity-body.desktop` — Desktop entry file
- `packaging/trinity-genesis.desktop` — Desktop entry file
- `configs/hardware/strix_halo.toml` — Hardware config
- `configs/runtime/default.toml` — Runtime config

## Steps

1. **Audit systemd services**:
   - Verify `trinity-brain.service` paths match the actual binary location
   - Verify `llama-server.service` matches the current model setup (DiffusionGemma, not Qwythos)
   - Check `WantedBy`, `After`, `Requires` directives
   - Test: `systemctl start trinity-brain` (if installed)

2. **Audit Caddyfile**:
   - Verify reverse proxy targets (:3000 for Trinity, :8188 for ComfyUI)
   - Verify TLS settings
   - Verify CORS headers (should match Phase 5 settings)
   - Test: `caddy validate --config configs/Caddyfile`

3. **Audit desktop files**:
   - Verify `Exec=` paths match binary location
   - Verify `Icon=` paths match `assets/icons/trinity-icon.png`
   - Verify categories and descriptions
   - Test: `desktop-file-validate packaging/trinity-body.desktop`

4. **Audit hardware config**:
   - Verify `strix_halo.toml` matches actual hardware (128GB, gfx1151)
   - Verify VRAM budget settings match Studio mode (P: 42GB, A: 17GB)
   - Verify thread allocation

5. **Build release binary**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo build -p trinity --release 2>&1 | tail -5
ls -lh target/release/trinity
```

6. **Test release binary**:
```bash
./target/release/trinity --headless &
sleep 5
curl -s http://localhost:3000/api/health | python3 -m json.tool
```

7. **Create install script** (if not exists):
   - Copy binary to `/usr/local/bin/`
   - Copy systemd units to `/etc/systemd/system/`
   - Copy desktop files to `/usr/share/applications/`
   - Copy configs to `/etc/trinity/`
   - Enable and start services

8. **Verify startup script**:
```bash
# Test the one-command startup
bash /home/joshua/Workflow/TRINITYIDAIOS/scripts/launch/trinity_day.sh
```

## Testing

```bash
# Release build
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo build -p trinity --release 2>&1 | tail -5

# Binary size (should be under 50MB)
ls -lh target/release/trinity

# Server starts and responds
./target/release/trinity --headless &
sleep 5
curl -s http://localhost:3000/api/health | python3 -m json.tool

# Caddy config valid
caddy validate --config /home/joshua/Workflow/TRINITYIDAIOS/configs/Caddyfile

# Desktop files valid
desktop-file-validate /home/joshua/Workflow/TRINITYIDAIOS/packaging/trinity-body.desktop
```

## Completion Criteria

- Release binary builds cleanly (under 50MB)
- All systemd unit files have correct paths and dependencies
- Caddyfile valid and matches current architecture
- Desktop files valid and have correct paths
- One-command startup script works
- Trinity can be installed as a system service
