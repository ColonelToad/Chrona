//! Tier-specific engine instances for UI state.

use core_types::Tier;
use data_layer::{NoopStore, TimeSeriesStore};
use llm_runtime::{LlmEngine, NoopLlm, Prompt, RealLlm};
use ml_runtime::{ActivityClassifier, Model, NoopModel};
use logic::{ActivityContext, build_mini_prompt};
use std::fs;
use sensors::{MhealthStreamingSensor, Sensor, SyntheticHeartRate};

/// Wrapper holding all runtime components for one tier.
pub struct TierEngine {
    pub tier: Tier,
    pub sensor: Box<dyn Sensor>,
    pub store: Box<dyn TimeSeriesStore>,
    pub model: Box<dyn Model>,
    pub llm: Box<dyn LlmEngine>,
    pub last_value: Option<f32>,
    // Mini 8GB: MHEALTH activity context
    pub activity_context: Option<ActivityContext>,
    pub baseline_hr: u32,
    // For Mini tier: keep reference to streaming sensor for activity window
    mhealth_sensor: Option<MhealthStreamingSensor>,
}

impl TierEngine {
    /// Create a new stub engine for a tier.
    pub fn new_stub(tier: Tier) -> Self {
        let start_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let baseline = match tier {
            Tier::Mini8 => 70.0,
            Tier::Standard16 => 72.0,
            Tier::Pro32 => 74.0,
        };

        // Try to load real model for this tier, fall back to noop
        let llm: Box<dyn LlmEngine> = match tier {
            Tier::Mini8 => {
                // Look for mini tier model (try multiple paths)
                let possible_paths = vec![
                    // New models directory structure
                    "C:\\Users\\legot\\Chrona\\models\\mini\\llm\\model-3b.llamafile",
                    "models/mini/llm/model-3b.llamafile",
                    "models\\mini\\llm\\model-3b.llamafile",
                    // Alternate model filename in models root
                    "C:\\Users\\legot\\Chrona\\models\\Llama-3.2-3B-Instruct.Q6_K.llamafile",
                    "models/Llama-3.2-3B-Instruct.Q6_K.llamafile",
                    // Legacy data paths for backward compatibility
                    "C:\\Users\\legot\\Chrona\\data\\mini\\model-3b.llamafile",
                    "data/mini/model-3b.llamafile",
                    "../data/mini/model-3b.llamafile",
                ];
                
                let mut loaded_llm: Option<RealLlm> = None;
                for model_path in &possible_paths {
                    match RealLlm::new(model_path) {
                        Ok(engine) => {
                            println!("✓ Loaded LLM from: {}", model_path);
                            loaded_llm = Some(engine);
                            break;
                        }
                        Err(e) => {
                            println!("✗ Failed to load LLM from {}: {}", model_path, e);
                        }
                    }
                }
                
                match loaded_llm {
                    Some(engine) => Box::new(engine),
                    None => {
                        println!("⚠ Falling back to NoopLlm");
                        Box::new(NoopLlm)
                    }
                }
            }
            Tier::Standard16 => Box::new(NoopLlm), // TODO: add 16GB model
            Tier::Pro32 => Box::new(NoopLlm),      // TODO: add 32GB model
        };
        
        // Load MHEALTH sensor for Mini tier
        let (sensor, mhealth_sensor_copy): (Box<dyn Sensor>, Option<MhealthStreamingSensor>) = match tier {
            Tier::Mini8 => {
                let possible_paths = vec![
                    "C:\\Users\\legot\\Chrona\\data\\mini\\mhealth_raw_data.csv",
                    "data\\mini\\mhealth_raw_data.csv",
                    "..\\..\\data\\mini\\mhealth_raw_data.csv",
                ];
                if let Some(content) = possible_paths.iter().find_map(|p| fs::read_to_string(p).ok()) {
                    let mhealth = MhealthStreamingSensor::from_csv(&content);
                    // Clone for window access
                    let mhealth_copy = MhealthStreamingSensor::from_csv(&content);
                    (Box::new(mhealth), Some(mhealth_copy))
                } else {
                    (Box::new(SyntheticHeartRate::new(baseline, 5.0, start_ts)), None)
                }
            }
            _ => (Box::new(SyntheticHeartRate::new(baseline, 5.0, start_ts)), None),
        };

        Self {
            tier,
            sensor,
            store: Box::new(NoopStore),
            model: Box::new(NoopModel),
            llm,
            last_value: None,
            activity_context: None,
            baseline_hr: baseline as u32,
            mhealth_sensor: mhealth_sensor_copy,
        }
    }

    /// Poll sensor and update state.
    pub fn poll(&mut self) {
        if let Some(sample) = self.sensor.poll() {
            self.store.write(self.sensor.name(), sample.clone());
            self.last_value = Some(sample.value);
        }

        // Update activity from MHEALTH sensor (Mini tier only)
        if matches!(self.tier, Tier::Mini8) {
            if let Some(ref mhealth) = self.mhealth_sensor {
                let window = mhealth.get_window();
                if !window.is_empty() {
                    let (_label, _conf) = ActivityClassifier::classify(&window);
                    if let Some(first) = window.first() {
                        let ctx = ActivityContext::from_record(first, self.baseline_hr);
                        self.activity_context = Some(ctx);
                    }
                }
            }
        }
    }

    /// Get current sensor value or placeholder.
    pub fn current_value(&self) -> String {
        self.last_value
            .map(|v| format!("{:.0} bpm", v))
            .unwrap_or_else(|| "-- bpm".to_string())
    }

    /// Ask LLM a question with tier context.
    pub fn ask_llm(&self, question: &str) -> String {
        // For Mini tier, include activity + HR context in prompt
        let response = if let (Tier::Mini8, Some(ctx)) = (self.tier, self.activity_context.clone()) {
            let (system, user) = build_mini_prompt(&ctx, self.baseline_hr);
            let prompt = Prompt { tier: self.tier, user: &user, system: &system };
            self.llm.run(prompt)
        } else {
            let prompt = Prompt {
                tier: self.tier,
                user: question,
                system: "You are a health assistant. Keep answers brief.",
            };
            self.llm.run(prompt)
        };
        response.text
    }
}
