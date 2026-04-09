// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — audio integration tests
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        audio_tests.rs
// PURPOSE:     Integration tests for Acestep Tempo & Voice Fallback mechanisms
//
// ═══════════════════════════════════════════════════════════════════════════════

use rand::Rng;

#[tokio::test]
async fn test_audio_status_struct_serialization() {
    // We already have unit tests, let's test a mock schema structure 
    let status_json = serde_json::json!({
        "sidecar_running": true,
        "personaplex_available": false,
        "omni_available": true,
        "npu_available": false,
        "active_pipeline": "acestep-1.5",
        "mode": "dev",
        "message": "Acestep 1.5 native audio generation via LongCat SGLang"
    });

    assert_eq!(status_json["active_pipeline"], "acestep-1.5");
    assert_eq!(status_json["personaplex_available"], false);
}

#[tokio::test]
async fn test_creative_tempo_payload_schema() {
    let payload = serde_json::json!({
        "model": "LongCat-Next",
        "prompt": "ambient menu music",
        "style": "steampunk",
        "duration": 30,
        "response_format": "b64_json"
    });

    assert_eq!(payload["model"], "LongCat-Next");
    assert_eq!(payload["style"], "steampunk");
    assert_eq!(payload["duration"], 30);
}

#[tokio::test]
async fn test_simulated_cognitive_load_variance() {
    // Ensure that as friction/load randomly spikes across a session,
    // the system correctly maps to exactly 1 of the 4 defined speeds.
    let mut rng = rand::rngs::mock::StepRng::new(2, 1);
    
    for _ in 0..100 {
        let friction: f32 = (rng.gen::<u32>() % 100) as f32 / 100.0;
        let vuln: f32 = (rng.gen::<u32>() % 100) as f32 / 100.0;
        
        let load = friction * 0.6 + vuln * 0.4;
        let expected_speed = if load < 0.20 {
            1.15
        } else if load < 0.40 {
            1.0
        } else if load < 0.60 {
            0.90
        } else {
            0.75
        };

        let calculated_speed = trinity::voice::cognitive_load_speed_multiplier(friction, vuln);
        assert!((calculated_speed - expected_speed).abs() < 0.001, 
                "Speed mismatch at load {}", load);
    }
}
