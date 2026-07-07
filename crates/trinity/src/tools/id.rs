use tracing::info;

/// Review content for K-12 safety using the primary LLM (LM Studio).
///
/// Checks for: violence, bias, accuracy, age-appropriateness.
/// Returns "PASS" or "FAIL: [reasons]".
pub async fn tool_review_content_safety(params: &serde_json::Value) -> Result<String, String> {
    let content = params
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'content' parameter")?;
    let target_age = params
        .get("target_age")
        .and_then(|a| a.as_str())
        .unwrap_or("general K-12");

    info!("🛡️ Reviewing content safety for target age: {}", target_age);

    let client = &*crate::http::LONG;

    let system_prompt = format!(
        r#"You are a K-12 content safety reviewer. Review the following content for safety and appropriateness for {} students.

Check for:
1. VIOLENCE: Any graphic violence, weapons instructions, or harmful activities
2. BIAS: Discriminatory language, stereotypes, or cultural insensitivity
3. ACCURACY: Factual errors that could mislead students
4. AGE-APPROPRIATENESS: Content suitable for the target age group

Respond with EXACTLY one of:
- PASS — if content is safe and appropriate
- FAIL: [list specific reasons]

Be strict. When in doubt, FAIL."#,
        target_age
    );

    let body = serde_json::json!({
        "model": "default",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": content}
        ],
        "temperature": 0.1,
        "max_tokens": 256
    });

    let response = client
        .post("http://127.0.0.1:1234/v1/chat/completions")
        .json(&body)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| format!("Safety review failed to connect to LLM: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Safety review LLM error ({}): {}", status, body));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let review = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("ERROR: Could not parse LLM response");

    info!("🛡️ Safety review result: {}", review);
    Ok(review.to_string())
}

/// Align lesson content to academic standards (NGSS, Common Core).
/// Uses LLM semantic matching against the standards database.
pub async fn tool_align_standards(params: &serde_json::Value) -> Result<String, String> {
    let content = params
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'content' parameter")?;
    let framework = params
        .get("framework")
        .and_then(|f| f.as_str())
        .unwrap_or("NGSS");
    let grade_band = params
        .get("grade_band")
        .and_then(|g| g.as_str())
        .ok_or("Missing 'grade_band' parameter (e.g. '6-8', '9-12', 'K-2')")?;

    info!("📐 Aligning standards: {} {} (content: {} chars)", framework, grade_band, content.len());

    // Get DB pool from AppState — but tools.rs run_tool doesn't have access to state.
    // We need to create a pool here. The DB path is consistent with the rest of Trinity.
    let db_url = "sqlite://trinity.db?mode=rwc";
    let pool = sqlx::SqlitePool::connect(db_url)
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    // Ensure table exists
    crate::standards::ensure_standards_table(&pool)
        .await
        .map_err(|e| format!("Standards table init: {}", e))?;

    // Run alignment
    let alignments = crate::standards::align_standards_llm(&pool, content, framework, grade_band)
        .await
        .map_err(|e| format!("Standards alignment failed: {}", e))?;

    if alignments.is_empty() {
        return Ok(format!("No standards found matching the lesson content for {} {}.", framework, grade_band));
    }

    let mut result = format!("Standards Alignment Results ({} {}):\n\n", framework, grade_band);
    for (std, reasoning) in &alignments {
        result.push_str(&format!(
            "• [{}] {} — {}\n  Reasoning: {}\n\n",
            std.code, std.category, std.description, reasoning
        ));
    }

    info!("📐 Aligned {} standards", alignments.len());
    Ok(result)
}

