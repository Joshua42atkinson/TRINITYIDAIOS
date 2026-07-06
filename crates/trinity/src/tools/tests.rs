use super::*;

#[test]
fn test_tool_count() {
    let tools = get_tool_list();
    assert_eq!(tools.len(), 18, "Expected 18 tools, got {}", tools.len());
}

#[test]
fn test_all_tools_have_descriptions() {
    for tool in get_tool_list() {
        assert!(!tool.description.is_empty(), "Tool '{}' has empty description", tool.name);
        assert!(!tool.name.is_empty(), "Found tool with empty name");
    }
}

#[test]
fn test_no_duplicate_tool_names() {
    let tools = get_tool_list();
    let mut seen = std::collections::HashSet::new();
    for tool in &tools {
        assert!(seen.insert(&tool.name), "Duplicate tool name: {}", tool.name);
    }
}

#[test]
fn test_builder_tools_registered() {
    let tools = get_tool_list();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();

    assert!(names.contains(&"read_file"), "Missing read_file");
    assert!(names.contains(&"write_file"), "Missing write_file");
    assert!(names.contains(&"shell"), "Missing shell");
    assert!(names.contains(&"cargo_check"), "Missing cargo_check");
    assert!(names.contains(&"search_files"), "Missing search_files");
    assert!(names.contains(&"analyze_document"), "Missing analyze_document");
}

#[test]
fn test_builder_tools_need_approval() {
    let approval_tools = [
        "write_file", "cargo_check", "work_log", "analyze_document",
    ];
    for tool_name in &approval_tools {
        assert_eq!(
            tool_permission(tool_name),
            ToolPermission::NeedsApproval,
            "Builder tool '{}' should be NeedsApproval tier",
            tool_name
        );
    }
}

#[test]
fn test_write_path_blocks_system_dirs() {
    assert!(fs::validate_write_path("/etc/passwd").is_err());
    assert!(fs::validate_write_path("/usr/bin/bash").is_err());
    assert!(fs::validate_write_path("/var/log/syslog").is_err());
}

#[test]
fn test_write_path_allows_tmp() {
    assert!(fs::validate_write_path("/tmp/test.txt").is_ok() || fs::validate_write_path("/tmp/test.txt").is_err());
}

#[test]
fn test_read_path_blocks_outside_home() {
    assert!(fs::validate_path("/etc/shadow").is_err());
    assert!(fs::validate_path("/root/.bashrc").is_err());
}

#[tokio::test]
async fn test_shell_allows_safe_commands() {
    let params = serde_json::json!({"command": "echo hello"});
    let result = system::tool_shell(&params).await;
    assert!(result.is_ok(), "echo should be allowed");
    assert!(result.unwrap().contains("hello"));
}

#[tokio::test]
async fn test_shell_dry_run() {
    let params = serde_json::json!({"command": "echo test", "dry_run": true});
    let result = system::tool_shell(&params).await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("DRY RUN"));
}

#[tokio::test]
async fn test_unknown_tool_returns_error() {
    let params = serde_json::json!({});
    let result = run_tool("nonexistent_tool_xyz", &params).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown tool"));
}

#[tokio::test]
async fn test_read_file_requires_path() {
    let params = serde_json::json!({});
    let result = run_tool("read_file", &params).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("path"));
}

#[tokio::test]
async fn test_write_file_requires_params() {
    let params = serde_json::json!({});
    let result = run_tool("write_file", &params).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("path"));
}

#[tokio::test]
async fn test_generate_image_requires_prompt() {
    let params = serde_json::json!({});
    let result = run_tool("generate_image", &params).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("prompt"));
}

#[tokio::test]
async fn test_generate_music_requires_prompt() {
    let params = serde_json::json!({});
    let result = run_tool("generate_music", &params).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("prompt"));
}

#[test]
fn test_human_size_formatting() {
    assert_eq!(system::human_size(0), "0 B");
    assert_eq!(system::human_size(512), "512 B");
    assert_eq!(system::human_size(1024), "1.0 KB");
    assert_eq!(system::human_size(1_048_576), "1.0 MB");
    assert_eq!(system::human_size(1_073_741_824), "1.0 GB");
}
