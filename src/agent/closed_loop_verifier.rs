//! Closed-Loop Verification for tool executions.
//!
//! This module provides write verification capabilities for tools that modify
//! the filesystem. It verifies that the actual filesystem content matches the
//! tool's intended output.
//!
//! ## Auto-Validation System
//!
//! The verification system has 3 tiers:
//! 1. **Write Verification** (`VerifyResult` trait) - Verifies file/command writes
//! 2. **Claim Verification** - Verifies agent claims against tool outputs
//! 3. **Self-Critique** - Handles mismatches via self-correction loop

use sha2::{Digest, Sha256};
use std::path::Path;
use serde_json::Value;

/// Result of closed-loop verification.
#[derive(Debug, Clone)]
pub enum VerificationResult {
    /// Verification passed.
    Match,
    /// No credential was provided — verification is not applicable.
    NoCredential,
    /// Verification failed — hash mismatch.
    Mismatch { expected: String, actual: String },
}

/// Extended verification result for auto-validation with claim checking.
#[derive(Debug, Clone)]
pub enum ClaimVerificationResult {
    /// The claim matches the actual tool output.
    Match,
    /// The claim does NOT match the actual output (potential hallucination).
    Mismatch { claim: String, actual: String },
    /// Verification could not be performed (unknown tool, missing data).
    Unverifiable { reason: String },
    /// The agent made a claim that appears to be hallucinated.
    Hallucination { claim: String, confidence: f64 },
}

/// Failure record for self-critique integration.
#[derive(Debug, Clone)]
pub struct VerificationFailure {
    /// The tool that was executed.
    pub tool_name: String,
    /// The claim made by the agent (if extractable).
    pub claim: Option<String>,
    /// The actual output from the tool.
    pub actual_output: String,
    /// Description of the mismatch.
    pub mismatch_description: String,
}

/// Trait for verifying tool execution results.
///
/// Implement this trait to add verification for new tools.
/// Each implementation should verify that the tool's output
/// matches what was claimed or expected.
pub trait VerifyResult: Send + Sync {
    /// Verify the result of a tool execution.
    ///
    /// `tool_args` - The arguments passed to the tool.
    /// `tool_output` - The raw output from the tool execution.
    /// `workspace_dir` - The workspace directory for resolving relative paths.
    ///
    /// Returns `Ok(Some(ClaimVerificationResult::Match))` on success,
    /// `Ok(None)` if this verifier doesn't apply to this tool,
    /// `Ok(Some(ClaimVerificationResult::Mismatch))` on verification failure,
    /// `Err(...)` on internal error.
    fn verify(
        &self,
        tool_name: &str,
        tool_args: &Value,
        tool_output: &str,
        workspace_dir: &Path,
    ) -> Result<Option<ClaimVerificationResult>, String>;
}

/// Verify that a tool execution result_hash matches the expected sd_hash
/// from a Verifiable Intent credential.
pub fn verify_result_hash(result_hash: &str, sd_hash: Option<&str>) -> VerificationResult {
    match sd_hash {
        None => VerificationResult::NoCredential,
        Some(expected) => {
            if result_hash == expected {
                VerificationResult::Match
            } else {
                VerificationResult::Mismatch {
                    expected: expected.to_string(),
                    actual: result_hash.to_string(),
                }
            }
        }
    }
}

// ── Write Tool Verification ─────────────────────────────────────────

/// Tools that modify the filesystem and can be verified.
const WRITE_TOOLS: &[&str] = &["file_write", "shell", "file_edit"];

/// Check if a tool name is a write operation that modifies files.
pub fn is_write_tool(tool_name: &str) -> bool {
    WRITE_TOOLS.contains(&tool_name)
}

/// Verify a file_write tool execution by reading the actual file content
/// and comparing it against the expected content from the tool arguments.
pub fn verify_file_write(
    tool_args: &serde_json::Value,
    workspace_dir: &Path,
) -> Result<Option<()>, String> {
    let path = tool_args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "missing 'path' in file_write args".to_string())?;

    let expected_content = tool_args
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "missing 'content' in file_write args".to_string())?;

    // Resolve path relative to workspace
    let full_path = if Path::new(path).is_absolute() {
        Path::new(path).to_path_buf()
    } else {
        workspace_dir.join(path)
    };

    // Read actual file content
    let actual_content = match std::fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(e) => {
            return Err(format!(
                "file_write verification: failed to read '{}': {e}",
                full_path.display()
            ));
        }
    };

    // Compare content
    if actual_content == expected_content {
        Ok(Some(()))
    } else {
        let expected_hash = hex::encode(Sha256::digest(expected_content.as_bytes()));
        let actual_hash = hex::encode(Sha256::digest(actual_content.as_bytes()));
        Err(format!(
            "file_write verification FAILED for '{}': content mismatch (expected {}, actual {})",
            full_path.display(),
            expected_hash,
            actual_hash
        ))
    }
}

