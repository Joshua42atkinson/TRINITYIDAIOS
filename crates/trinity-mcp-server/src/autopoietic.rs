// Trinity AI Agent System
// Copyright (c) Joshua
// Shared under license for Ask_Pete (Purdue University)

//! Autopoietic Loop - Self-Modifying Code Engine
//!
//! ## Philosophy (The Soul)
//! "The Soul is the mechanism of self-creation. An agent that cannot
//!  modify its own source code is merely a tool. The autopoietic loop
//!  transforms Trinity from software into a living system."

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for the autopoietic loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopoieticConfig {
    /// Root directory of the Trinity source code
    pub source_root: PathBuf,
    /// Directory for staging mutations
    pub staging_dir: PathBuf,
    /// Directory for version backups
    pub backup_dir: PathBuf,
    /// Number of backup versions to keep
    pub max_backups: usize,
    /// Whether to run tests before accepting mutations
    pub require_tests: bool,
    /// Google Drive folder for cloud backup (if configured)
    pub cloud_backup_path: Option<PathBuf>,
    /// Files that are NEVER allowed to be modified
    pub immutable_files: Vec<String>,
    /// Maximum consecutive failures before halting
    pub max_failures: u32,
}

impl Default for AutopoieticConfig {
    fn default() -> Self {
        let base = PathBuf::from("/home/joshua/Workflow/desktop_trinity/trinity-genesis");
        Self {
            source_root: base.clone(),
            staging_dir: base.parent().unwrap().join(".trinity_staging"),
            backup_dir: base.parent().unwrap().join(".trinity_backup"),
            max_backups: 10,
            require_tests: false,
            cloud_backup_path: None,
            immutable_files: vec![
                "safety.rs".to_string(),
                "autopoietic.rs".to_string(),
                "Cargo.lock".to_string(),
            ],
            max_failures: 3,
        }
    }
}

