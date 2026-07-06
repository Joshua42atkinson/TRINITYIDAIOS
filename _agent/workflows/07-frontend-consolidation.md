---
description: Phase 7 — Consolidate frontend: decide between phone.html PWA and React app, remove the stale one.
---

# Phase 7: Frontend Consolidation

## Objective

There are three frontend artifacts:
1. `static/phone.html` (30KB) — Working PWA with voice, TTS, chat, hotel status
2. `static/index.html` (108KB) — Full desktop IDE UI (monolithic HTML)
3. `frontend/src/` — React app with 25 components, 7 hooks (stale — `dist/` and `node_modules/` empty)

Decide which to keep, remove the others, and ensure the chosen frontend works.

## Prerequisites

- Phase 3 complete (main.rs split)
- `cargo check` passes

## Decision Matrix

| Option | Pros | Cons | Effort |
|--------|------|------|--------|
| Keep phone.html only | Works now, lightweight, mobile-first | Limited UI for desktop | 0 hr |
| Keep React app only | Rich UI, 25 components, testable | Stale (no build), needs npm install, 45 files | ~4 hr to revive |
| Keep both | Phone for mobile, React for desktop | Maintenance burden, two codebases | ~2 hr to fix React |
| Merge: phone.html → React | One codebase, best of both | Large effort, risky | ~8 hr |

## Recommended: Keep phone.html, archive React

The React app is stale (`dist/` empty, `node_modules/` empty, paths may be wrong). Reviving it is high effort with low immediate value. The phone.html PWA works and is the primary interface per the docs.

## Steps

1. **Verify phone.html works**:
   - Start Trinity server
   - Open `http://localhost:3000/trinity/phone.html`
   - Test: chat, mode toggle, voice input, TTS, hotel status

2. **Verify index.html works** (or not):
   - Open `http://localhost:3000/`
   - Test core features
   - If broken, note what's broken

3. **Archive the React frontend**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS
mkdir -p crates/trinity/frontend_archive
mv crates/trinity/frontend/src crates/trinity/frontend_archive/
mv crates/trinity/frontend/package.json crates/trinity/frontend_archive/
mv crates/trinity/frontend/package-lock.json crates/trinity/frontend_archive/
mv crates/trinity/frontend/vite.config.js crates/trinity/frontend_archive/
mv crates/trinity/frontend/index.html crates/trinity/frontend_archive/
```

4. **Update `main.rs`** to serve `static/` files correctly:
   - Ensure `static/phone.html` is served at `/trinity/phone.html`
   - Ensure `static/index.html` is served at `/`
   - Remove any references to the React `frontend/dist/` path

5. **Update `.gitignore`** to exclude `frontend_archive/` if desired, or keep it tracked.

6. **If keeping index.html**: Consider splitting it into separate JS/CSS files for maintainability. A 108KB HTML file is hard to maintain.

## Testing

```bash
# Server starts and serves both pages
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo run -p trinity -- --headless &
sleep 3
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/trinity/phone.html
```

## Completion Criteria

- One clear frontend (phone.html for mobile, index.html for desktop if it works)
- React app archived or removed
- No empty `node_modules/` or `dist/` directories
- Frontend served correctly by Trinity server
- `cargo check` passes
