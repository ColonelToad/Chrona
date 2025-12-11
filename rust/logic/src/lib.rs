#![deny(missing_docs)]

//! Orchestration logic tying sensors, storage, ML, and LLMs together.

use core_types::{Sample, Tier};
use data_layer::TimeSeriesStore;
use llm_runtime::{LlmEngine, Prompt, Response};
use ml_runtime::Model;
use sensors::Sensor;

/// High-level orchestrator for one tier.
pub struct Engine<S, T, M, L>
where
    S: Sensor,
    T: TimeSeriesStore,
    M: Model,
    L: LlmEngine,
{
    pub tier: Tier,
    pub sensor: S,
    pub store: T,
    pub model: M,
    pub llm: L,
}

impl<S, T, M, L> Engine<S, T, M, L>
where
    S: Sensor,
    T: TimeSeriesStore,
    M: Model,
    L: LlmEngine,
{
    /// Poll sensor, store data, run model, and send a lightweight prompt.
    pub fn step(&mut self) -> Response {
        if let Some(sample) = self.sensor.poll() {
            self.store.write(self.sensor.name(), sample.clone());
            let score = self.model.infer(&[sample.clone()]);
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
