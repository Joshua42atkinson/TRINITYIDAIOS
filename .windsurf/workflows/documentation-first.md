---
description: Documentation-First Development Workflow
---

## 🛡️ Documentation-First Development

This workflow ensures we never rebuild completed work by checking Trinity's documentation first.

### Steps:

1. **Check Implementation Status**
   - Call MCP server: `check_implementation_status`
   - Parameter: the feature/component you're about to work on
   - Review the status returned

2. **Search Relevant Documentation**
   - Call MCP server: `search_documentation`
   - Query: keywords related to your task
   - Read the top 5 results

3. **Report Findings**
   - "Feature X is already [completed/in_progress/pending]"
   - "According to docs, this was done on [date]"
   - "Found X relevant documents about this"

4. **Proceed Only After Confirmation**
   - If status is "completed" - STOP and ask user
   - If status is "in_progress" - Check what's missing
   - If status is "unknown" - Proceed with caution

### Example Usage:

```
User: "Fix the Trinity self-work capabilities"

Workflow executes:
1. check_implementation_status("self-work")
   → Returns: "completed" with evidence from docs
   
2. search_documentation("autopoietic loop self-work")
   → Returns: 10 docs showing implementation complete
   
3. Reports: "Self-work is already completed (Mar 6). 
   Asset generation pipeline is functional."

4. Asks: "Are you sure you want to fix this? 
   It appears to be already implemented."
```

### MCP Tools Required:
- `trinity_mcp_server` running on localhost:8080
- PostgreSQL with pgvector extension
- Documents indexed in vector database

### Environment Variables:
- `DATABASE_URL=postgresql://trinity:trinity6226@localhost:5432/trinity`
- `TRINITY_MCP_URL=http://localhost:8080`

### Before Starting Work:
1. Run `./ingest_documentation.rs` to index latest docs
2. Start MCP server: `cargo run --package trinity-mcp-server`
3. Let the workflow check documentation first
