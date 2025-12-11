#![deny(missing_docs)]

//! Local LLM integration (llamafile/gguf wrappers).

use core_types::Tier;

/// Request to the LLM layer.
pub struct Prompt<'a> {
    /// Tier to inform context limits.
    pub tier: Tier,
    /// User-visible question or instruction.
    pub user: &'a str,
    /// Optional system guidance.
    pub system: &'a str,
}

/// Response from the LLM layer.
pub struct Response {
    /// Model identifier.
    pub model: String,
    /// Plain text reply.
    pub text: String,
}

/// Minimal interface to send prompts to a local model.
pub trait LlmEngine {
    /// Model identifier.
    fn model_id(&self) -> &str;
    /// Execute a prompt and return a response.
    fn run(&self, prompt: Prompt) -> Response;
}

/// No-op engine for wiring.
pub struct NoopLlm;

impl LlmEngine for NoopLlm {
    fn model_id(&self) -> &str {
        "noop-llm"
    }

    fn run(&self, prompt: Prompt) -> Response {
        Response {
            model: self.model_id().to_string(),
            text: format!("LLM stub for tier {:?}: {}", prompt.tier, prompt.user),
        }
    }
}
