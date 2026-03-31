//! Trinity Self-Work MCP Extensions
//!
//! Enables Trinity to perform self-improvement tasks through MCP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::fs;
use tracing::info;

use crate::TrinityMcpServer;

impl TrinityMcpServer {
    /// Execute a self-work workflow
    pub async fn execute_self_work(&self, workflow: &SelfWorkWorkflow) -> Result<SelfWorkResult> {
        info!("Executing self-work workflow: {}", workflow.name);

        let mut results = Vec::new();
        let mut status = "in_progress".to_string();

        for step in &workflow.steps {
            let step_result = self.execute_workflow_step(step).await?;

            if step_result.status != "success" {
                status = "failed".to_string();
                results.push(step_result);
                break;
            }
            results.push(step_result);
        }

        if status == "in_progress" {
            status = "completed".to_string();
        }

        Ok(SelfWorkResult {
            workflow_id: workflow.id.clone(),
            status,
            results,
            execution_time: chrono::Utc::now(),
        })
    }

    /// Execute a single workflow step
    async fn execute_workflow_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        match step.step_type.as_str() {
            "analyze_code" => self.analyze_code_step(step).await,
            "modify_code" => self.modify_code_step(step).await,
            "test_code" => self.test_code_step(step).await,
            "search_docs" => self.search_docs_step(step).await,
            "compile_check" => self.compile_check_step(step).await,
            "update_status" => self.update_status_step(step).await,
            _ => Ok(StepResult {
                step_id: step.id.clone(),
                status: "failed".to_string(),
                message: format!("Unknown step type: {}", step.step_type),
                output: None,
            }),
        }
    }

    /// Analyze code for potential improvements
    async fn analyze_code_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let file_path = step
            .parameters
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path parameter"))?;

        // Read file content
        let content = fs::read_to_string(file_path).await?;

        // Search for relevant patterns in documentation
        let search_results = self
            .search_documentation(
                &format!(
                    "best practices {}",
                    step.parameters
                        .get("analysis_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("code quality")
                ),
                5,
            )
            .await?;

        // Simple analysis based on documentation
        let mut suggestions = Vec::new();
        for result in search_results {
            if result.similarity > 0.7 {
                suggestions.push(format!("Consider: {}", result.content));
            }
        }

        Ok(StepResult {
            step_id: step.id.clone(),
            status: "success".to_string(),
            message: format!(
                "Analyzed {} with {} suggestions",
                file_path,
                suggestions.len()
            ),
            output: Some(serde_json::json!({
                "suggestions": suggestions,
                "file_size": content.len()
            })),
        })
    }

    /// Modify code based on analysis
    async fn modify_code_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let file_path = step
            .parameters
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path parameter"))?;

        let modifications = step
            .parameters
            .get("modifications")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("Missing modifications parameter"))?;

        // Read current file
        let mut content = fs::read_to_string(file_path).await?;

        // Apply modifications (simplified)
        for mod_item in modifications {
            if let (Some(old_text), Some(new_text)) = (
                mod_item.get("old").and_then(|v| v.as_str()),
                mod_item.get("new").and_then(|v| v.as_str()),
            ) {
                content = content.replace(old_text, new_text);
            }
        }

        // Write back
        fs::write(file_path, content).await?;

        Ok(StepResult {
            step_id: step.id.clone(),
            status: "success".to_string(),
            message: format!(
                "Modified {} with {} changes",
                file_path,
                modifications.len()
            ),
            output: Some(serde_json::json!({
                "modifications_applied": modifications.len()
            })),
        })
    }

    /// Test code changes
    async fn test_code_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let test_target = step
            .parameters
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("all");

        let output = if test_target == "all" {
            Command::new("cargo").args(["test"]).output()?
        } else {
            Command::new("cargo")
                .args(["test", "--package", test_target])
                .output()?
        };

        let status = if output.status.success() {
            "success"
        } else {
            "failed"
        };

        Ok(StepResult {
            step_id: step.id.clone(),
            status: status.to_string(),
            message: format!(
                "Test results for {}: {}",
                test_target,
                if status == "success" {
                    "All passed"
                } else {
                    "Some failed"
                }
            ),
            output: Some(serde_json::json!({
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr)
            })),
        })
    }

    /// Search documentation for guidance
    async fn search_docs_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let query = step
            .parameters
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

        let results = self.search_documentation(query, 10).await?;

        Ok(StepResult {
            step_id: step.id.clone(),
            status: "success".to_string(),
            message: format!("Found {} relevant documents", results.len()),
            output: Some(serde_json::json!({
                "results": results
            })),
        })
    }

    /// Check compilation status
    async fn compile_check_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let package = step.parameters.get("package").and_then(|v| v.as_str());

        let output = if let Some(pkg) = package {
            Command::new("cargo")
                .args(["check", "--package", pkg])
                .output()?
        } else {
            Command::new("cargo").arg("check").output()?
        };

        let status = if output.status.success() {
            "success"
        } else {
            "failed"
        };
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Count errors
        let error_count = stderr.matches("error[").count();

        Ok(StepResult {
            step_id: step.id.clone(),
            status: status.to_string(),
            message: format!("Compilation check: {} errors found", error_count),
            output: Some(serde_json::json!({
                "error_count": error_count,
                "stderr": &stderr[..stderr.len().min(1000)]
            })),
        })
    }

    /// Update implementation status in database
    async fn update_status_step(&self, step: &WorkflowStep) -> Result<StepResult> {
        let feature = step
            .parameters
            .get("feature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing feature parameter"))?;

        let status = step
            .parameters
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("in_progress");

        let description = step.parameters.get("description").and_then(|v| v.as_str());

        self.update_implementation_status(
            feature,
            status,
            description,
            step.parameters.get("evidence").cloned().unwrap_or_default(),
        )
        .await?;

        Ok(StepResult {
            step_id: step.id.clone(),
            status: "success".to_string(),
            message: format!("Updated status for {} to {}", feature, status),
            output: Some(serde_json::json!({
                "feature": feature,
                "new_status": status
            })),
        })
    }
}

/// Self-work workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfWorkWorkflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
}

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub step_type: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfWorkResult {
    pub workflow_id: String,
    pub status: String,
    pub results: Vec<StepResult>,
    pub execution_time: chrono::DateTime<chrono::Utc>,
}

/// Step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub status: String,
    pub message: String,
    pub output: Option<serde_json::Value>,
}
