# Trinity Operations Bible
**Document:** 03-OPERATIONS.md  
**Purpose:** Commands, deployment, troubleshooting  
**Lines:** ~400  
**Isomorphic to:** [01-ARCHITECTURE.md §5](01-ARCHITECTURE.md), [02-IMPLEMENTATION.md §2](02-IMPLEMENTATION.md)

---

## 1. Quick Start

### 1.1 One-Command Launch

```bash
# Start everything (Trinity Server + longcat-sglang)
./run_trinity.sh
```

**What it does:**
1. Kills stale longcat-sglang processes
2. Starts longcat-sglang on port 8080 (base brain model)
3. Waits for model load
4. Starts `trinity` on port 3000
5. Opens browser to http://localhost:3000

### 1.2 Manual Launch (Fine Control)

```bash
# Terminal 1: Start llama.cpp server (Conductor)
./llama.cpp/build/bin/longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 99 -c 32768 --port 8080

# Terminal 2: Start Trinity Server
cargo run -p trinity --bin trinity

# Browser: Open http://localhost:3000
```

---

## 2. Launch Sequences by Role

### 2.1 Conductor (Always Required)

```bash
# Minimum viable launch
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 99 \
  -c 32768 \
  --port 8080 \
  --host 0.0.0.0
```

**Verification:**
```bash
curl http://localhost:8080/health
# Expected: HTTP 200 with model info
```

### 2.2 Yardmaster / EYE (On-Demand)

```bash
# Via sidecar API (called by Conductor)
curl -X POST http://localhost:3000/api/tools/execute \
  -H "Content-Type: application/json" \
  -d '{"tool": "sidecar_start", "params": {"model": "engineer"}}'

# Or manual start
longcat-sglang \
  -m ~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf \
  -ngl 99 -c 32768 --port 8082
```

**Verification:**
```bash
curl http://localhost:8082/health
# Expected: HTTP 200
```

### 2.3 Researcher/Swarm (Optional)

```bash
longcat-sglang \
  -m ~/trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf \
  -ngl 99 -c 32000 --port 8081
```

---

## 3. Health Checks & Monitoring

### 3.1 System Health Endpoint

```bash
# Trinity Server health (checks llama + database)
curl http://localhost:3000/api/health
```

**Expected response:**
```json
{
  "status": "ok",
  "llama": true,
  "llama_url": "http://127.0.0.1:8080",
  "database": true,
  "timestamp": "2026-03-16T17:30:00Z"
}
```

### 3.2 Individual Component Checks

| Component | Check Command | Expected |
|-----------|---------------|----------|
| **longcat-sglang** | `curl http://localhost:8080/health` | `{"status": "ok"}` |
| **Trinity Server** | `curl http://localhost:3000/api/health` | JSON with `llama: true` |
| **PostgreSQL** | `psql $DATABASE_URL -c "SELECT 1"` | `1` |
| **Sidecar** | `curl http://localhost:8082/health` | `{"status": "ok"}` |

### 3.3 Database Verification

```bash
# Check connection
psql postgres://trinity:trinity@localhost/trinity -c "SELECT COUNT(*) FROM document_embeddings;"

# Check quest state
psql postgres://trinity:trinity@localhost/trinity -c "SELECT * FROM quest_state WHERE player_id='default';"

# Check tables exist
psql postgres://trinity:trinity@localhost/trinity -c "\dt"
```

### 3.4 Memory Monitoring

```bash
# Check system memory
free -h

# Check GPU memory (if applicable)
vramfs-status  # or nvidia-smi for NVIDIA

# Check Trinity's view of memory
curl http://localhost:3000/api/status | jq '.memory_used'
```

---

## 4. Sidecar Management

### 4.1 Starting a Sidecar

```bash
# Via API (recommended — Conductor manages this)
curl -X POST http://localhost:3000/api/tools/execute \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "sidecar_start",
    "params": {
      "model": "engineer",
      "role": "yardman"
    }
  }'

# Direct binary execution
./target/release/trinity-sidecar-engineer --role engineer
```

### 4.2 Checking Sidecar Status

```bash
# Via API
curl http://localhost:3000/api/tools/execute \
  -H "Content-Type: application/json" \
  -d '{"tool": "sidecar_status"}'

# Direct health check
curl http://localhost:8090/status
```

### 4.3 Stopping a Sidecar

