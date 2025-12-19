//! Synthetic sensor simulator driven by UserProfile and schedule.

use data_layer::profile::{UserProfile, activity_hr_boost, activity_intensity, activity_accel_mean, activity_accel_noise};
use core_types::Sample;
use rand::Rng;

pub struct SensorSimulator {
    pub profile: UserProfile,
    pub ts_ms: i64,
    pub minute_of_day: u16,
}

impl SensorSimulator {
    /// Create a new SensorSimulator starting at the given timestamp (ms since epoch).
    pub fn new(profile: UserProfile, start_ts_ms: i64) -> Self {
        // Compute minute of day from timestamp
        let minute_of_day = ((start_ts_ms / 60000) % 1440) as u16;
        Self {
            profile,
            ts_ms: start_ts_ms,
            minute_of_day,
        }
    }

    /// Poll all signals for the current time step (HR, HRV, EDA, temp, accel).
    pub fn poll_all(&mut self) -> Vec<Sample> {
        // Advance time by 1 second
        self.ts_ms += 1000;
        self.minute_of_day = ((self.ts_ms / 60000) % 1440) as u16;

        let activity = self.profile.activity_schedule.activity_at_minute(self.minute_of_day);
        let mut rng = rand::rng();

        // HR
        let hr = self.profile.resting_hr + activity_hr_boost(activity, self.profile.resting_hr, self.profile.max_hr)
            + rng.random_range(-2.0..2.0); // add small noise
        // HRV
        let intensity = activity_intensity(activity);
        let hrv = self.profile.hrv_baseline * (1.0 - 0.5 * intensity)
            + rng.random_range(-3.0..3.0);
        // EDA
        let eda = self.profile.baseline_eda + self.profile.stress_sensitivity * intensity
            + rng.random_range(-0.05..0.05);
        // Temp
        let temp = self.profile.baseline_temp - 0.2 * intensity
            + rng.random_range(-0.05..0.05);
        // Accel
        let accel_mean = activity_accel_mean(activity);
        let accel_noise = activity_accel_noise(activity);
        let accel = accel_mean + rng.random_range(-accel_noise..accel_noise);

        let ts = self.ts_ms;
        vec![
            Sample { ts_ms: ts, value: hr },
            Sample { ts_ms: ts, value: hrv },
            Sample { ts_ms: ts, value: eda },
            Sample { ts_ms: ts, value: temp },
            Sample { ts_ms: ts, value: accel },
        ]
    }
}
