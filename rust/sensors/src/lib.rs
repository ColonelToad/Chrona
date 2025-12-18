#![deny(missing_docs)]

//! Sensor providers: API-backed, synthetic, or file replay.

use core_types::Sample;
use data_layer::{MhealthRecord, TimeSeriesStore};
use rand::Rng;

/// Generic sensor interface.
pub trait Sensor {
    /// Human-readable name.
    fn name(&self) -> &str;
    /// Poll for the next sample (non-blocking).
    fn poll(&mut self) -> Option<Sample>;
}

/// MHEALTH sensor streamer - cycles through records with added noise.
pub struct MhealthStreamingSensor {
    records: Vec<MhealthRecord>,
    current_index: usize,
    ts_ms: i64,
}

impl MhealthStreamingSensor {
    /// Create from MHEALTH CSV file.
    pub fn from_csv(csv_content: &str) -> Self {
        let mut records = Vec::new();
        for line in csv_content.lines() {
            if let Some(rec) = MhealthRecord::from_csv_line(line) {
                records.push(rec);
            }
        }
        
        let start_ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        
        Self {
            records,
            current_index: 0,
            ts_ms: start_ts,
        }
    }
    
    /// Get current MHEALTH record.
    pub fn current_record(&self) -> Option<&MhealthRecord> {
        self.records.get(self.current_index)
    }
    
    /// Get 1-second window (50 samples at 50Hz) for activity classification.
    pub fn get_window(&self) -> Vec<MhealthRecord> {
        let start = self.current_index;
        let end = (start + 50).min(self.records.len());
        self.records[start..end].to_vec()
    }
}

impl Sensor for MhealthStreamingSensor {
    fn name(&self) -> &str {
        "mhealth_stream"
    }
    
    fn poll(&mut self) -> Option<Sample> {
        if self.records.is_empty() {
            return None;
        }
        
        let record = &self.records[self.current_index];
        
        // Predict HR from activity with some natural variance
        let baseline_hr = 70.0;
        let max_hr = 180.0;
        let intensity = record.intensity();
        let predicted_hr = baseline_hr + (max_hr - baseline_hr) * intensity;
        
        // Add natural variance (±5 bpm)
        let mut rng = rand::thread_rng();
        let noise = rng.gen_range(-5.0..5.0);
        let hr_with_noise = predicted_hr + noise;
        
        // Advance to next record (cycle through dataset)
        self.current_index = (self.current_index + 1) % self.records.len();
        
        self.ts_ms += 1000; // 1 second increment
        
        Some(Sample {
            ts_ms: self.ts_ms,
            value: hr_with_noise,
        })
    }
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

/// Helper to push a polled sample into storage.
pub fn poll_into<S: Sensor, T: TimeSeriesStore>(sensor: &mut S, store: &mut T) {
    if let Some(sample) = sensor.poll() {
        store.write(sensor.name(), sample);
    }
}
