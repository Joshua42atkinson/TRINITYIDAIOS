---
description: Fix Rust backend logic — compile-verify before touching frontend
---

# Fix Rust Backend (Safe)

// turbo-all

## When to use
Use this when changing any `.rs` file in `crates/trinity/src/`, `crates/trinity-quest/src/`, or any protocol crate.
**Never touch frontend until this workflow completes with zero errors.**

## Steps

1. Make your Rust edits, then check for compile errors only (fast):
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo check -p trinity 2>&1 | grep "^error" | head -20
```

2. If zero errors, run the full workspace build:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo build -p trinity --release 2>&1 | grep -E "^error|Finished" | head -10
```

3. Run only the affected crate's tests:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo test -p trinity-quest 2>&1 | tail -8
```

4. Verify the API endpoint you changed responds correctly:
```bash
curl -s http://localhost:3000/api/quest | python3 -c "import sys,json; d=json.load(sys.stdin); print('Phase:', d['phase'], '| Chapter:', d['chapter'], '| XP:', d['xp'])"
```

5. If the server needs restarting, use /build-and-test — do NOT write custom kill commands inline.