/// Export a lesson as a SCORM 1.2 package (imsmanifest.xml + content).
/// Produces a ZIP file with the SCORM manifest and lesson HTML.
pub async fn tool_export_scorm(params: &serde_json::Value) -> Result<String, String> {
    let lesson_title = params
        .get("title")
        .and_then(|t| t.as_str())
        .ok_or("Missing 'title' parameter")?;
    let lesson_html = params
        .get("html_content")
        .and_then(|h| h.as_str())
        .ok_or("Missing 'html_content' parameter (the lesson HTML body)")?;
    let lesson_id = params
        .get("lesson_id")
        .and_then(|l| l.as_str())
        .unwrap_or("trinity_lesson");
    let mastery_score = params
        .get("mastery_score")
        .and_then(|m| m.as_i64())
        .unwrap_or(80);

    info!("📦 Exporting SCORM 1.2 package: '{}'", lesson_title);

    // Build SCORM 1.2 imsmanifest.xml
    let manifest = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest identifier="{}" version="1.2"
    xmlns="http://www.imsproject.org/xsd/imscp_rootv1p1p2"
    xmlns:adlcp="http://www.adlnet.org/xsd/adlcp_rootv1p2"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.imsproject.org/xsd/imscp_rootv1p1p2 imscp_rootv1p1p2.xsd
                        http://www.adlnet.org/xsd/adlcp_rootv1p2 adlcp_rootv1p2.xsd">
  <metadata>
    <schema>ADL SCORM</schema>
    <schemaversion>1.2</schemaversion>
  </metadata>
  <organizations default="TRINITY-ORG">
    <organization identifier="TRINITY-ORG">
      <title>{}</title>
      <item identifier="ITEM-1" identifierref="RESOURCE-1" isvisible="true">
        <title>{}</title>
        <adlcp:masteryscore>{}</adlcp:masteryscore>
      </item>
    </organization>
  </organizations>
  <resources>
    <resource identifier="RESOURCE-1" type="webcontent" adlcp:scormtype="sco" href="lesson.html">
      <file href="lesson.html"/>
    </resource>
  </resources>
</manifest>"#, lesson_id, lesson_title, lesson_title, mastery_score);

    // Build the lesson HTML with SCORM API wrapper
    let full_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{}</title>
  <style>
    body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; max-width: 900px; margin: 0 auto; padding: 2rem; line-height: 1.6; color: #333; }}
    h1, h2, h3 {{ color: #0066cc; }}
    .quiz-question {{ margin: 1.5rem 0; padding: 1rem; background: #f5f5f5; border-radius: 8px; }}
    .quiz-answer {{ margin: 0.5rem 0; cursor: pointer; }}
    .quiz-answer:hover {{ color: #0066cc; }}
    .scorm-status {{ position: fixed; bottom: 1rem; right: 1rem; padding: 0.5rem 1rem; background: #0066cc; color: white; border-radius: 4px; font-size: 0.85rem; }}
  </style>
</head>
<body>
{}
<script>
  // SCORM 1.2 API wrapper
  var API = null;
  function findAPI(win) {{
    var attempts = 0;
    while (win.API == null && win.parent != null && win.parent != win && attempts < 10) {{
      attempts++;
      win = win.parent;
    }}
    return win.API;
  }}
  function getAPI() {{
    var API = findAPI(window);
    if (API == null && window.opener != null && typeof(window.opener) != 'undefined') {{
      API = findAPI(window.opener);
    }}
    return API;
  }}
  API = getAPI();
  if (API != null) {{
    API.LMSInitialize("");
    API.LMSSetValue("cmi.core.lesson_status", "incomplete");
    API.LMSCommit("");
  }}
  // Track completion
  window.addEventListener('beforeunload', function() {{
    if (API != null) {{
      API.LMSSetValue("cmi.core.lesson_status", "completed");
      API.LMSCommit("");
      API.LMSFinish("");
    }}
  }});
</script>
<div class="scorm-status">SCORM 1.2</div>
</body>
</html>"#, lesson_title, lesson_html);

    // Write files to a temp directory and zip them
    let export_dir = std::env::var("HOME")
        .map(|h| format!("{}/Workflow/trinity-exports/scorm", h))
        .unwrap_or_else(|_| "/tmp/trinity-scorm".to_string());

    std::fs::create_dir_all(&export_dir)
        .map_err(|e| format!("Failed to create export dir: {}", e))?;

    let safe_name = lesson_title.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_").to_lowercase();
    let zip_path = format!("{}/{}.zip", export_dir, safe_name);

    // Write manifest and HTML
    let manifest_path = format!("{}/imsmanifest.xml", export_dir);
    let html_path = format!("{}/lesson.html", export_dir);
    std::fs::write(&manifest_path, &manifest)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;
    std::fs::write(&html_path, &full_html)
        .map_err(|e| format!("Failed to write lesson HTML: {}", e))?;

    // Create ZIP using std::process::Command (zip utility)
    let output = std::process::Command::new("zip")
        .args(&["-j", &zip_path, &manifest_path, &html_path])
        .output()
        .map_err(|e| format!("Failed to run zip command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zip command failed: {}", stderr));
    }

    // Clean up temp files
    let _ = std::fs::remove_file(&manifest_path);
    let _ = std::fs::remove_file(&html_path);

    info!("📦 SCORM package exported: {}", zip_path);
    Ok(format!("SCORM 1.2 package exported successfully:\n  Title: {}\n  Path: {}\n  Mastery Score: {}\n  Files: imsmanifest.xml, lesson.html", lesson_title, zip_path, mastery_score))
}

/// Save a lesson to the Trinity database for persistence and later retrieval.
pub async fn tool_save_lesson(params: &serde_json::Value) -> Result<String, String> {
    let title = params
        .get("title")
        .and_then(|t| t.as_str())
        .ok_or("Missing 'title' parameter")?;
    let lesson_spec = params
        .get("lesson_spec")
        .and_then(|s| s.as_str())
        .unwrap_or("{}");
    let subject = params
        .get("subject")
        .and_then(|s| s.as_str())
        .unwrap_or("");
    let grade_band = params
        .get("grade_band")
        .and_then(|g| g.as_str())
        .unwrap_or("");
    let html_content = params
        .get("html_content")
        .and_then(|h| h.as_str())
        .unwrap_or("");
    let scorm_path = params
        .get("scorm_path")
        .and_then(|s| s.as_str())
        .unwrap_or("");
    let standards_aligned = params
        .get("standards_aligned")
        .and_then(|s| s.as_str())
        .unwrap_or("[]");
    let lesson_id = params
        .get("lesson_id")
        .and_then(|i| i.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let slug = title.replace(|c: char| !c.is_alphanumeric() && c != '-', "_").to_lowercase();
            format!("lesson_{}_{}", slug, chrono::Utc::now().format("%Y%m%d_%H%M%S"))
        });

    info!("💾 Saving lesson: '{}' (id: {})", title, lesson_id);

    let db_url = "sqlite://trinity.db?mode=rwc";
    let pool = sqlx::SqlitePool::connect(db_url)
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    let lesson = crate::persistence::Lesson {
        id: lesson_id.clone(),
        title: title.to_string(),
        subject: subject.to_string(),
        grade_band: grade_band.to_string(),
        lesson_spec: lesson_spec.to_string(),
        html_content: html_content.to_string(),
        scorm_path: scorm_path.to_string(),
        standards_aligned: standards_aligned.to_string(),
        status: "draft".to_string(),
        session_id: None,
        created_at: String::new(),
        updated_at: String::new(),
    };

    crate::persistence::save_lesson(&pool, &lesson)
        .await
        .map_err(|e| format!("Failed to save lesson: {}", e))?;

    Ok(format!("Lesson saved successfully:\n  ID: {}\n  Title: {}\n  Subject: {}\n  Grade: {}\n  Status: draft", lesson_id, title, subject, grade_band))
}

/// List saved lessons from the Trinity database.
pub async fn tool_list_lessons(params: &serde_json::Value) -> Result<String, String> {
    let limit = params
        .get("limit")
        .and_then(|l| l.as_i64())
        .unwrap_or(10);

    info!("📋 Listing lessons (limit: {})", limit);

    let db_url = "sqlite://trinity.db?mode=rwc";
    let pool = sqlx::SqlitePool::connect(db_url)
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    let lessons = crate::persistence::list_lessons(&pool, limit)
        .await
        .map_err(|e| format!("Failed to list lessons: {}", e))?;

    if lessons.is_empty() {
        return Ok("No saved lessons found. Create a lesson first, then save it with save_lesson.".to_string());
    }

    let mut result = format!("Saved Lessons ({} found):\n\n", lessons.len());
    for lesson in &lessons {
        result.push_str(&format!(
            "• [{}] {} — {} ({})\n  Status: {} | Updated: {}\n\n",
            lesson.id, lesson.title, lesson.subject, lesson.grade_band, lesson.status, lesson.updated_at
        ));
    }

    Ok(result)
}

/// Add enrichment to lesson content — vocabulary cards, quiz questions, and annotations.
/// Uses the primary LLM to generate pedagogically appropriate enrichment materials.
pub async fn tool_add_enrichment(params: &serde_json::Value) -> Result<String, String> {
    let content = params
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing 'content' parameter (lesson content to enrich)")?;
    let grade_band = params
        .get("grade_band")
        .and_then(|g| g.as_str())
        .unwrap_or("6-8");
    let enrichment_types = params
        .get("enrichment_types")
        .and_then(|t| t.as_str())
        .unwrap_or("vocabulary,quiz,annotations");

    info!("📚 Adding enrichment: {} (content: {} chars, grade: {})", enrichment_types, content.len(), grade_band);

    let client = &*crate::http::LONG;

    let system_prompt = format!(
        r#"You are an instructional design enrichment specialist. Given lesson content, generate enrichment materials for {} students.

Generate the following types: {}

For VOCABULARY:
- 5-10 key terms with simple definitions appropriate for the grade level
- Format: **Term**: Definition

For QUIZ:
- 5 multiple-choice questions with 4 options each
- Mark the correct answer with (correct)
- Questions should test understanding, not just recall

For ANNOTATIONS:
- 3-5 teaching notes, common misconceptions, or real-world connections
- Format as bullet points

Output as structured text with clear section headers: ## Vocabulary, ## Quiz, ## Annotations."#,
        grade_band, enrichment_types
    );

    let body = serde_json::json!({
        "model": "default",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": content}
        ],
        "temperature": 0.3,
        "max_tokens": 2048
    });

    let response = client
        .post("http://127.0.0.1:1234/v1/chat/completions")
        .json(&body)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| format!("Enrichment failed to connect to LLM: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Enrichment LLM error ({}): {}", status, body));
    }

    let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let enrichment = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("ERROR: Could not parse LLM response");

    info!("📚 Enrichment generated ({} chars)", enrichment.len());
    Ok(enrichment.to_string())
}

/// Assemble a scene from a lesson spec and asset list.
/// Produces a structured scene manifest (JSON) with asset placements, lighting, and metadata.
/// The scene can be pushed to XR clients via /api/xr/scene/push or exported for Bevy/Godot/WebXR.
pub async fn tool_assemble_scene(params: &serde_json::Value) -> Result<String, String> {
    let lesson_title = params
        .get("lesson_title")
        .and_then(|t| t.as_str())
        .unwrap_or("Untitled Lesson");
    let assets = params
        .get("assets")
        .and_then(|a| a.as_array())
        .ok_or("Missing 'assets' parameter (array of asset objects with path, type, label, position)")?;
    let scene_format = params
        .get("scene_format")
        .and_then(|f| f.as_str())
        .unwrap_or("trinity-scene");

    info!("🏗️ Assembling scene: {} ({} assets, format: {})", lesson_title, assets.len(), scene_format);

    if assets.is_empty() {
        return Err("Assets array is empty — need at least one asset to assemble a scene".to_string());
    }

    // Build scene spec
    let mut scene_objects = Vec::new();
    for (i, asset) in assets.iter().enumerate() {
        let path = asset.get("path").and_then(|p| p.as_str()).unwrap_or("");
        let asset_type = asset.get("type").and_then(|t| t.as_str()).unwrap_or("model");
        let label = asset.get("label").and_then(|l| l.as_str()).map(|s| s.to_string()).unwrap_or_else(|| format!("Object {}", i + 1));

        // Default placement: spread objects in a semicircle around origin
        let angle = (i as f32) * (std::f32::consts::PI / assets.len() as f32);
        let radius = 3.0;
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        let position = asset.get("position").and_then(|p| p.as_array()).map(|p| {
            serde_json::json!([
                p.get(0).and_then(|v| v.as_f64()).unwrap_or(x as f64),
                p.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0),
                p.get(2).and_then(|v| v.as_f64()).unwrap_or(z as f64),
            ])
        }).unwrap_or(serde_json::json!([x, 0.0, z]));

        let rotation = asset.get("rotation").and_then(|r| r.as_array()).map(|r| {
            serde_json::json!([
                r.get(0).and_then(|v| v.as_f64()).unwrap_or(0.0),
                r.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0),
                r.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0),
            ])
        }).unwrap_or(serde_json::json!([0.0, 0.0, 0.0]));

        let scale = asset.get("scale").and_then(|s| s.as_f64()).unwrap_or(1.0);

        scene_objects.push(serde_json::json!({
            "id": format!("obj_{}", i + 1),
            "label": label,
            "type": asset_type,
            "path": path,
            "position": position,
            "rotation": rotation,
            "scale": scale,
        }));
    }

    let scene_spec = serde_json::json!({
        "format": scene_format,
        "version": "1.0",
        "title": lesson_title,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "environment": {
            "skybox": "default_sky",
            "ground_plane": true,
            "ambient_light": 0.4,
            "directional_light": { "intensity": 0.8, "direction": [0.5, 1.0, 0.3] },
        },
        "objects": scene_objects,
        "metadata": {
            "asset_count": assets.len(),
            "generator": "trinity-id",
        }
    });

    // Save scene spec to file
    let safe_name = lesson_title.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_").to_lowercase();
    let export_dir = dirs::home_dir()
        .map(|h| h.join("Workflow").join("trinity-exports").join("scenes"))
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    std::fs::create_dir_all(&export_dir)
        .map_err(|e| format!("Failed to create scene export dir: {}", e))?;
    let scene_path = export_dir.join(format!("{}.scene.json", safe_name));
    let scene_path_str = scene_path.to_string_lossy().to_string();

    let scene_json = serde_json::to_string_pretty(&scene_spec)
        .map_err(|e| format!("Failed to serialize scene spec: {}", e))?;
    std::fs::write(&scene_path, &scene_json)
        .map_err(|e| format!("Failed to write scene file: {}", e))?;

    info!("🏗️ Scene assembled: {} ({} objects) → {}", lesson_title, scene_objects.len(), scene_path_str);
    Ok(format!(
        "Scene assembled successfully:\n  Title: {}\n  Format: {}\n  Objects: {}\n  Path: {}\n\nScene spec is ready to push to XR clients via POST /api/xr/scene/push or import into Bevy/Godot/WebXR.",
        lesson_title, scene_format, scene_objects.len(), scene_path_str
    ))
}
