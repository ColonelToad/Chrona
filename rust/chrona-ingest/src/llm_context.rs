//! Build LLM context from latest sensor_data row.

use data_layer::query::SensorDataRow;

/// Context for LLM prompt, built from sensor_data.
#[derive(Debug, Clone)]
pub struct LlmSensorContext {
    pub activity: String,
    pub hr: f32,
    pub hrv_rmssd: f32,
    pub eda_mus: f32,
    pub temp_c: f32,
    pub accel_mag_g: f32,
    pub stress_level: f32,
    pub exercise_flag: bool,
}

impl LlmSensorContext {
    pub fn from_sensor_row(row: &SensorDataRow) -> Self {
        Self {
            activity: row.activity.clone(),
            hr: row.hr,
            hrv_rmssd: row.hrv_rmssd,
            eda_mus: row.eda_mus,
            temp_c: row.temp_c,
            accel_mag_g: row.accel_mag_g,
            stress_level: row.stress_level,
            exercise_flag: row.exercise_flag,
        }
    }

    /// Render a concise context string for the LLM.
    pub fn to_llm_context(&self) -> String {
        format!(
            "Activity: {}. HR: {:.0} bpm. HRV: {:.0} ms. EDA: {:.2} μS. Temp: {:.2}°C. Accel: {:.2}g. Stress: {:.2}. Exercise: {}.",
            self.activity,
            self.hr,
            self.hrv_rmssd,
            self.eda_mus,
            self.temp_c,
            self.accel_mag_g,
            self.stress_level,
            if self.exercise_flag { "yes" } else { "no" }
        )
    }
}
