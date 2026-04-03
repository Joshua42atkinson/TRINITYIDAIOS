# Trinity ID AI OS — Session Guide
## Updated: March 21, 2026 — Phase 4 UI Wiring Complete (v4.4.0)

---

## Quick Start (5 minutes)

### 1. Start Mistral (the brain)
```bash
# From the trinity-genesis workspace
./scripts/launch/demo_quick_start.sh
# or manually:
llama-server \
  -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --host 0.0.0.0 --port 8080 -ngl 99 -c 8192
```

### 2. Start Trinity
```bash
# No database setup needed — Trinity uses embedded SQLite

# Build and run
cargo run -p trinity --release
```
Server starts on `http://localhost:3000`

### 3. Chat with Pete
Open browser to `http://evo-x2:3000/` or:
```bash
curl -X POST http://localhost:3000/api/chat/yardmaster \
  -H "Content-Type: application/json" \
  -d '{"message": "Hey Pete, I want to build a vocabulary game for my biology class"}'
```

---

## Session Checklist

### Before Starting
- [ ] LM Studio / Ollama running (or `LLM_URL` set)
- [ ] Mistral loaded (`curl http://localhost:8080/health`)
- [ ] In workspace: `cd ~/Workflow/desktop_trinity/trinity-genesis`

### During Session
- [ ] Trinity compiles: `cargo check -p trinity`
- [ ] Tests pass: `cargo test --workspace` (93 tests)
- [ ] API responds: `curl http://localhost:3000/api/health`

### After Session
- [ ] Run `/commit-wrap` in Antigravity IDE, or:
- [ ] Git commit: `git add -A && git commit -m "session notes"`
- [ ] Check session history: `GET /api/sessions`

---

## Key API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/api/chat/yardmaster` | Agentic chat (tool-calling) |
| POST | `/api/v1/trinity` | Unified chat ("talk to Trinity") |
| GET | `/api/quest` | Full game state + ADDIECRAPEYE phases |
| POST | `/api/quest/compile` | Compile GDD from quest progress |
| GET | `/api/sessions` | List conversation sessions |
| GET | `/api/sessions/history` | Load chat history |
| GET | `/api/rag/stats` | RAG knowledge base statistics |
| POST | `/api/rag/search` | Semantic search |
| GET | `/api/projects` | List game projects |
| POST | `/api/tools/execute` | Agentic tool execution |
| GET | `/api/health` | Subsystem health checks |

---

## Troubleshooting

### LLM Not Responding
```bash
curl http://localhost:8080/health
# If down, restart llama-server (see Quick Start step 1)
```

### Database Issues
```bash
# Trinity uses SQLite — data lives in trinity_memory.db
# If needed, delete and restart to reset:
rm trinity_memory.db && cargo run -p trinity --release
```

### Compile Errors
```bash
cargo check -p trinity 2>&1 | head -20
# Current state: 0 errors, warnings only
```

---

## Architecture Quick Reference

```
128GB Unified RAM (Strix Halo)
├── Mistral Small 4 119B (~68GB on :8080)
├── Trinity Axum Server (~2GB on :3000)
│   ├── Agent chat + tool-calling
│   ├── SQLite persistence (sessions, messages, projects)
│   ├── ONNX RAG (semantic search + auto-ingest)
│   ├── ADDIECRAPEYE orchestration (conductor_leader.rs)
│   ├── Bevy game scaffolding (templates/)
│   └── VAAM alignment (vocabulary weights → system prompts)
├── ComfyUI SDXL Turbo (~2GB on :8188, optional)
├── Voice Pipeline (~2GB on :7777, optional)
└── System (~10GB)
```

---

*Updated for Phase 4 UI Wiring — March 21, 2026*

---

## Antigravity IDE Workflows

These slash commands are available in the Antigravity IDE chat window:

| Command | What It Does |
|---------|-------------|
| `/build-and-test` | Build release, run tests, start server, verify health |
| `/session-start` | Check all services, build, open browser |
| `/commit-wrap` | Run tests, stage changes, commit with message |
| `/research-implementation` | Implement research findings (turbo-all) |

All workflows have `// turbo-all` — the IDE auto-runs every command step.
