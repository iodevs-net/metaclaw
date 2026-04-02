//! Reflection engine for post-task analysis and memory hardening.
//!
//! This module analyzes conversation history to extract lessons learned,
//! architectural insights, and user preferences, which are then stored
//! in the agent's long-term memory.

use crate::memory::{Memory, MemoryCategory};
use crate::providers::{ChatMessage, ChatRequest, Provider};
use anyhow::Result;
use std::sync::Arc;

/// Engine for performing post-task reflection.
pub struct ReflectionEngine<P, M>
where
    P: ReflectionProvider,
    M: Memory,
{
    provider: P,
    model: String,
    memory: Arc<M>,
}

impl<P, M> ReflectionEngine<P, M>
where
    P: ReflectionProvider,
    M: Memory,
{
    /// Create a new reflection engine.
    pub fn new(provider: P, model: String, memory: Arc<M>) -> Self {
        Self {
            provider,
            model,
            memory,
        }
    }

    /// Analyze the conversation history and extract lessons learned.
    pub async fn reflect(
        &self,
        history: &[ChatMessage],
        _session_id: Option<&str>,
    ) -> Result<String> {
        if history.is_empty() {
            return Ok("No history to reflect upon.".into());
        }

        let reflection_prompt = r#"
Analyze the following conversation history between an AI Assistant and a User.
Extract "Lessons Learned" regarding:
1. Technical/Architectural insights (patterns, project structure, bugs found).
2. User preferences (coding style, preferred tools, workflow).
3. System "Gotchas" or obstacles encountered and how they were resolved.

Format your response as a concise Markdown summary.
Do NOT use emojis. Maintain a Senior Full Stack Orchestrator tone.
If nothing significant was learned, respond with "NO_SIGNIFICANT_LEARNINGS".
"#;

        let mut messages = vec![ChatMessage::system(reflection_prompt.to_string())];
        // Add a condensed version of the history to the reflection request
        for msg in history.iter().rev().take(10).rev() {
            messages.push(msg.clone());
        }

        let response = self
            .provider
            .chat(
                ChatRequest {
                    messages: &messages,
                    tools: None,
                },
                &self.model,
                0.0, // deterministic
            )
            .await?;

        let text = response.text_or_empty();
        if text == "NO_SIGNIFICANT_LEARNINGS" || text.trim().is_empty() {
            return Ok("No significant learnings extracted.".into());
        }

        // Store the reflection in memory
        let key = format!("reflection_{}", chrono::Utc::now().to_rfc3339());
        self.memory.store(
            &key,
            &text,
            MemoryCategory::Custom("reflection".to_string()),
            None,
        ).await?;

        Ok(text.to_string())
    }
}

/// Provider trait for reflection - abstracts over the chat provider.
#[async_trait::async_trait]
pub trait ReflectionProvider: Send + Sync {
    async fn chat(
        &self,
        request: ChatRequest<'_>,
        model: &str,
        temperature: f64,
    ) -> anyhow::Result<crate::providers::ChatResponse>;
}

/// Wrapper for using a Provider as a ReflectionProvider.
pub struct ProviderReflectionProvider<P> {
    provider: P,
}

impl<P> ProviderReflectionProvider<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

