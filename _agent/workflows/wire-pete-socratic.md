---
description: Wire Pete's Socratic Protocol — test that AI responses reflect the current quest phase
---

# Wire Pete Socratic Protocol

## When to use
When changing the system prompt or chat endpoint in `crates/trinity/src/main.rs` or `trinity_api.rs`.
Goal: Pete should ALWAYS know: current phase, active objectives, PEARL subject/vision.

## Steps

1. Run /fix-rust-backend first to verify the Rust change compiles.

// turbo
2. Restart Trinity server with the updated binary:
```bash
lsof -ti :3000 | xargs kill 2>/dev/null; sleep 1
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo run -p trinity --release >> /tmp/trinity.log 2>&1 &
sleep 8 && curl -s http://localhost:3000/api/health | python3 -c "import sys,json; d=json.load(sys.stdin); print('UP | DB:', d['database']['connected'])"
```

3. Verify Pete's prompt includes phase context by checking the chat response:
```bash
curl -s -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What are my current objectives?", "mode": "iron-road"}' \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('response','')[:500])"
```

4. Expected: Pete's response should mention the specific phase objectives (e.g. "Describe yourself: What do you teach?") — NOT generic coach talk.
If generic: the system prompt injection in main.rs is not reaching the chat handler.

5. Test PEARL context injection:
```bash
curl -s -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What subject am I working on?", "mode": "iron-road"}' \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('response','')[:300])"
```

6. Expected: Pete mentions the specific subject (e.g. "Algebra") and medium (e.g. "Game").
