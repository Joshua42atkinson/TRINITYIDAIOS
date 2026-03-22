---
description: Build, test, and start the Trinity server with health verification
---

# Build and Test Trinity

// turbo-all

## Steps

0. Kill zombie cargo/rustc/git processes and remove stale lock files:
```bash
pkill -f "cargo build" 2>/dev/null; pkill -f "cargo clippy" 2>/dev/null; pkill -f "cargo test" 2>/dev/null; rm -f /home/joshua/Workflow/desktop_trinity/trinity-genesis/.git/index.lock; echo "✅ Zombies cleared"
```

1. Run the full test suite to catch regressions:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo test --workspace 2>&1 | tail -10
```

2. Build the release binary:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo build -p trinity --release 2>&1 | tail -5
```

3. Kill any existing Trinity server on port 3000:
```bash
lsof -ti :3000 | xargs kill 2>/dev/null; echo "Port cleared"
```

4. Start the Trinity server:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo run -p trinity --release 2>&1 &
sleep 5
```

5. Verify server health:
```bash
curl -s http://localhost:3000/api/health | python3 -m json.tool
```

6. Open the UI in the browser for manual verification:
```
Navigate to http://localhost:3000/
```
