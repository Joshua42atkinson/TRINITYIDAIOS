# §17 Comment Standard for Trinity
**Document:** §17-STANDARD.md  
**Purpose:** Mandatory comment format for all Rust source files  
**Applies to:** All `.rs` files in `crates/`  
**Lines:** ~200

---

## 1. Philosophy: "Teachability Through Documentation"

Every Trinity source file must be understandable by:
- **Vibe coders** who don't know Rust
- **LLM assistants** with limited context windows
- **Future maintainers** who need to know "why" not just "what"
- **New team members** onboarding to the project

The §17 standard ensures files are **self-documenting** through standardized headers.

---

## 2. File Header Format (Mandatory)

Every `.rs` file MUST begin with this exact structure:

```rust
// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — [crate-name]
// ═══════════════════════════════════════════════════════════════════════════════
// 
// FILE:        [filename].rs
// PURPOSE:     [One-line description of what this file does]
// 
// ARCHITECTURE:
//   • [Bullet 1: High-level structural pattern]
//   • [Bullet 2: Key module/component relationships]
//   • [Bullet 3: Data flow or state management approach]
//   • [Bullet 4: Hardware integration or special considerations]
//
// DEPENDENCIES:
//   - [crate::module] — [What it provides to this file]
//   - [external-crate] — [Specific feature used]
//   - [std::module] — [Standard library component]
//
// CHANGES:
//   YYYY-MM-DD  [Author]  [What changed and why]
//   YYYY-MM-DD  [Author]  [What changed and why]
//
// ═══════════════════════════════════════════════════════════════════════════════
```

---

## 3. Required Sections

### 3.1 FILE
- Just the filename, no path
- Example: `FILE: main.rs`

### 3.2 PURPOSE
- One clear sentence
- Active voice: "HTTP API entry point" not "This is the HTTP API entry point"
- Focus on "what" not "how"

**Good:** `PURPOSE: Axum server initialization and route registration`  
**Bad:** `PURPOSE: This file sets up the server`

### 3.3 ARCHITECTURE (Minimum 4 bullets)

Each bullet must follow: `• [Structure]: [Explanation of why/how]`

| Position | Content | Example |
|----------|---------|---------|
| 1st | Layer/position in system | `Layer 1 of Trinity 3-Layer Architecture` |
| 2nd | Key modules/components | `Module structure: agent, tools, rag, quests` |
| 3rd | Data flow/state | `Broadcast channel for SSE streaming` |
| 4th | Hardware/special | `NPU Integration: FastFlowLM for Iron Road` |

### 3.4 DEPENDENCIES

Format: `- crate::module — Specific purpose`

Include:
- Internal crate dependencies
- External crates with specific features
- Standard library modules (if non-obvious)

**Example:**
```rust
// DEPENDENCIES:
//   - axum — HTTP framework for API routes
//   - tokio — Async runtime (multi-threaded)
//   - serde — JSON serialization
//   - tower-http — CORS middleware
//   - trinity_protocol — Shared ChatMessage types
```

### 3.5 CHANGES

Format: `YYYY-MM-DD  Author  Brief description`

- Most recent changes first
- Include "why" not just "what"
- Reference PR/issue numbers if applicable

**Example:**
```rust
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//   2026-03-14  Cascade  Added SSE streaming for real-time updates
//   2026-03-12  Cascade  Integrated NPU Great Recycler for book updates
```

---

## 4. Function Documentation

### 4.1 Doc Comments (///)

Every **public** function must have:

