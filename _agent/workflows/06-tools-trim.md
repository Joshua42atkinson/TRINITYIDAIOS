---
description: Phase 6 — Trim tools.rs from 42 tools to ~18 builder-relevant tools. Remove Purdue-era and monitoring bloat.
---

# Phase 6: Tools Trim

## Objective

`tools.rs` (1,468 lines) defines 42 tools for the agent loop. Many are Purdue-era educational tools or monitoring bloat irrelevant to the creative studio vision. Trim to ~18 builder-relevant tools.

## Prerequisites

- Phase 3 complete (main.rs split)
- `cargo check` passes

## Analysis

### KEEP — Builder-Relevant Tools (~18)
These are tools a creative studio needs:
- `list_dir` — File system navigation
- `read_file` — Read source files
- `write_file` — Write generated content
- `edit_file` — Modify existing files
- `run_command` — Execute shell commands
- `search_web` — Web search for references
- `generate_image` — Art generation via ComfyUI
- `generate_tempo` — Music/audio generation
- `generate_video` — Video generation
- `generate_mesh3d` — 3D model generation
- `rag_search` — Knowledge base search
- `rag_ingest` — Add documents to RAG
- `save_session_context` — Persist session state
- `load_session_context` — Recall prior session
- `create_project` — Start a new creative project
- `list_projects` — List existing projects
- `save_character_sheet` — Save user profile/progress
- `load_character_sheet` — Load user profile/progress

### REMOVE — Purdue-Era / Irrelevant Tools
Audit `tools.rs` for tools matching these patterns and remove them:
- Educational assessment tools (quiz grading, rubric scoring)
- Scope creep detection tools
- Perspective/reflection tools
- RLHF/feedback tools
- Quality scorecard tools
- Any tool that's only relevant to the instructional design product, not the creative studio

## Steps

1. **List all tools** defined in `tools.rs`:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && grep -n "pub fn\|\"name\".*:" crates/trinity/src/tools.rs | head -50
```

2. **Map each tool** to KEEP or REMOVE based on the lists above.

3. **Remove tools one at a time**:
   - Delete the tool function
   - Delete the tool registration in the tool registry (likely in `tools.rs` or `agent.rs`)
   - Run `cargo check`
   - Note: The agent loop will gracefully handle missing tools — the LLM just won't be offered them

4. **Update tool descriptions** for remaining tools to match the creative studio vision (not educational language).

5. **Verify the agent loop still works** with the trimmed tool set.

## Testing

```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo test -p trinity 2>&1 | tail -10
```

## Completion Criteria

- `tools.rs` is under 800 lines
- ~18 builder-relevant tools remain
- No references to removed tools in `agent.rs` or `main.rs`
- `cargo check` passes
- Agent loop functions correctly with trimmed tool set
