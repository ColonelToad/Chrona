#![deny(missing_docs)]

//! Local LLM integration using llama.cpp GGUF/llamafile format.
//!
//! This module provides abstractions for LLM engines and a simple implementation
//! that shells out to `llama-cli` for inference (installed via llama.cpp).

use core_types::Tier;
use std::path::{Path, PathBuf};
use std::process::Command;

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

/// Trait for LLM engines.
pub trait LlmEngine {
    /// Model identifier.
    fn model_id(&self) -> &str;
    /// Execute a prompt and return a response.
    fn run(&self, prompt: Prompt) -> Response;
}

/// Real LLM engine using llama-cli (from llama.cpp installation).
pub struct RealLlm {
    model_path: PathBuf,
    model_id: String,
}

impl RealLlm {
    /// Create a new real LLM engine from a .llamafile or GGUF path.
    ///
    /// Validates that the model file exists but does not pre-load it.
    /// Inference happens on-demand by shelling out to `llama-cli`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(format!("Model file not found: {:?}", path));
        }

        let model_id = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Self { model_path: path, model_id })
    }
}

impl LlmEngine for RealLlm {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn run(&self, prompt: Prompt) -> Response {
        // Determine context window and number of tokens to generate per tier
        let (context_size, n_predict) = match prompt.tier {
            Tier::Mini8 => (256, 64),       // Conservative for mini
            Tier::Standard16 => (512, 128), // Medium for standard
            Tier::Pro32 => (1024, 256),     // Generous for pro
        };

        // Build full prompt with system context
        let full_prompt = if prompt.system.is_empty() {
            prompt.user.to_string()
        } else {
            format!("{}\n\n{}", prompt.system, prompt.user)
        };

        // Try to run the .llamafile directly (it's an executable)
        let output = Command::new(&self.model_path)
            .arg("-c")
            .arg(context_size.to_string())
            .arg("-n")
            .arg(n_predict.to_string())
            .arg("-p")
            .arg(&full_prompt)
            .arg("-t")
            .arg("4") // Use 4 threads
            .output();

        let text = match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                if !stdout.is_empty() {
                    // Extract the actual response (skip the prompt echo if present)
                    let response = stdout.trim();
                    if let Some(idx) = response.rfind(&full_prompt) {
                        response[idx + full_prompt.len()..].trim().to_string()
                    } else {
                        response.to_string()
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    if !stderr.is_empty() {
                        format!("Error: {}", stderr.trim())
                    } else {
                        format!(
                            "Model inference for '{}' (limit: {} tokens)",
                            prompt.user, n_predict
                        )
                    }
                }
            }
            Err(e) => {
                // Fallback if .llamafile execution fails
                format!(
                    "Could not execute model: {}. Ensure the .llamafile has execute permissions.",
                    e
                )
            }
        };

        Response {
            model: self.model_id.clone(),
            text,
        }
    }
}

/// No-op engine for testing and tier setup (graceful fallback when llama-cli unavailable).
pub struct NoopLlm;

impl LlmEngine for NoopLlm {
    fn model_id(&self) -> &str {
        "noop-llm"
    }

    fn run(&self, prompt: Prompt) -> Response {
        // Generate a plausible stub response
        let response_text = match prompt.user {
            q if q.to_lowercase().contains("heart") => {
                "Your heart rate is within normal range. Continue monitoring. Regular exercise helps maintain cardiovascular health."
                    .to_string()
            }
            _ => format!(
                "Running tier {:?} analysis on your query. More specific prompts yield better results.",
                prompt.tier
            ),
        };

        Response {
            model: self.model_id().to_string(),
            text: response_text,
        }
    }
}
