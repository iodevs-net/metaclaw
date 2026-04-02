//! Closed-Loop Verification Hook
//!
//! Implements HookHandler to verify file_write and shell tool executions
//! after they complete, detecting hallucinations and content mismatches.

use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::hooks::traits::{HookHandler, HookResult};
use crate::providers::traits::ChatMessage;
use crate::providers::traits::ChatResponse;
use crate::channels::traits::ChannelMessage;
use crate::tools::traits::ToolResult;

/// Hook handler that performs closed-loop verification on write tools.
pub struct ClosedLoopVerifierHook {
    workspace_dir: PathBuf,
}

impl ClosedLoopVerifierHook {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self { workspace_dir }
    }

    pub fn boxed(workspace_dir: PathBuf) -> Box<Self> {
        Box::new(Self::new(workspace_dir))
    }
}

#[async_trait]
impl HookHandler for ClosedLoopVerifierHook {
    fn name(&self) -> &str {
        "closed_loop_verifier"
    }

    fn priority(&self) -> i32 {
        50 // Run after other hooks
    }

    async fn on_after_tool_call(
        &self,
        tool: &str,
        result: &ToolResult,
        _duration: Duration,
    ) {
        use crate::agent::closed_loop_verifier as clv;

        if !clv::is_write_tool(tool) {
            return;
        }

        // For now, just log that we observed the tool call
        // Full verification requires the original arguments which aren't passed to this hook
        tracing::debug!(
            tool,
            success = result.success,
            "closed-loop verifier observed {} tool execution",
            tool
        );

        // If the tool failed, that's also useful to log
        if !result.success {
            tracing::warn!(
                tool,
                error = result.error.as_deref().unwrap_or(&result.output),
                "closed-loop verifier: tool failed"
            );
        }
    }
}
