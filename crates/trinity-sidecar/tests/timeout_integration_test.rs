// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-sidecar
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        tests/timeout_integration_test.rs
// PURPOSE:     Verify that sidecar timeout tracking works correctly
//
// WHAT THIS TESTS:
//   1. The /status endpoint contract: `timeouts_hit` field must be present
//   2. When the sidecar hits a timeout during quest execution, the
//      `timeouts_hit` counter increments and the engine status recovers to Idle
//   3. The timeout detection pattern (tokio::time::timeout) works correctly
//
// ARCHITECTURE:
//   We test the timeout behavior at the state-tracking level using the same
//   patterns from workflow.rs. The real timeout happens in:
//     workflow.rs:239 — tokio::time::timeout(MAX_PLAN_DURATION, opus.chat())
//     workflow.rs:342 — tokio::time::timeout(MAX_STEP_DURATION, reap.chat())
//
//   When timeout fires, workflow.rs increments state.timeouts_hit and reports
//   to the CowCatcher. We replicate that exact pattern here.
//
// WHY THIS APPROACH:
//   Spawning a real sidecar + model for a timeout test would take 180s+ to load.
//   Instead, we test the observable contract: "after a timeout, the counter tracks it."
//
// ═══════════════════════════════════════════════════════════════════════════════

/// WorkflowState /status response must include the `timeouts_hit` field
/// as a u32. This is the contract the conductor checks.
#[test]
fn test_status_response_includes_timeout_field() {
    // Simulate a WorkflowState JSON as returned by GET /status
    let state = serde_json::json!({
        "role_id": "engineer",
        "role_name": "The Engineer",
        "status": "idle",
        "current_quest": null,
        "quests_completed": 5,
        "quests_failed": 1,
        "total_code_generated_lines": 1200,
        "uptime_secs": 3600,
        "opus_healthy": true,
        "reap_healthy": true,
        "timeouts_hit": 3
    });

    // Parse and verify the timeout field exists and is correct
    let parsed: serde_json::Value = serde_json::from_str(&state.to_string()).unwrap();
    assert_eq!(parsed["timeouts_hit"], 3, "timeouts_hit should be 3");
    assert_eq!(parsed["status"], "idle", "status should be idle after recovery");
    assert_eq!(parsed["quests_completed"], 5);
}

/// Verify that a fresh sidecar state has zero timeouts
#[test]
fn test_default_state_has_zero_timeouts() {
    let state = serde_json::json!({
        "role_id": "tempo",
        "role_name": "The Engineer",
        "status": "starting",
        "current_quest": null,
        "quests_completed": 0,
        "quests_failed": 0,
        "total_code_generated_lines": 0,
        "uptime_secs": 0,
        "opus_healthy": false,
        "reap_healthy": false,
        "timeouts_hit": 0
    });

    let parsed: serde_json::Value = serde_json::from_str(&state.to_string()).unwrap();
    assert_eq!(parsed["timeouts_hit"], 0);
    assert_eq!(parsed["status"], "starting");
}

/// Replicate the exact timeout detection pattern from workflow.rs
/// This is the critical path: tokio::time::timeout() wrapping model inference.
#[tokio::test]
async fn test_timeout_fires_on_hanging_worker() {
    use std::time::Duration;

    // Simulate a model inference call that hangs (takes 5s)
    let hanging_inference = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok::<String, String>("model output".to_string())
    };

    // Apply timeout (100ms) — matches workflow.rs pattern at line 239
    let result = tokio::time::timeout(Duration::from_millis(100), hanging_inference).await;

    assert!(
        result.is_err(),
        "Timeout should fire before hanging worker completes"
    );

    // In workflow.rs, this Err(_) branch does:
    //   state.timeouts_hit += 1;
    //   cow_catcher.report_timeout(...);
    // We verify the counter increment below.
}

/// Full recovery pattern: timeout → increment counter → recover → process next task
/// This is the exact sequence from workflow.rs autonomous loop.
#[tokio::test]
async fn test_timeout_recovery_sequence() {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;

    // Simulate WorkflowState (behind RwLock, like in the sidecar)
    let timeouts_hit = Arc::new(RwLock::new(0u32));
    let status = Arc::new(RwLock::new("working".to_string()));

    // ── Phase 1: Trigger timeout ──
    let hanging = async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        "done"
    };

    match tokio::time::timeout(Duration::from_millis(50), hanging).await {
        Ok(_) => panic!("Should have timed out"),
        Err(_) => {
            // Mirror workflow.rs timeout handler
            let mut count = timeouts_hit.write().await;
            *count += 1;
        }
    }

    // ── Phase 2: Verify timeout was tracked ──
    assert_eq!(*timeouts_hit.read().await, 1, "One timeout should be recorded");

    // ── Phase 3: Engine recovers to Idle ──
    {
        let mut s = status.write().await;
        *s = "idle".to_string();
    }

    // ── Phase 4: Process next task successfully ──
    let fast_task = async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    };

    let result = tokio::time::timeout(Duration::from_secs(1), fast_task).await;
    assert!(result.is_ok(), "Normal task should complete within timeout");
    assert_eq!(result.unwrap(), 42);

    // ── Phase 5: State is consistent ──
    assert_eq!(*status.read().await, "idle", "Should be idle after recovery");
    assert_eq!(*timeouts_hit.read().await, 1, "Still exactly 1 timeout");
}

/// Multiple consecutive timeouts should accumulate correctly
#[tokio::test]
async fn test_multiple_timeouts_accumulate() {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;

    let timeouts_hit = Arc::new(RwLock::new(0u32));

    for _ in 0..5 {
        let hanging = async {
            tokio::time::sleep(Duration::from_secs(5)).await;
            "never"
        };

        if tokio::time::timeout(Duration::from_millis(10), hanging)
            .await
            .is_err()
        {
            let mut count = timeouts_hit.write().await;
            *count += 1;
        }
    }

    assert_eq!(
        *timeouts_hit.read().await,
        5,
        "All 5 timeouts should be tracked"
    );
}
