---
description: Start of session checklist — check services, build Trinity, open browser
---

# Session Start

// turbo-all

## Steps

1. Check if longcat-sglang is running:
```bash
curl -s http://127.0.0.1:8080/v1/models 2>/dev/null | head -5 || echo "⚠️ longcat-sglang NOT running on :8080 — start it with: longcat-sglang -m /path/to/model.gguf --port 8080 -ngl 99"
```

2. Check PostgreSQL:
```bash
pg_isready 2>/dev/null && echo "✅ PostgreSQL is running" || echo "⚠️ PostgreSQL not running"
```

3. Read CONTEXT.md for current project state:
```bash
head -50 /home/joshua/Workflow/desktop_trinity/trinity-genesis/CONTEXT.md
```

4. Load last session context (run AFTER Trinity is built and serving):
```
Call Trinity's load_session_context tool via the Yardmaster API:
curl -s -X POST http://127.0.0.1:3000/api/tools -H 'Content-Type: application/json' -d '{"tool":"load_session_context","params":{}}'
```
> **Note**: This returns the most recent session summary from ~/.local/share/trinity/sessions/

5. Build and start Trinity (uses /build-and-test workflow):
```
Run /build-and-test
```