#[async_trait::async_trait]
impl<P> ReflectionProvider for ProviderReflectionProvider<P>
where
    P: Provider + Send + Sync,
{
    async fn chat(
        &self,
        request: ChatRequest<'_>,
        model: &str,
        temperature: f64,
    ) -> anyhow::Result<crate::providers::ChatResponse> {
        self.provider.chat(request, model, temperature).await
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SimpleReflectionEngine - uses trait objects for dynamic dispatch
// ─────────────────────────────────────────────────────────────────────────────

/// Reflection engine that uses trait objects (Arc<dyn ...>) instead of generics.
/// This allows it to work with Arc<dyn Provider> and Arc<dyn Memory>.
pub struct SimpleReflectionEngine {
    provider: Arc<dyn ReflectionProvider>,
    memory: Arc<dyn Memory>,
    model: String,
}

impl SimpleReflectionEngine {
    /// Create a new simple reflection engine with dynamic dispatch.
    pub fn new(
        provider: Arc<dyn ReflectionProvider>,
        memory: Arc<dyn Memory>,
        model: String,
    ) -> Self {
        Self {
            provider,
            memory,
            model,
        }
    }

    /// Analyze the conversation history and extract lessons learned.
    /// Stores significant learnings in memory.
    pub async fn reflect(
        &self,
        history: &[ChatMessage],
        _session_id: Option<&str>,
    ) -> anyhow::Result<String> {
        if history.is_empty() {
            return Ok("No history to reflect upon.".into());
        }

        let reflection_prompt = r#"
Analyze the following conversation history between an AI Assistant and a User.
Extract "Lessons Learned" regarding:
1. Technical/Architectural insights (patterns, project structure, bugs found).
2. User preferences (coding style, preferred tools, workflow).
3. System "Gotchas" or obstacles encountered and how they were resolved.

Format your response as a concise Markdown summary.
Do NOT use emojis. Maintain a Senior Full Stack Orchestrator tone.
If nothing significant was learned, respond with "NO_SIGNIFICANT_LEARNINGS".
"#;

        let mut messages = vec![ChatMessage::system(reflection_prompt.to_string())];
        // Add a condensed version of the history to the reflection request
        for msg in history.iter().rev().take(10).rev() {
            messages.push(msg.clone());
        }

        let response = self
            .provider
            .chat(
                ChatRequest {
                    messages: &messages,
                    tools: None,
                },
                &self.model,
                0.0, // deterministic
            )
            .await?;

        let text = response.text_or_empty();
        if text == "NO_SIGNIFICANT_LEARNINGS" || text.trim().is_empty() {
            return Ok("No significant learnings extracted.".into());
        }

        // Store the reflection in memory
        let key = format!("reflection_{}", chrono::Utc::now().to_rfc3339());
        self.memory.store(
            &key,
            &text,
            MemoryCategory::Custom("reflection".to_string()),
            None,
        ).await?;

        Ok(text.to_string())
    }
}

/// Trait for reflection engines that can perform post-response reflection.
#[async_trait::async_trait]
pub trait Reflector: Send + Sync {
    /// Perform reflection on the conversation history and store insights.
    async fn reflect(
        &self,
        history: &[ChatMessage],
        session_id: Option<&str>,
    ) -> anyhow::Result<String>;
}

#[async_trait::async_trait]
impl Reflector for SimpleReflectionEngine {
    async fn reflect(
        &self,
        history: &[ChatMessage],
        session_id: Option<&str>,
    ) -> anyhow::Result<String> {
        self.reflect(history, session_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::MemoryEntry;
    use crate::providers::ChatResponse;

    struct MockProvider;
    struct MockMemory;

    #[async_trait::async_trait]
    impl ReflectionProvider for MockProvider {
        async fn chat(
            &self,
            _request: ChatRequest<'_>,
            _model: &str,
            _temperature: f64,
        ) -> anyhow::Result<ChatResponse> {
            Ok(ChatResponse {
                text: Some("NO_SIGNIFICANT_LEARNINGS".to_string()),
                tool_calls: vec![],
                usage: None,
                reasoning_content: None,
            })
        }
    }

    #[async_trait::async_trait]
    impl Memory for MockMemory {
        fn name(&self) -> &str { "mock" }

        async fn store(&self, _key: &str, _content: &str, _category: MemoryCategory, _session_id: Option<&str>) -> anyhow::Result<()> {
            Ok(())
        }

        async fn recall(&self, _query: &str, _limit: usize, _session_id: Option<&str>, _since: Option<&str>, _until: Option<&str>) -> anyhow::Result<Vec<MemoryEntry>> {
            Ok(vec![])
        }

        async fn get(&self, _key: &str) -> anyhow::Result<Option<MemoryEntry>> {
            Ok(None)
        }

        async fn list(&self, _category: Option<&MemoryCategory>, _session_id: Option<&str>) -> anyhow::Result<Vec<MemoryEntry>> {
            Ok(vec![])
        }

        async fn forget(&self, _key: &str) -> anyhow::Result<bool> {
            Ok(true)
        }

        async fn count(&self) -> anyhow::Result<usize> {
            Ok(0)
        }

        async fn health_check(&self) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn reflection_with_no_history_returns_early() {
        let provider = MockProvider;
        let memory = Arc::new(MockMemory);
        let engine = ReflectionEngine::new(provider, "default".to_string(), memory);

        let result = engine.reflect(&[], None).await.unwrap();
        assert_eq!(result, "No history to reflect upon.");
    }

    #[tokio::test]
    async fn reflection_with_no_significant_learnings() {
        let provider = MockProvider;
        let memory = Arc::new(MockMemory);
        let engine = ReflectionEngine::new(provider, "default".to_string(), memory);

        let history = vec![ChatMessage::user("Hello".to_string())];
        let result = engine.reflect(&history, None).await.unwrap();
        assert_eq!(result, "No significant learnings extracted.");
    }
}
