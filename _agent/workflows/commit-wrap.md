---
description: End of session — run tests, commit changes, update docs
---

# Commit and Wrap

// turbo-all

> **Root Cause Fixed (March 22)**: `git commit` was hanging because `git-lfs filter-process`
> was configured system-wide (`/etc/gitconfig`) with `required=true`, but this repo has NO
> LFS files. Fix: `git lfs uninstall --local` + `GIT_LFS_SKIP_SMUDGE=1` env var on all git commands.
> Git operations now complete in <1 second.

## Steps

1. Run tests to ensure everything passes:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && cargo test --workspace 2>&1 | tail -5
```

2. Save session summary for next session context:
```
Call Trinity's save_session_summary tool via the Yardmaster API:
curl -s -X POST http://127.0.0.1:3000/api/tools -H 'Content-Type: application/json' -d '{"tool":"save_session_summary","params":{"title":"[SESSION TITLE]","summary":"[WHAT WAS DONE]","next_steps":"[WHAT TO DO NEXT]","files_changed":"[LIST OF FILES]"}}'
```
> **Note**: Replace the bracketed values with actual session details.

3. Check what changed:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && git diff --stat
```

4. Stage all changes:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && GIT_LFS_SKIP_SMUDGE=1 git add -A
```

5. Commit with a descriptive message (EDIT the message before running):
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && GIT_LFS_SKIP_SMUDGE=1 GIT_TERMINAL_PROMPT=0 git commit --no-gpg-sign -m "session: [describe what was done]"
```

6. Show the commit:
```bash
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis && git log -1 --stat
```
