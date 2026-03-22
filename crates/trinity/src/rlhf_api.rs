use axum::Json;

/// Accept any JSON for RLHF feedback — the UI sends {message_id, score, phase}
/// but the original handler expected ResonanceFeedbackEvent. Accept both shapes.
pub async fn submit_resonance_feedback(
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // Log whatever we receive — this is a stub that will be wired to the Great Recycler
    let score = payload.get("score").and_then(|v| v.as_i64()).unwrap_or(0);
    let phase = payload
        .get("phase")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let msg_id = payload
        .get("message_id")
        .and_then(|v| v.as_str())
        .unwrap_or("?");

    tracing::info!(
        "RLHF feedback: msg={}, score={}, phase={}",
        msg_id,
        score,
        phase
    );

    Json(
        serde_json::json!({"status": "success", "message": "Feedback recorded for the Great Recycler"}),
    )
}