// ============================================================================
// Mutation Request
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRequest {
    pub target_file: String,
    pub mutation_type: MutationType,
    pub description: String,
    pub code: Option<String>,
    pub find_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationType {
    Insert { location: String },
    Replace,
    CreateFile,
    DeleteFile,
    Append,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationResult {
    pub success: bool,
    pub backup_version: Option<u64>,
    pub compiler_output: Option<String>,
    pub test_output: Option<String>,
    pub error: Option<String>,
}

// ============================================================================
// Autopoietic Engine
// ============================================================================

pub struct AutopoieticEngine {
    config: AutopoieticConfig,
    failure_count: u32,
    current_version: u64,
}

impl AutopoieticEngine {
    pub fn new(config: AutopoieticConfig) -> Result<Self> {
        let _ = std::fs::create_dir_all(&config.staging_dir);
        let _ = std::fs::create_dir_all(&config.backup_dir);
        let current_version = Self::find_latest_version(&config.backup_dir)?;

        info!("Autopoietic engine initialized at v{}", current_version);
        Ok(Self {
            config,
            failure_count: 0,
            current_version,
        })
    }

    pub fn current_version(&self) -> u64 {
        self.current_version
    }

    fn find_latest_version(backup_dir: &Path) -> Result<u64> {
        let mut max_version = 0;
        if backup_dir.exists() {
            for entry in std::fs::read_dir(backup_dir)? {
                let _entry = entry?; // Renamed to _entry as it's not directly used after this line
                if let Some(name) = _entry.file_name().to_str() {
                    if let Some(v_str) = name.strip_prefix("v") {
                        if let Ok(v) = v_str.parse::<u64>() {
                            max_version = max_version.max(v);
                        }
                    }
                }
            }
        }
        Ok(max_version)
    }

    fn is_mutable(&self, file: &str) -> bool {
        !self.config.immutable_files.iter().any(|f| file.ends_with(f))
    }

    pub fn execute(&mut self, request: MutationRequest) -> Result<MutationResult> {
        if !self.is_mutable(&request.target_file) {
            return Ok(MutationResult {
                success: false,
                backup_version: None,
                compiler_output: None,
                test_output: None,
                error: Some(format!("File '{}' is immutable", request.target_file)),
            });
        }

        if self.failure_count >= self.config.max_failures {
            return Ok(MutationResult {
                success: false,
                backup_version: None,
                compiler_output: None,
                test_output: None,
                error: Some("Max failures reached. Manual intervention required.".into()),
            });
        }

        self.copy_to_staging()?;
        let staging_target = self.config.staging_dir.join(&request.target_file);
        self.apply_mutation(&staging_target, &request)?;

        if request.target_file.ends_with(".rs") {
            if let Err(e) = self.validate_rust_syntax(&staging_target) {
                self.failure_count += 1;
                return Ok(MutationResult {
                    success: false,
                    backup_version: None,
                    compiler_output: None,
                    test_output: None,
                    error: Some(format!("Syntax error: {}", e)),
                });
            }
        }

        let compile_res = self.compile_staging()?;
        if !compile_res.success {
            self.failure_count += 1;
            return Ok(MutationResult {
                success: false,
                backup_version: None,
                compiler_output: Some(compile_res.output),
                test_output: None,
                error: Some("Compilation failed".into()),
            });
        }

        self.current_version += 1;
        self.create_backup()?;
        self.promote_staging()?;
        self.failure_count = 0;

        Ok(MutationResult {
            success: true,
            backup_version: Some(self.current_version),
            compiler_output: Some(compile_res.output),
            test_output: None,
            error: None,
        })
    }

    fn copy_to_staging(&self) -> Result<()> {
        if self.config.staging_dir.exists() {
            let _ = std::fs::remove_dir_all(&self.config.staging_dir);
        }
        copy_dir_recursive(&self.config.source_root.join("crates"), &self.config.staging_dir.join("crates"))?;
        std::fs::copy(self.config.source_root.join("Cargo.toml"), self.config.staging_dir.join("Cargo.toml"))?;
        if self.config.source_root.join("Cargo.lock").exists() {
            std::fs::copy(self.config.source_root.join("Cargo.lock"), self.config.staging_dir.join("Cargo.lock"))?;
        }
        Ok(())
    }

    fn apply_mutation(&self, target: &Path, request: &MutationRequest) -> Result<()> {
        match &request.mutation_type {
            MutationType::CreateFile => {
                if let Some(parent) = target.parent() { let _ = std::fs::create_dir_all(parent); }
                std::fs::write(target, request.code.as_ref().context("code required")?)?;
            }
            MutationType::Append => {
                let mut content = std::fs::read_to_string(target).unwrap_or_default();
                content.push_str("\n");
                content.push_str(request.code.as_ref().context("code required")?);
                std::fs::write(target, content)?;
            }
            MutationType::Replace => {
                let content = std::fs::read_to_string(target)?;
                let find = request.find_pattern.as_ref().context("find required")?;
                let code = request.code.as_ref().context("code required")?;
                std::fs::write(target, content.replace(find, code))?;
            }
            MutationType::Insert { location } => {
                let content = std::fs::read_to_string(target)?;
                let code = request.code.as_ref().context("code required")?;
                std::fs::write(target, insert_at_location(&content, code, location)?)?;
            }
            MutationType::DeleteFile => { if target.exists() { let _ = std::fs::remove_file(target); } }
        }
        Ok(())
    }

    fn validate_rust_syntax(&self, file: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file)?;
        let mut stack = Vec::new();
        for ch in content.chars() {
            match ch {
                '{'|'('|'[' => stack.push(ch),
                '}' => if stack.pop() != Some('{') { anyhow::bail!("Unmatched }}"); },
                ')' => if stack.pop() != Some('(') { anyhow::bail!("Unmatched )"); },
                ']' => if stack.pop() != Some('[') { anyhow::bail!("Unmatched ]"); },
                _ => {}
            }
        }
        if !stack.is_empty() { anyhow::bail!("Unclosed: {:?}", stack); }
        Ok(())
    }

    fn compile_staging(&self) -> Result<CommandResult> {
        let output = Command::new("cargo").arg("check").current_dir(&self.config.staging_dir).output()?;
        Ok(CommandResult {
            success: output.status.success(),
            output: format!("{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)),
        })
    }

    fn create_backup(&self) -> Result<()> {
        let path = self.config.backup_dir.join(format!("v{}", self.current_version));
        copy_dir_recursive(&self.config.source_root.join("crates"), &path.join("crates"))?;
        Ok(())
    }

    fn promote_staging(&self) -> Result<()> {
        let staged = self.config.staging_dir.join("crates");
        let live = self.config.source_root.join("crates");
        for entry in std::fs::read_dir(&staged)? {
            let entry = entry?;
            let name = entry.file_name();
            let staged_src = staged.join(&name).join("src");
            let live_src = live.join(&name).join("src");
            if staged_src.exists() { copy_dir_recursive(&staged_src, &live_src)?; }
        }
        Ok(())
    }
}

struct CommandResult { success: bool, output: String }

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() { return Ok(()); }
    let _ = std::fs::create_dir_all(dst);
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() { copy_dir_recursive(&src_path, &dst_path)?; }
        else { let _ = std::fs::copy(&src_path, &dst_path); }
    }
    Ok(())
}

fn insert_at_location(content: &str, code: &str, location: &str) -> Result<String> {
    match location {
        "end_of_file" => Ok(format!("{}\n{}", content, code)),
        "after_imports" => {
            let lines: Vec<&str> = content.lines().collect();
            let mut last_use = 0;
            for (i, l) in lines.iter().enumerate() { if l.trim().starts_with("use ") { last_use = i; } }
            let mut res = Vec::new();
            for (i, l) in lines.iter().enumerate() {
                res.push(*l);
                if i == last_use { res.push(""); res.push(code); }
            }
            Ok(res.join("\n"))
        }
        _ => anyhow::bail!("Unknown location: {}", location),
    }
}