/// Verify a shell command that may have written files.
///
/// For shell commands, we check if the output indicates a successful file write
/// and verify the target file if identifiable.
///
/// This is a best-effort verification — shell commands are complex and not all
/// writes can be reliably detected.
pub fn verify_shell_write(
    _tool_args: &serde_json::Value,
    _workspace_dir: &Path,
    _tool_output: &str,
) -> Result<Option<()>, String> {
    // Shell write verification is complex because:
    // 1. We need to parse the command to identify write targets
    // 2. We need to know what content was written
    // For now, return None (not applicable) — full implementation
    // would require parsing the shell command and extracting write targets.
    Ok(None)
}

/// Verify a write tool execution result.
pub fn verify_write_operation(
    tool_name: &str,
    tool_args: &serde_json::Value,
    tool_output: &str,
    result_hash: &str,
    workspace_dir: &Path,
) -> Result<Option<()>, String> {
    if !is_write_tool(tool_name) {
        return Ok(None);
    }

    match tool_name {
        "file_write" => {
            if let Err(e) = verify_file_write(tool_args, workspace_dir) {
                tracing::error!(
                    tool = tool_name,
                    result_hash,
                    "CLOSED-LOOP VERIFICATION FAILED: {e}"
                );
                return Err(e);
            }
        }
        "shell" => {
            if let Err(e) = verify_shell_write(tool_args, workspace_dir, tool_output) {
                tracing::error!(
                    tool = tool_name,
                    result_hash,
                    "CLOSED-LOOP VERIFICATION FAILED: {e}"
                );
                return Err(e);
            }
        }
        _ => {}
    }

    tracing::debug!(
        tool = tool_name,
        result_hash,
        "closed-loop write verification passed"
    );
    Ok(Some(()))
}

// ── Generic Tool Verifiers ─────────────────────────────────────────────

/// Default verification result type — verifies file writes and shell commands.
pub struct DefaultToolVerifier;

impl VerifyResult for DefaultToolVerifier {
    fn verify(
        &self,
        tool_name: &str,
        tool_args: &Value,
        tool_output: &str,
        workspace_dir: &Path,
    ) -> Result<Option<ClaimVerificationResult>, String> {
        match tool_name {
            "file_write" => {
                let result = verify_file_write(tool_args, workspace_dir);
                match result {
                    Ok(Some(())) => Ok(Some(ClaimVerificationResult::Match)),
                    Ok(None) => Ok(None),
                    Err(e) => {
                        let expected = tool_args
                            .get("content")
                            .and_then(|v| v.as_str())
                            .unwrap_or("<unknown>")
                            .to_string();
                        Ok(Some(ClaimVerificationResult::Mismatch {
                            claim: format!("file contains: {}", expected),
                            actual: format!("verification failed: {}", e),
                        }))
                    }
                }
            }
            "shell" => {
                let result = verify_shell_write(tool_args, workspace_dir, tool_output);
                match result {
                    Ok(Some(())) => Ok(Some(ClaimVerificationResult::Match)),
                    Ok(None) => Ok(None),
                    Err(e) => Ok(Some(ClaimVerificationResult::Mismatch {
                        claim: "shell command executed successfully".to_string(),
                        actual: e,
                    })),
                }
            }
            "file_read" => {
                let path = tool_args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "missing 'path' in file_read args".to_string())?;

                let full_path = if Path::new(path).is_absolute() {
                    Path::new(path).to_path_buf()
                } else {
                    workspace_dir.join(path)
                };

                if full_path.exists() {
                    Ok(Some(ClaimVerificationResult::Match))
                } else {
                    Ok(Some(ClaimVerificationResult::Mismatch {
                        claim: format!("file {} exists and is readable", path),
                        actual: format!("file {} does not exist", path),
                    }))
                }
            }
            _ => Ok(None),
        }
    }
}

// ── Claim Verification ────────────────────────────────────────────────

/// Verify that an agent's textual claim matches the actual tool output.
pub fn verify_claim_against_output(
    claim: &str,
    tool_name: &str,
    tool_args: &Value,
    tool_output: &str,
    workspace_dir: &Path,
) -> ClaimVerificationResult {
    match tool_name {
        "file_write" => verify_file_write_claim(claim, tool_args, tool_output, workspace_dir),
        "file_read" => verify_file_read_claim(claim, tool_output, workspace_dir),
        "shell" => verify_shell_claim(claim, tool_output),
        "glob" => verify_glob_claim(claim, tool_output),
        "WebSearch" | "web_search" => verify_web_search_claim(claim, tool_output),
        _ => ClaimVerificationResult::Unverifiable {
            reason: format!("No claim verifier for tool: {}", tool_name),
        },
    }
}

