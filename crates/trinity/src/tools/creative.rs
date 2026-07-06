use tracing::info;

pub async fn tool_generate_image(params: &serde_json::Value) -> Result<String, String> {
    let prompt = params
        .get("prompt")
        .and_then(|p| p.as_str())
        .ok_or("Missing 'prompt' parameter")?;
    let width = params
        .get("width")
        .and_then(|w| w.as_u64())
        .unwrap_or(1024) as u32;
    let height = params
        .get("height")
        .and_then(|h| h.as_u64())
        .unwrap_or(1024) as u32;

    info!("🎨 Generating image via ComfyUI: {} ({}x{})", prompt, width, height);

    // Call creative.rs endpoint internally
    let client = &*crate::http::LONG;
    let body = serde_json::json!({ "prompt": prompt, "width": width, "height": height });
    let response = client
        .post("http://127.0.0.1:3000/api/creative/image")
        .json(&body)
        .timeout(std::time::Duration::from_secs(300))
        .send()
        .await
        .map_err(|e| format!("Image generation failed to connect: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Image generation returned error: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let path = result["image_path"].as_str().unwrap_or("unknown");
    Ok(format!("Image generated successfully: {}", path))
}

pub async fn tool_generate_music(params: &serde_json::Value) -> Result<String, String> {
    let prompt = params
        .get("prompt")
        .and_then(|p| p.as_str())
        .ok_or("Missing 'prompt' parameter")?;
    let style = params
        .get("style")
        .and_then(|s| s.as_str())
        .unwrap_or("lofi");
    let duration_secs = params
        .get("duration_secs")
        .and_then(|d| d.as_u64())
        .unwrap_or(30);

    info!("🎵 Generating music: {} (style: {}, {}s)", prompt, style, duration_secs);

    // Call creative.rs tempo endpoint via internal HTTP
    let client = &*crate::http::LONG;
    let body = serde_json::json!({
        "prompt": prompt,
        "style": style,
        "duration_secs": duration_secs,
    });

    let response = client
        .post("http://127.0.0.1:3000/api/creative/tempo")
        .json(&body)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| format!("Music generation failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Music generation error: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let audio_path = result["audio_path"]
        .as_str()
        .unwrap_or("unknown");
    Ok(format!("Music generated: {}", audio_path))
}

pub async fn tool_update_vibe(params: &serde_json::Value) -> Result<String, String> {
    let client = &*crate::http::LONG;
    let visual_style_str = params.get("visual_style").and_then(|v| v.as_str());
    let music_style_str = params.get("music_style").and_then(|v| v.as_str());
    let narrator_mood_str = params.get("narrator_mood").and_then(|v| v.as_str());

    // 1. GET current sheet via internal HTTP wrapper pattern
    let sheet_json: serde_json::Value = client.get("http://127.0.0.1:3000/api/character")
        .send().await.map_err(|e| format!("Failed to fetch sheet: {}", e))?
        .json().await.map_err(|e| format!("Failed to parse sheet: {}", e))?;

    // 2. Extract specific segments to update cleanly without destroying nested formats
    let mut creative_cfg = sheet_json.get("creative_config").cloned().unwrap_or(serde_json::json!({}));
    let mut audio_prefs = sheet_json.get("audio_preferences").cloned().unwrap_or(serde_json::json!({}));

    // Capitalize correctly for enums (e.g., 'warm' -> 'Warm')
    let capitalize = |s: &str| -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    };

    if let Some(vs) = visual_style_str {
        creative_cfg["visual_style"] = serde_json::json!(capitalize(vs));
    }
    if let Some(ms) = music_style_str {
        creative_cfg["music_style"] = serde_json::json!(capitalize(ms));
    }
    if let Some(nm) = narrator_mood_str {
        audio_prefs["narrator_mood"] = serde_json::json!(capitalize(nm));
    }

    // 3. POST merged updates via internal HTTP
    let payload = serde_json::json!({
        "creative_config": creative_cfg,
        "audio_preferences": audio_prefs
    });

    client.post("http://127.0.0.1:3000/api/character")
        .json(&payload)
        .send().await.map_err(|e| format!("Failed to post sheet updates: {}", e))?;

    Ok(format!("Dynamic Vibe successfully set. Visuals: {} | Music: {} | Mood: {}", 
        creative_cfg.get("visual_style").and_then(|v| v.as_str()).unwrap_or("Unchanged"),
        creative_cfg.get("music_style").and_then(|v| v.as_str()).unwrap_or("Unchanged"),
        audio_prefs.get("narrator_mood").and_then(|v| v.as_str()).unwrap_or("Unchanged")))
}
