#![deny(missing_docs)]

//! Sensor providers: API-backed, synthetic, or file replay.

use core_types::Sample;
use data_layer::TimeSeriesStore;
use rand::Rng;

/// Generic sensor interface.
pub trait Sensor {
    /// Human-readable name.
    fn name(&self) -> &str;
    /// Poll for the next sample (non-blocking).
    fn poll(&mut self) -> Option<Sample>;
}

/// Simple synthetic heart-rate-like generator.
pub struct SyntheticHeartRate {
    pub baseline: f32,
    pub jitter: f32,
    pub ts_ms: i64,
}

impl SyntheticHeartRate {
    /// Create a new generator.
    pub fn new(baseline: f32, jitter: f32, start_ts_ms: i64) -> Self {
        Self { baseline, jitter, ts_ms: start_ts_ms }
    }
}

impl Sensor for SyntheticHeartRate {
    fn name(&self) -> &str {
        "synthetic_hr"
    }

    fn poll(&mut self) -> Option<Sample> {
        let mut rng = rand::thread_rng();
        let delta = rng.gen_range(-self.jitter..self.jitter);
        self.ts_ms += 1000;
        Some(Sample { ts_ms: self.ts_ms, value: self.baseline + delta })
    }
}

/// Helper to push a polled sample into storage.
pub fn poll_into<S: Sensor, T: TimeSeriesStore>(sensor: &mut S, store: &mut T) {
    if let Some(sample) = sensor.poll() {
        store.write(sensor.name(), sample);
    }
}
