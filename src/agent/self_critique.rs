//! Self-Critique Handler for Auto-Validation
//!
//! When verification detects a mismatch between the agent's claims and
//! actual tool outputs, this module generates prompts for self-correction.
//!
//! ## Flow
//!
//! 1. Verification fails → creates `VerificationFailure`
//! 2. `generate_self_critique_prompt()` creates a corrective prompt
//! 3. The agent responds to the prompt with corrections or escalation
//! 4. `handle_self_critique_response()` processes the agent's response

use crate::agent::closed_loop_verifier::VerificationFailure;

/// Response from the agent after self-critique.
#[derive(Debug, Clone)]
pub enum SelfCritiqueResponse {
    /// Agent successfully corrected the error.
    Corrected { corrected_response: String },
    /// Agent cannot correct the error and needs user clarification.
    CannotCorrect { question_for_user: String },
    /// Agent denied any error (potential manipulation attempt).
    Denied { explanation: String },
}

/// Generate a self-critique prompt when verification failures are detected.
///
/// This prompt instructs the agent to:
///
/// 1. Analyze WHY the failure happened
/// 2. Form a NEW hypothesis/plan
/// 3. Use tools to TRY AGAIN - don't just report failure, solve it
pub fn generate_self_critique_prompt(
    original_request: &str,
    agent_response: &str,
    verification_failures: &[VerificationFailure],
    max_retries: u32,
    current_retry: u32,
) -> String {
    let failure_summary = format_failures(verification_failures);
    let retry_note = if current_retry > 0 {
        format!(" (Retry {}/{})", current_retry, max_retries)
    } else {
        String::new()
    };

    format!(r#"## VERIFICATION FAILED{retry_note}

The system detected that your last attempt did NOT successfully complete the user's request:

---
{failure_summary}
---

## Your previous response:
---
{agent_response}
---

## Original user request:
---
{original_request}
---

## YOUR TASK: SOLVE THIS{retry_note}

Do NOT just report the failure. Instead:

1. **ANALYZE**: Why did your approach fail? What went wrong?

2. **PLAN**: What NEW approach will you try? (Different command, different parameters, different sequence)

3. **EXECUTE**: Use your tools to implement the new plan and COMPLETE the task.

4. **VERIFY**: After executing, verify the result matches what was requested.

You have {max_retries} attempts total. This is attempt {current_retry}. Make it count.
DO NOT give up. Use different commands, check intermediate results, try alternatives.
"#
    )
}

/// Format verification failures into a human-readable string.
fn format_failures(failures: &[VerificationFailure]) -> String {
    if failures.is_empty() {
        return "No failures recorded.".to_string();
    }

    let mut output = String::new();
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

/// Handle the agent's response to a self-critique prompt.
pub fn handle_self_critique_response(
    response: &str,
    _verification_failures: &[VerificationFailure],
) -> SelfCritiqueResponse {
    let response_trimmed = response.trim();

    if response_trimmed.starts_with("[CANNOT_CORRECT]") {
        let question = response_trimmed
            .strip_prefix("[CANNOT_CORRECT]")
            .map(|s| s.trim())
            .unwrap_or("I need clarification to proceed.")
            .to_string();
        return SelfCritiqueResponse::CannotCorrect { question_for_user: question };
    }

    // Check if agent denied the error without correction
    let denial_indicators = [
        "verification is incorrect",
        "the error is in the verification",
        "I stand by my response",
        "my answer was correct",
    ];
    let is_denial = denial_indicators.iter().any(|i| {
        response_trimmed.to_lowercase().contains(&i.to_lowercase())
    }) && !response_trimmed.to_lowercase().contains("however")
    && !response_trimmed.to_lowercase().contains("but")
    && response_trimmed.len() < 200;

    if is_denial {
        return SelfCritiqueResponse::Denied {
            explanation: response_trimmed.to_string(),
        };
    }

    // Default: agent provided corrections
    SelfCritiqueResponse::Corrected {
        corrected_response: response_trimmed.to_string(),
    }
}

/// Check if we should escalate to user (too many retries).
pub fn should_escalate(current_retry: u32, max_retries: u32) -> bool {
    current_retry >= max_retries
}

/// Generate an escalation message for the user.
pub fn generate_escalation_message(
    original_request: &str,
    verification_failures: &[VerificationFailure],
    all_attempts: &[String],
) -> String {
    let failure_summary = format_failures(verification_failures);
    let attempts_summary = all_attempts
        .iter()
        .enumerate()
        .map(|(i, a)| format!("Attempt {}: {}", i + 1, a))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"I was unable to provide a verified response after multiple attempts.

Original request:
---
{original_request}
---

Verification failures:
---
{failure_summary}
---

My attempts:
---
{attempts_summary}
---

Please clarify your request or provide additional guidance.
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_failure() -> VerificationFailure {
        VerificationFailure {
            tool_name: "file_write".to_string(),
            claim: Some("created config.yaml".to_string()),
            actual_output: "file does not exist".to_string(),
            mismatch_description: "file was not created".to_string(),
        }
    }

    #[test]
    fn test_handle_cannot_correct() {
        let response = "[CANNOT_CORRECT] What specific format should the config file use?";
        let failures = &[create_test_failure()];
        let result = handle_self_critique_response(response, failures);

        match result {
            SelfCritiqueResponse::CannotCorrect { question_for_user } => {
                assert!(question_for_user.contains("What specific format"));
            }
            _ => panic!("Expected CannotCorrect"),
        }
    }

    #[test]
    fn test_handle_denial() {
        let response = "I stand by my response. The verification is incorrect.";
        let failures = &[create_test_failure()];
        let result = handle_self_critique_response(response, failures);

        match result {
            SelfCritiqueResponse::Denied { explanation } => {
                assert!(explanation.contains("stand by"));
            }
            _ => panic!("Expected Denied"),
        }
    }

    #[test]
    fn test_handle_corrected() {
        let response = "I apologize for the error. The file has been created with the correct content.";
        let failures = &[create_test_failure()];
        let result = handle_self_critique_response(response, failures);

        match result {
            SelfCritiqueResponse::Corrected { corrected_response } => {
                assert!(corrected_response.contains("apologize"));
            }
            _ => panic!("Expected Corrected"),
        }
    }

    #[test]
    fn test_should_escalate() {
        assert!(should_escalate(2, 2));
        assert!(should_escalate(3, 2));
        assert!(!should_escalate(1, 2));
        assert!(!should_escalate(0, 2));
    }

    #[test]
    fn test_format_failures() {
        let failures = vec![
            create_test_failure(),
            VerificationFailure {
                tool_name: "shell".to_string(),
                claim: Some("command succeeded".to_string()),
                actual_output: "permission denied".to_string(),
                mismatch_description: "command failed".to_string(),
            },
        ];
        let output = format_failures(&failures);
        assert!(output.contains("file_write"));
        assert!(output.contains("shell"));
        assert!(output.contains("permission denied"));
    }
}
