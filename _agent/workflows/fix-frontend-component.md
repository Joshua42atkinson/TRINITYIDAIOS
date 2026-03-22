---
description: Update a single React component — build-verify before moving to the next one
---

# Fix Frontend Component (Safe)

// turbo-all

## When to use
Use this when changing any `.jsx` or `.js` file in `crates/trinity/frontend/src/`.
One component at a time. Build and verify before the next component.

## The anti-pattern to avoid
❌ Don't change 4 components then build once — errors compound and are hard to isolate.
✅ Change 1 component → build → verify in browser → then proceed.

## Steps

1. Make your edits to the single target component.

2. Build the frontend (fast, ~500ms):
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis/crates/trinity/frontend && npm run build 2>&1 | tail -6
```

3. If build fails, read the error and fix it before proceeding. Do not chain more changes.
If build succeeds, hard-reload the browser tab at http://localhost:3000/ and verify the component renders correctly.

4. Check specifically for React render errors in browser console:
```
Open browser DevTools → Console → look for red errors
```

5. Only after visual verification, proceed to the next component change.

## Prop threading rule
When adding a new prop (e.g. `onRefetch`), trace the full chain BEFORE writing any code:
`App.jsx → [intermediate component] → target component`
Write all layers in one edit to avoid dangling prop warnings.
