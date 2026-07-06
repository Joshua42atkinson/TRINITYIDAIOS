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