```rust
/// Brief description of what this function does.
/// 
/// # Arguments
/// * `param` — Description of parameter
/// 
/// # Returns
/// Description of return value
/// 
/// # Example
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 84);
/// ```
/// 
/// # Errors
/// When this function returns Err, what happened
pub fn my_function(param: i32) -> Result<i32, Error> {
```

### 4.2 Section Headers Within Functions

For complex logic, use inline section markers:

```rust
pub fn complex_operation() -> Result<(), Error> {
    // ═══════ STAGE 1: INPUT VALIDATION ═══════
    // We validate here because [reason]. This prevents [problem].
    // The validation order matters: [explain dependency]
    validate_inputs()?;
    
    // ═══════ STAGE 2: DATA TRANSFORMATION ═══════
    // Convert from [format A] to [format B] because [reason]
    let transformed = transform_data(&inputs);
    
    // ═══════ STAGE 3: PERSISTENCE ═══════
    // Write to PostgreSQL for durability. If this fails, [recovery].
    save_to_database(&transformed).await?;
    
    Ok(())
}
```

---

## 5. Complete Example

```rust
// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
// 
// FILE:        agent.rs
// PURPOSE:     Agentic chat endpoint — multi-turn tool-calling loops
// 
// ARCHITECTURE:
//   • Implements ReAct pattern (Reason + Act) for agentic workflows
//   • Loop: User input → LLM reasoning → Tool execution → Observation → LLM
//   • Tool results appended to conversation history for context
//   • VAAM integration for streaming tool execution status
//   • Timeout: 120s max per request to prevent runaway loops
//
// DEPENDENCIES:
//   - axum — HTTP handlers and JSON extraction
//   - serde — Request/response serialization
//   - tokio::sync — Async channel for SSE updates
//   - crate::tools — Tool execution (7 agentic tools)
//   - crate::inference — LLM completion API
//   - trinity_protocol — ChatMessage type definitions
//
// CHANGES:
//   2026-03-16  Cascade  Migrated to §17 comment standard
//   2026-03-14  Cascade  Added tool timeout handling (120s limit)
//   2026-03-10  Cascade  Initial ReAct loop implementation
//
// ═══════════════════════════════════════════════════════════════════════════════

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

/// Execute agentic chat with tool-calling capabilities.
/// 
/// This implements the ReAct (Reason + Act) pattern where the LLM:
/// 1. Receives user message
/// 2. Reasons about what tools to call
/// 3. Executes tools
/// 4. Observes results
/// 5. Generates final response
/// 
/// # Arguments
/// * `state` — Application state (DB pool, LLM client)
/// * `request` — ChatRequest with message and optional RAG flag
/// 
/// # Returns
/// SSE stream of ChatMessage chunks
/// 
/// # Example
/// ```
/// POST /api/chat/agent
/// {"message": "Read README.md and summarize"}
/// ```
/// 
/// # Errors
/// Returns 500 if LLM unavailable or tool execution fails
pub async fn agent_chat_stream(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // ═══════ STAGE 1: SETUP ═══════
    // Initialize conversation and tool registry
    let mut messages = vec![system_message(), user_message(&request.message)];
    let tools = load_tools();
    
    // ═══════ STAGE 2: REACT LOOP ═══════
    // Continue until LLM returns content (not tool_calls) or max iterations
    for iteration in 0..MAX_ITERATIONS {
        let response = llm_complete(&state, &messages).await?;
        
        match response.finish_reason {
            FinishReason::ToolCalls => {
                // Execute tools and append results
                let results = execute_tools(&response.tool_calls).await;
                messages.extend(tool_messages(results));
            }
            FinishReason::Stop => {
                // Final response received
                return stream_response(response.content);
            }
        }
    }
    
    // ═══════ STAGE 3: FALLBACK ═══════
    // If max iterations reached, return best effort response
    stream_response("Maximum iterations reached. Partial results provided.".to_string())
}
```

---

## 6. Verification Checklist

Before committing any `.rs` file, verify:

- [ ] File begins with `// ═══════` header block
- [ ] FILE line matches actual filename
- [ ] PURPOSE is one clear sentence
- [ ] ARCHITECTURE has minimum 4 bullets
- [ ] DEPENDENCIES lists all non-obvious crates
- [ ] CHANGES includes recent edits
- [ ] All public functions have `///` doc comments
- [ ] Complex logic has inline `// ═══════` section markers

---

## 7. Tool Support

### 7.1 Automated Checking

```bash
# Check all .rs files for §17 compliance
./scripts/check_headers.sh

# Fix missing headers (interactive)
./scripts/fix_headers.sh
```

### 7.2 IDE Snippets

**VS Code snippet** (`.vscode/trinity.code-snippets`):
```json
{
  "§17 Header": {
    "prefix": "trinity-header",
    "body": [
      "// ═══════════════════════════════════════════════════════════════════════════════",
      "// TRINITY ID AI OS — ${TM_DIRECTORY/(.*\\/crates\\/)([^/]+)(\\/.*)?/$2/}",
      "// ═══════════════════════════════════════════════════════════════════════════════",
      "// ",
      "// FILE:        ${TM_FILENAME}",
      "// PURPOSE:     $1",
      "// ",
      "// ARCHITECTURE:",
      "//   • $2",
      "//   • $3",
      "//   • $4",
      "//   • $5",
      "//",
      "// DEPENDENCIES:",
      "//   - $6",
      "//",
      "// CHANGES:",
      "//   ${CURRENT_YEAR}-${CURRENT_MONTH}-${CURRENT_DATE}  ${USER}  $7",
      "//",
      "// ═══════════════════════════════════════════════════════════════════════════════",
      ""
    ],
    "description": "Insert §17 Trinity header"
  }
}
```

---

## 8. Isomorphic Application

The §17 standard applies **isomorphically** across the codebase:

| Layer | Example File | Standard Applied |
|-------|--------------|------------------|
| **Server** | `trinity-server/src/main.rs` | Full §17 header + fn docs |
| **Kernel** | `trinity-kernel/src/agent.rs` | Full §17 header + fn docs |
| **Protocol** | `trinity-protocol/src/lib.rs` | Full §17 header + fn docs |
| **Tests** | `trinity-server/tests/*.rs` | §17 header + test descriptions |

---

## 9. Migration Guide

### For Existing Files Without Headers

1. Read file to understand purpose
2. Identify architecture patterns
3. List dependencies
4. Check git log for recent changes
5. Insert §17 header at top
6. Update function docs as you touch code
7. Commit with message: `docs: §17 header for [filename]`

### Example Migration Commit

```bash
git commit -m "docs: §17 headers for trinity-server

- main.rs: HTTP API entry point
- agent.rs: Agentic chat with ReAct loop  
- tools.rs: 7 agentic tools (read, write, shell, search)
- quests.rs: Hero's Journey quest engine
- rag.rs: PostgreSQL RAG with pgvector

Standardizes documentation for LLM parsing and
maintainability."
```

---

## 10. Cross-References

| Section | Reference |
|---------|-----------|
| Architecture philosophy | [01-ARCHITECTURE.md §1](01-ARCHITECTURE.md) |
| Code examples | [02-IMPLEMENTATION.md](02-IMPLEMENTATION.md) |
| Operations | [03-OPERATIONS.md](03-OPERATIONS.md) |

---

*End of §17-STANDARD.md — Trinity Comment Standard*