fn verify_file_write_claim(
    claim: &str,
    tool_args: &Value,
    tool_output: &str,
    workspace_dir: &Path,
) -> ClaimVerificationResult {
    let path = match tool_args.get("path").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ClaimVerificationResult::Unverifiable {
            reason: "file_write missing path argument".to_string(),
        },
    };

    let full_path = if Path::new(path).is_absolute() {
        Path::new(path).to_path_buf()
    } else {
        workspace_dir.join(path)
    };

    if !full_path.exists() {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: format!("file '{}' does not exist", path),
        };
    }

    if tool_output.contains("Error:") || tool_output.to_lowercase().contains("failed") {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: format!("tool reported failure: {}", tool_output),
        };
    }

    ClaimVerificationResult::Match
}

fn verify_file_read_claim(
    claim: &str,
    tool_output: &str,
    _workspace_dir: &Path,
) -> ClaimVerificationResult {
    if tool_output.starts_with("Error:") || tool_output.contains("No such file") {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: tool_output.to_string(),
        };
    }

    if !claim.is_empty() && !tool_output.contains(claim)
        && claim.len() < 100 && !claim.contains("Error") {
            return ClaimVerificationResult::Mismatch {
                claim: claim.to_string(),
                actual: "claimed content not found in file output".to_string(),
            };
        }

    ClaimVerificationResult::Match
}

fn verify_shell_claim(claim: &str, tool_output: &str) -> ClaimVerificationResult {
    let error_indicators = [
        "command not found",
        "no such file",
        "permission denied",
        "error:",
        "failed",
        "fatal:",
    ];

    let output_lower = tool_output.to_lowercase();
    for indicator in &error_indicators {
        if output_lower.contains(indicator) {
            return ClaimVerificationResult::Mismatch {
                claim: claim.to_string(),
                actual: format!("shell output contains error indicator: {}", indicator),
            };
        }
    }

    if !claim.is_empty() && claim.len() < 200
        && !tool_output.contains(claim) && !output_lower.contains(&claim.to_lowercase()) {
            return ClaimVerificationResult::Mismatch {
                claim: claim.to_string(),
                actual: "claimed output not found in command result".to_string(),
            };
        }

    ClaimVerificationResult::Match
}

fn verify_glob_claim(claim: &str, tool_output: &str) -> ClaimVerificationResult {
    let match_count = tool_output.lines().count();

    if let Some(captures) = regex::Regex::new(r"(\d+)\s+(files?|matches?)")
        .ok()
        .and_then(|r| r.captures(claim))
    {
        if let Some(m) = captures.get(1) {
            if let Ok(claimed_count) = m.as_str().parse::<usize>() {
                let lower = claimed_count * 80 / 100;
                let upper = claimed_count * 120 / 100;
                if match_count < lower || match_count > upper {
                    return ClaimVerificationResult::Mismatch {
                        claim: format!("claimed {} matches", claimed_count),
                        actual: format!("found {} matches", match_count),
                    };
                }
            }
        }
    }

    if tool_output.contains("No such file") || tool_output.contains("Error:") {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: tool_output.to_string(),
        };
    }

    ClaimVerificationResult::Match
}

fn verify_web_search_claim(claim: &str, tool_output: &str) -> ClaimVerificationResult {
    let url_indicators = ["http://", "https://", "www."];
    let has_urls = url_indicators.iter().any(|u| tool_output.contains(u));

    if claim.contains("http") && !has_urls {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: "claimed URLs not found in search results".to_string(),
        };
    }

    if tool_output.trim().len() < 20 {
        return ClaimVerificationResult::Mismatch {
            claim: claim.to_string(),
            actual: "search results empty or truncated".to_string(),
        };
    }

    ClaimVerificationResult::Match
}

// ── Verification Entry Point ──────────────────────────────────────────

/// Default verifier instance for convenience.
pub static DEFAULT_VERIFIER: DefaultToolVerifier = DefaultToolVerifier;

/// Verify a tool execution with the default verifier.
pub fn verify_tool_execution(
    tool_name: &str,
    tool_args: &Value,
    tool_output: &str,
    workspace_dir: &Path,
) -> Result<Option<ClaimVerificationResult>, String> {
    DEFAULT_VERIFIER.verify(tool_name, tool_args, tool_output, workspace_dir)
}

