#![deny(missing_docs)]

//! Data access layer abstractions (SQLite/DuckDB/etc.).

use core_types::Sample;

/// Minimal interface for time-series storage.
pub trait TimeSeriesStore {
    /// Write a sample into a named series.
    fn write(&mut self, series: &str, sample: Sample);
    /// Read the latest `count` samples from a series.
    fn read_latest(&self, series: &str, count: usize) -> Vec<Sample>;
}

/// No-op store for early wiring and tests.
pub struct NoopStore;

impl TimeSeriesStore for NoopStore {
    fn write(&mut self, _series: &str, _sample: Sample) {}

    fn read_latest(&self, _series: &str, _count: usize) -> Vec<Sample> {
        Vec::new()
    }
}

/// MHEALTH dataset record (one sensor reading).
#[derive(Debug, Clone)]
pub struct MhealthRecord {
    /// Left ankle acceleration X
    pub al_x: f32,
    /// Left ankle acceleration Y
    pub al_y: f32,
    /// Left ankle acceleration Z
    pub al_z: f32,
    /// Left ankle gyro X
    pub gl_x: f32,
    /// Left ankle gyro Y
    pub gl_y: f32,
    /// Left ankle gyro Z
    pub gl_z: f32,
    /// Right arm acceleration X
    pub ar_x: f32,
    /// Right arm acceleration Y
    pub ar_y: f32,
    /// Right arm acceleration Z
    pub ar_z: f32,
    /// Right arm gyro X
    pub gr_x: f32,
    /// Right arm gyro Y
    pub gr_y: f32,
    /// Right arm gyro Z
    pub gr_z: f32,
    /// Subject ID (1-10)
    pub subject: u32,
    /// Activity label (L1-L12, stored as 1-12)
    pub activity: u8,
}

impl MhealthRecord {
    /// Parse a CSV line into MhealthRecord.
    /// Expected format: alx,aly,alz,glx,gly,glz,arx,ary,arz,grx,gry,grz,Activity,subject
    pub fn from_csv_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() < 14 {
            return None;
        }

        let parse_f32 = |s: &str| s.trim().parse::<f32>().ok();

        let activity = parts[12].trim().parse::<u8>().ok()?;
        let subject_str = parts[13].trim();
        // Parse subject from "subject1", "subject2", etc.
        let subject = subject_str
            .strip_prefix("subject")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        Some(MhealthRecord {
            al_x: parse_f32(parts[0])?,
            al_y: parse_f32(parts[1])?,
            al_z: parse_f32(parts[2])?,
            gl_x: parse_f32(parts[3])?,
            gl_y: parse_f32(parts[4])?,
            gl_z: parse_f32(parts[5])?,
            ar_x: parse_f32(parts[6])?,
            ar_y: parse_f32(parts[7])?,
            ar_z: parse_f32(parts[8])?,
            gr_x: parse_f32(parts[9])?,
            gr_y: parse_f32(parts[10])?,
            gr_z: parse_f32(parts[11])?,
            subject,
            activity: activity + 1, // Convert 0-11 to 1-12
        })
    }

    /// Get activity name (L1-L12).
    pub fn activity_name(&self) -> &'static str {
        match self.activity {
            1 => "L1: Standing still",
            2 => "L2: Sitting and relaxing",
            3 => "L3: Lying down",
            4 => "L4: Walking",
            5 => "L5: Climbing stairs",
            6 => "L6: Waist bends forward",
            7 => "L7: Frontal elevation of arms",
            8 => "L8: Knees bending (crouching)",
            9 => "L9: Cycling",
            10 => "L10: Jogging",
            11 => "L11: Running",
            12 => "L12: Jump front & back",
            _ => "Unknown",
        }
    }

    /// Estimated intensity (0.0-1.0) based on activity type.
    /// Used to predict heart rate from activity.
    pub fn intensity(&self) -> f32 {
        match self.activity {
            1 | 2 | 3 => 0.1,      // Standing, sitting, lying = low
            4 | 7 => 0.3,          // Walking, arm elevation = moderate
            5 | 9 => 0.5,          // Climbing, cycling = moderate-high
            6 | 8 => 0.6,          // Bending, crouching = high
            10 => 0.7,             // Jogging = high
            11 | 12 => 0.9,        // Running, jumping = very high
            _ => 0.5,
        }
    }
}
