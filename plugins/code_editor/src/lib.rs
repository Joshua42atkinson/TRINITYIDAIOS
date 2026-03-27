// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — WASM Plugin: Code Editor
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         plugins/code_editor/src/lib.rs
// BIBLE CAR:    Car 12 — EVOLVE (Deployment & Extension)
// HOOK SCHOOL:  ⚙️ Systems — WASM Sandbox
// PURPOSE:      Code editor WASM plugin for the Trinity sandbox. Reads and
//               writes files within the sandboxed workspace using capability-
//               gated host functions (host_read_file, host_write_file). The
//               host enforces path validation (Ring 5) before granting access.
//               Demonstrates the capability-based security model for plugins.
//
// ARCHITECTURE:
//   • WASM ABI: alloc/dealloc + edit(ptr, len) → i64 packed result
//   • Host imports: host_read_file, host_write_file, host_log
//   • EditorAction enum: Read { path } | Write { path, content }
//   • Error codes: -2 (permission denied), -3 (not found), -4 (too large)
//   • Bible Car 5.4: Ring 5 sandboxing — host validates paths before granting
//
// DEPENDENCIES:
//   - serde — JSON serialization for action dispatch
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created as WASM plugin prototype
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

// ============================================================================
// Host Function Imports
// ============================================================================

#[link(wasm_import_module = "env")]
extern "C" {
    /// Read a file from the host
    fn host_read_file(path_ptr: i32, path_len: i32, out_ptr: i32, out_capacity: i32) -> i32;

    /// Write a file to the host
    fn host_write_file(path_ptr: i32, path_len: i32, data_ptr: i32, data_len: i32) -> i32;

    /// Log a message
    fn host_log(ptr: i32, len: i32);
}

// ============================================================================
// Data Types
// ============================================================================

#[derive(Deserialize)]
#[serde(tag = "action", content = "args")]
enum EditorAction {
    Read { path: String },
    Write { path: String, content: String },
}

#[derive(Serialize)]
struct EditorOutput {
    success: bool,
    data: Option<String>,
    error: Option<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn log(msg: &str) {
    let msg_bytes = msg.as_bytes();
    unsafe {
        host_log(msg_bytes.as_ptr() as i32, msg_bytes.len() as i32);
    }
}

fn read_file(path: &str) -> Result<String, String> {
    let path_bytes = path.as_bytes();
    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer max

    unsafe {
        let len = host_read_file(
            path_bytes.as_ptr() as i32,
            path_bytes.len() as i32,
            buffer.as_mut_ptr() as i32,
            buffer.len() as i32,
        );

        if len < 0 {
            match len {
                -2 => return Err("Permission denied".to_string()),
                -3 => return Err("File not found or read error".to_string()),
                -4 => return Err("File too large for buffer".to_string()),
                _ => return Err("Unknown error".to_string()),
            }
        }

        buffer.truncate(len as usize);
    }

    String::from_utf8(buffer).map_err(|e| format!("Invalid UTF-8: {}", e))
}

fn write_file(path: &str, content: &str) -> Result<usize, String> {
    let path_bytes = path.as_bytes();
    let content_bytes = content.as_bytes();

    unsafe {
        let len = host_write_file(
            path_bytes.as_ptr() as i32,
            path_bytes.len() as i32,
            content_bytes.as_ptr() as i32,
            content_bytes.len() as i32,
        );

        if len < 0 {
            match len {
                -2 => return Err("Permission denied".to_string()),
                -3 => return Err("Write error".to_string()),
                _ => return Err("Unknown error".to_string()),
            }
        }

        Ok(len as usize)
    }
}

// ============================================================================
// Entry Point (ABI)
// ============================================================================

/// Memory allocation for plugin ABI
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let layout = std::alloc::Layout::from_size_align(len as usize, 1).unwrap();
    unsafe { std::alloc::alloc(layout) as i32 }
}

/// Free allocated memory
#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {
    let layout = std::alloc::Layout::from_size_align(len as usize, 1).unwrap();
    unsafe { std::alloc::dealloc(ptr as *mut u8, layout) }
}

/// Pack result into i64 (ptr in high 32 bits, len in low 32 bits)
fn pack_result(output: &[u8]) -> i64 {
    let len = output.len() as i32;
    let ptr = alloc(len);

    unsafe {
        std::ptr::copy_nonoverlapping(output.as_ptr(), ptr as *mut u8, len as usize);
    }

    ((ptr as i64) << 32) | (len as i64 & 0xFFFFFFFF)
}

/// Main entry point
#[no_mangle]
pub extern "C" fn edit(ptr: i32, len: i32) -> i64 {
    // Read input from memory
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    let input_str = match std::str::from_utf8(input_bytes) {
        Ok(s) => s,
        Err(_) => return pack_result(b"{\"success\":false,\"error\":\"Invalid UTF-8 input\"}"),
    };

    // Parse JSON command
    let action: EditorAction = match serde_json::from_str(input_str) {
        Ok(a) => a,
        Err(e) => {
            return pack_result(
                serde_json::to_vec(&EditorOutput {
                    success: false,
                    data: None,
                    error: Some(format!("JSON parse error: {}", e)),
                })
                .unwrap()
                .as_slice(),
            )
        }
    };

    // Execute action
    let result = match action {
        EditorAction::Read { path } => {
            log(&format!("Reading file: {}", path));
            match read_file(&path) {
                Ok(content) => EditorOutput {
                    success: true,
                    data: Some(content),
                    error: None,
                },
                Err(e) => EditorOutput {
                    success: false,
                    data: None,
                    error: Some(e),
                },
            }
        }
        EditorAction::Write { path, content } => {
            log(&format!("Writing file: {}", path));
            match write_file(&path, &content) {
                Ok(_) => EditorOutput {
                    success: true,
                    data: Some("File written successfully".to_string()),
                    error: None,
                },
                Err(e) => EditorOutput {
                    success: false,
                    data: None,
                    error: Some(e),
                },
            }
        }
    };

    pack_result(&serde_json::to_vec(&result).unwrap_or_default())
}