```bash
# Graceful shutdown via API
curl -X POST http://localhost:8090/shutdown

# Or kill process
killall longcat-sglang  # Caution: kills ALL longcat-sglang instances
```

### 4.4 Sidecar Swap Sequence (Hotel Pattern)

```bash
# 1. Check current sidecar
curl http://localhost:8090/status

# 2. Stop current sidecar
curl -X POST http://localhost:8090/shutdown

# 3. Wait for shutdown (poll until 503)
while curl -s http://localhost:8090/status; do sleep 1; done

# 4. Start new sidecar
curl -X POST http://localhost:3000/api/tools/execute \
  -H "Content-Type: application/json" \
  -d '{"tool": "sidecar_start", "params": {"model": "artist"}}'

# 5. Wait for startup (poll until 200)
until curl -s http://localhost:8090/status; do sleep 1; done

echo "Sidecar swap complete!"
```

---

## 5. Database Operations

### 5.1 Setup (First Time)

```bash
# Create database
psql postgres://postgres:postgres@localhost -c "CREATE DATABASE trinity;"

# Create user
psql postgres://postgres:postgres@localhost -c "CREATE USER trinity WITH PASSWORD 'trinity';"

# Grant privileges
psql postgres://postgres:postgres@localhost -c "GRANT ALL PRIVILEGES ON DATABASE trinity TO trinity;"

# Enable pgvector (if not already)
psql postgres://trinity:trinity@localhost/trinity -c "CREATE EXTENSION IF NOT EXISTS vector;"
```

### 5.2 Migration/Reset

```bash
# Reset quest state (WARNING: Loses progress)
psql postgres://trinity:trinity@localhost/trinity -c "DELETE FROM quest_state;"

# Full reset (WARNING: Loses everything)
psql postgres://trinity:trinity@localhost/trinity -c "DROP TABLE IF EXISTS quest_state, quest_history, document_embeddings CASCADE;"
```

### 5.3 Backup & Restore

```bash
# Backup
pg_dump postgres://trinity:trinity@localhost/trinity > trinity_backup_$(date +%Y%m%d).sql

# Restore
psql postgres://trinity:trinity@localhost/trinity < trinity_backup_20260316.sql
```

### 5.4 RAG Document Ingestion

```bash
# Via API
curl -X POST http://localhost:3000/api/ingest \
  -H "Content-Type: application/json" \
  -d '{"path": "docs/manual.md"}'

# Bulk ingestion (script)
for file in docs/*.md; do
  curl -X POST http://localhost:3000/api/ingest \
    -H "Content-Type: application/json" \
    -d "{\"path\": \"$file\"}"
done
```

---

## 6. Troubleshooting

### 6.1 longcat-sglang Won't Start

**Symptoms:**
- `curl http://localhost:8080/health` hangs or returns 000
- Trinity Server logs show "⚠️ llama.cpp server not running"

**Diagnosis:**
```bash
# Check if port is already in use
lsof -i :8080

# Check model file exists
ls -lh ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf

# Check free memory
free -h

# Try verbose launch
longcat-sglang -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf -ngl 99 --verbose
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| Port in use | `killall longcat-sglang` or use different port |
| Model not found | Check path, download model |
| OOM (Out of Memory) | Reduce `-ngl` (GPU layers) or `-c` (context) |
| GPU not detected | Use `-ngl 0` for CPU-only mode |

### 6.2 PostgreSQL Connection Failed

**Symptoms:**
- Trinity Server logs: "⚠️ PostgreSQL not available"
- RAG features disabled

**Diagnosis:**
```bash
# Check PostgreSQL running
systemctl status postgresql

# Check connection string
psql postgres://trinity:trinity@localhost/trinity -c "SELECT 1"

# Check if pgvector installed
psql postgres://trinity:trinity@localhost/trinity -c "SELECT * FROM pg_extension WHERE extname = 'vector';"
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| PostgreSQL not running | `sudo systemctl start postgresql` |
| Database doesn't exist | Run setup in §5.1 |
| User doesn't exist | `createuser -P trinity` |
| pgvector not installed | `psql -c "CREATE EXTENSION vector;"` |

### 6.3 Trinity Server Won't Start

**Symptoms:**
- `cargo run` panics or hangs
- Port 3000 already in use

