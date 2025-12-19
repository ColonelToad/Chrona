


use core_types::Sample;
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
    /// Baseline value in bpm.
    pub baseline: f32,
    /// Random jitter range (+/-).
    pub jitter: f32,
    /// Current timestamp in milliseconds.
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

