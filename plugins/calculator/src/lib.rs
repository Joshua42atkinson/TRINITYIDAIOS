// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — WASM Plugin: Calculator
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         plugins/calculator/src/lib.rs
// BIBLE CAR:    Car 12 — EVOLVE (Deployment & Extension)
// HOOK SCHOOL:  ⚙️ Systems — WASM Sandbox
// PURPOSE:      Calculator WASM plugin for the Trinity sandbox. Evaluates
//               mathematical expressions via the meval crate. Demonstrates
//               the WASM plugin ABI: alloc/dealloc memory management with
//               i64-packed ptr+len result format. Used as a reference for
//               building additional sandboxed tools.
//
// ARCHITECTURE:
//   • WASM ABI: alloc(len) → ptr, dealloc(ptr, len), calculate(ptr, len) → i64
//   • Input: JSON {"expression": "2 + 2"} via shared memory
//   • Output: JSON {"result": 4.0} or {"error": "..."} packed into i64
//   • Host calls alloc(), writes input, calls calculate(), reads output
//   • Bible Car 5.4: Part of Ring 5 sandboxing — plugin cannot access host FS
//
// DEPENDENCIES:
//   - serde      — JSON serialization for plugin I/O
//   - meval      — Safe mathematical expression evaluator
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created as WASM plugin prototype
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CalcInput {
    expression: String,
}

#[derive(Serialize)]
struct CalcOutput {
    result: f64,
    expression: String,
}

#[derive(Serialize)]
struct CalcError {
    error: String,
    expression: String,
}

/// Memory allocation for plugin ABI
/// Called by host to allocate memory for input
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

/// Main calculate function
/// Takes JSON input, returns JSON output
/// Input: {"expression": "2 + 2"}
/// Output: {"result": 4.0, "expression": "2 + 2"} or {"error": "...", "expression": "..."}
#[no_mangle]
pub extern "C" fn calculate(ptr: i32, len: i32) -> i64 {
    // Read input from memory
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    let input_str = match std::str::from_utf8(input_bytes) {
        Ok(s) => s,
        Err(_) => return pack_result(b"{\"error\":\"Invalid UTF-8 input\"}"),
    };

    // Parse JSON input
    let input: CalcInput = match serde_json::from_str(input_str) {
        Ok(i) => i,
        Err(e) => {
            let err = CalcError {
                error: format!("JSON parse error: {}", e),
                expression: input_str.to_string(),
            };
            return pack_result(&serde_json::to_vec(&err).unwrap_or_default());
        }
    };

    // Evaluate expression
    match meval::eval_str(&input.expression) {
        Ok(result) => {
            let output = CalcOutput {
                result,
                expression: input.expression,
            };
            pack_result(&serde_json::to_vec(&output).unwrap_or_default())
        }
        Err(e) => {
            let err = CalcError {
                error: format!("Evaluation error: {}", e),
                expression: input.expression,
            };
            pack_result(&serde_json::to_vec(&err).unwrap_or_default())
        }
    }
}

/// Pack result into i64 (ptr in high 32 bits, len in low 32 bits)
fn pack_result(output: &[u8]) -> i64 {
    let len = output.len() as i32;
    let ptr = alloc(len);

    // Write output to allocated memory
    unsafe {
        std::ptr::copy_nonoverlapping(output.as_ptr(), ptr as *mut u8, len as usize);
    }

    // Pack ptr and len into i64
    ((ptr as i64) << 32) | (len as i64 & 0xFFFFFFFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meval() {
        let result = meval::eval_str("2 + 3 * 4").unwrap();
        assert_eq!(result, 14.0);
    }
}
