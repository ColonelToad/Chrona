#![deny(missing_docs)]

//! Orchestration logic tying sensors, storage, ML, and LLMs together.

use core_types::Tier;
// use data_layer::{MhealthRecord, TimeSeriesStore};
use llm_runtime::{LlmEngine, Prompt, Response};
use ml_runtime::Model;
use sensors::Sensor;


/// High-level orchestrator for one tier.
pub struct Engine<S, T, M, L>
where
    S: Sensor,
    // T: TimeSeriesStore,
    M: Model,
    L: LlmEngine,
{
    /// The tier this engine represents.
    pub tier: Tier,
    /// The sensor instance.
    pub sensor: S,
    /// The time-series storage.
    pub store: T,
    /// The ML model.
    pub model: M,
    /// The LLM engine.
    pub llm: L,
}

impl<S, T, M, L> Engine<S, T, M, L>
where
    S: Sensor,
    // T: TimeSeriesStore,
    M: Model,
    L: LlmEngine,
{
    /// Poll sensor, store data, run model, and send a lightweight prompt.
    pub fn step(&mut self) -> Response {
        if let Some(sample) = self.sensor.poll() {
            // self.store.write(self.sensor.name(), sample.clone());
            let _score = self.model.infer(&[sample.clone()]);
            let prompt = Prompt {
                tier: self.tier,
                user: "Explain the latest score",
                system: "Keep it short",
            };
            self.llm.run(prompt)
        } else {
            Response { model: self.llm.model_id().to_string(), text: String::from("No data") }
        }
    }
}

/// Activity context for 8GB tier (current activity + derived HR).
#[derive(Debug, Clone)]
pub struct ActivityContext {
    /// Activity label (1-12, L1-L12).
    pub activity: u8,
    /// Activity name (L1: Standing still, etc.).
    pub activity_name: String,
    /// Confidence of activity classification (0.0-1.0).
    pub confidence: f32,
    /// Predicted heart rate based on activity intensity.
    pub predicted_hr: u32,
    /// Intensity level (0.0-1.0) for the activity.
    pub intensity: f32,
}

impl ActivityContext {
    /// Create context from MHEALTH record and resting HR.
        // /// Create context from MHEALTH record and resting HR.
        // pub fn from_record(record: &MhealthRecord, resting_hr: u32) -> Self {
        //     let (activity, confidence) = ActivityClassifier::classify(&[record.clone()]);
        //     let intensity = record.intensity();
        //     
        //     // Predict HR: resting + (max_hr - resting) * intensity
        //     // Assume max HR ~= 180 for typical person
        //     let max_hr = 180;
        //     let predicted_hr = (resting_hr as f32 + (max_hr - resting_hr) as f32 * intensity) as u32;
        //
        //     ActivityContext {
        //         activity,
        //         activity_name: record.activity_name().to_string(),
        //         confidence,
        //         predicted_hr,
        //         intensity,
        //     }
        // }

    /// Generate LLM prompt for 8GB tier.
    /// Returns a context string for the LLM about current activity and HR.
    pub fn to_llm_context(&self, baseline_hr: u32) -> String {
        let delta = self.predicted_hr as i32 - baseline_hr as i32;
        let delta_desc = if delta > 10 {
            "significantly elevated"
        } else if delta > 0 {
            "elevated"
        } else {
            "normal"
        };

        format!(
            "Activity: {} (confidence {:.0}%). HR: {} bpm, {}.",
            self.activity_name, self.confidence * 100.0, self.predicted_hr, delta_desc
        )
    }
}

/// Build a 128-token prompt for 8GB LLM (stateless responder).
/// 
/// Returns a tuple of (system_prompt, user_prompt) since Prompt uses borrowed strings.
pub fn build_mini_prompt(activity_context: &ActivityContext, baseline_hr: u32) -> (String, String) {
    let context = activity_context.to_llm_context(baseline_hr);
    let user = format!("{}. Brief status check in one sentence.", context);
    let system = "You are a health assistant. Provide a very concise status. Max one sentence.".to_string();
    
    (system, user)
}