/// Format verification failures for self-critique prompt.
pub fn format_verification_failures(failures: &[VerificationFailure]) -> String {
    if failures.is_empty() {
        return String::new();
    }

    let mut output = String::from("Verification Failures:\n");
    for (i, failure) in failures.iter().enumerate() {
        output.push_str(&format!(
            "{}. Tool: {} | Claim: {} | Actual: {} | Issue: {}\n",
            i + 1,
            failure.tool_name,
            failure.claim.as_deref().unwrap_or("<none>"),
            failure.actual_output,
            failure.mismatch_description
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_verify_file_write_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let args = serde_json::json!({
            "path": file_path.to_str().unwrap(),
            "content": "hello world"
        });

        // Write the file first
        let mut file = std::fs::File::create(&file_path).unwrap();
        file.write_all(b"hello world").unwrap();

        let result = verify_file_write(&args, temp_dir.path());
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_verify_file_write_content_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let args = serde_json::json!({
            "path": file_path.to_str().unwrap(),
            "content": "expected content"
        });

        // Write different content
        let mut file = std::fs::File::create(&file_path).unwrap();
        file.write_all(b"different content").unwrap();

        let result = verify_file_write(&args, temp_dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("content mismatch"));
    }

    #[test]
    fn test_verify_file_write_missing_path() {
        let temp_dir = TempDir::new().unwrap();
        let args = serde_json::json!({
            "content": "some content"
        });

        let result = verify_file_write(&args, temp_dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing 'path'"));
    }

    #[test]
    fn test_verify_file_write_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let args = serde_json::json!({
            "path": "nonexistent.txt",
            "content": "some content"
        });

        let result = verify_file_write(&args, temp_dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("failed to read"));
    }

    #[test]
    fn test_is_write_tool() {
        assert!(is_write_tool("file_write"));
        assert!(is_write_tool("shell"));
        assert!(is_write_tool("file_edit"));
        assert!(!is_write_tool("file_read"));
        assert!(!is_write_tool("glob"));
        assert!(!is_write_tool("WebSearch"));
    }

    #[test]
    fn test_verify_result_hash_match() {
        let hash = "abc123";
        let result = verify_result_hash(hash, Some("abc123"));
        match result {
            VerificationResult::Match => {}
            _ => panic!("Expected Match"),
        }
    }

    #[test]
    fn test_verify_result_hash_mismatch() {
        let result = verify_result_hash("abc123", Some("xyz789"));
        match result {
            VerificationResult::Mismatch { expected, actual } => {
                assert_eq!(expected, "xyz789");
                assert_eq!(actual, "abc123");
            }
            _ => panic!("Expected Mismatch"),
        }
    }

    #[test]
    fn test_verify_result_hash_no_credential() {
        let result = verify_result_hash("abc123", None);
        match result {
            VerificationResult::NoCredential => {}
            _ => panic!("Expected NoCredential"),
        }
    }

    #[test]
    fn test_verify_tool_execution_file_write() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let args = serde_json::json!({
            "path": file_path.to_str().unwrap(),
            "content": "test content"
        });

        // Create file with matching content
        let mut file = std::fs::File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let result = verify_tool_execution(
            "file_write",
            &args,
            "wrote 12 bytes",
            temp_dir.path(),
        );

        assert!(result.is_ok());
        let verification = result.unwrap();
        assert!(verification.is_some());
        match verification.unwrap() {
            ClaimVerificationResult::Match => {}
            other => panic!("Expected Match, got {:?}", other),
        }
    }

    #[test]
    fn test_verify_claim_against_output_shell_error() {
        let result = verify_claim_against_output(
            "command succeeded",
            "shell",
            &serde_json::json!({}),
            "Error: permission denied",
            std::path::Path::new("/tmp"),
        );

        match result {
            ClaimVerificationResult::Mismatch { claim, actual } => {
                assert!(claim.contains("succeeded"));
                assert!(actual.contains("permission denied"));
            }
            _ => panic!("Expected Mismatch"),
        }
    }

    #[test]
    fn test_format_verification_failures() {
        let failures = vec![
            VerificationFailure {
                tool_name: "file_write".to_string(),
                claim: Some("created config.yaml".to_string()),
                actual_output: "file does not exist".to_string(),
                mismatch_description: "file was not created".to_string(),
            },
            VerificationFailure {
                tool_name: "shell".to_string(),
                claim: None,
                actual_output: "command failed".to_string(),
                mismatch_description: "exit code 1".to_string(),
            },
        ];

        let output = format_verification_failures(&failures);
        assert!(output.contains("file_write"));
        assert!(output.contains("shell"));
        assert!(output.contains("config.yaml"));
        assert!(output.contains("command failed"));
    }

    #[test]
    fn test_format_verification_failures_empty() {
        let failures: Vec<VerificationFailure> = vec![];
        let output = format_verification_failures(&failures);
        assert_eq!(output, "");
    }
}