**Diagnosis:**
```bash
# Check port
lsof -i :3000

# Check compilation
cargo check -p trinity --bin trinity

# Run with logging
RUST_LOG=debug cargo run -p trinity --bin trinity
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| Port 3000 in use | `killall trinity` or change port |
| Compilation error | `cargo clean && cargo build` |
| Database URL wrong | Check `DATABASE_URL` env var |
| Missing static files | Check `crates/trinity/static/` exists |

### 6.4 Sidecar Won't Start

**Symptoms:**
- `sidecar_start` tool returns error
- Sidecar health check fails

**Diagnosis:**
```bash
# Check if binary exists
ls -lh target/release/trinity-sidecar-engineer

# Check if model loaded in main longcat-sglang
curl http://localhost:8080/v1/models

# Check memory availability
free -h
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| Binary not built | `cargo build --release --bin trinity-sidecar-engineer` |
| Conductor not running | Start longcat-sglang on :8080 first |
| Insufficient memory | Check Hotel pattern, unload other models |
| Port conflict | Check if :8082 already in use |

### 6.5 OOM (Out of Memory) Crashes

**Symptoms:**
- `longcat-sglang` killed by system
- `dmesg` shows "Out of memory: Killed process"

**Diagnosis:**
```bash
# Check OOM killer logs
dmesg | grep -i "out of memory"

# Check memory at time of crash
free -h

# Check swap
swapon -s
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| Model too large | Use smaller quantization (Q4 instead of Q6) |
| Context too large | Reduce `-c` from 32768 to 16384 |
| Too many models | Ensure Hotel pattern (only 2 concurrent) |
| No swap | Enable swap: `sudo swapon /swapfile` |

### 6.6 Audio/Voice Issues (PersonaPlex)

**Symptoms:**
- No audio output from Pete
- Microphone not detected

**Diagnosis:**
```bash
# Check audio devices
pactl list sources | grep Name
pactl list sinks | grep Name

# Check WirePlumber
systemctl --user status wireplumber

# Check USB mic volume
pactl list sources | grep -A 5 "USB"
```

**Solutions:**
| Problem | Solution |
|---------|----------|
| USB mic at 0% | `pactl set-source-volume [NAME] 100%` |
| WirePlumber crash | `systemctl --user restart wireplumber` |
| Chrome disconnect | Restart Chrome after WirePlumber restart |
| Priority wrong | Check `~/.config/wireplumber/` configs |

---

## 7. Log Locations

| Component | Log Location | Rotation |
|-----------|--------------|----------|
| Trinity Server | `~/.trinity/logs/server.log` | Daily |
| Sidecar Engineer | `~/.trinity/logs/engineer.log` | Daily |
| longcat-sglang | STDOUT (redirect to file) | Manual |
| PostgreSQL | `/var/log/postgresql/` | System |

**View logs:**
```bash
# Trinity Server (live)
tail -f ~/.trinity/logs/server.log

# longcat-sglang (if redirected)
tail -f ~/.trinity/logs/llama.log
```

---

## 8. Performance Tuning

### 8.1 Strix Halo Optimization

```bash
# Maximum performance (full GPU offload)
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 99 \              # All layers on GPU
  -c 32768 \             # Maximum context
  -b 2048 \              # Large batches
  -t 16 \                # All threads
  --mlock                # Lock memory (prevent swap)
```

### 8.2 Memory-Constrained Mode

```bash
# For systems with < 64GB RAM
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 50 \              # Partial GPU offload
  -c 8192 \              # Smaller context
  -b 512 \               # Smaller batches
  -t 8                   # Fewer threads
```

### 8.3 CPU-Only Mode

```bash
# No GPU available
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf \
  -ngl 0 \               # CPU only
  -c 4096 \
  -b 256 \
  -t 8
```

---

## 9. Cross-References

| Section | Reference |
|---------|-----------|
| Architecture philosophy | [01-ARCHITECTURE.md §1](01-ARCHITECTURE.md) |
| ADDIE phases | [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md) |
| P-ART-Y roles | [01-ARCHITECTURE.md §5](01-ARCHITECTURE.md) |
| API endpoints | [02-IMPLEMENTATION.md §2](02-IMPLEMENTATION.md) |
| Database schema | [02-IMPLEMENTATION.md §3](02-IMPLEMENTATION.md) |
| Model parameters | [04-MODELS.md §2](04-MODELS.md) |

---

*End of 03-OPERATIONS.md — Commands, Deployment, and Troubleshooting*
