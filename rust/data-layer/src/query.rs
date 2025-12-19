//! Query functions for sensor_data in SQLite.

use rusqlite::{Connection, Result, Row};

/// A single row of sensor data from the database.
#[derive(Debug, Clone)]
pub struct SensorDataRow {
    /// Unix timestamp (seconds since epoch).
    pub ts_unix_sec: i64,
    /// Heart rate (bpm).
    pub hr: f32,
    /// Heart rate variability (RMSSD, ms).
    pub hrv_rmssd: f32,
    /// Electrodermal activity (μS).
    pub eda_mus: f32,
    /// Skin temperature (°C).
    pub temp_c: f32,
    /// Accelerometer magnitude (g).
    pub accel_mag_g: f32,
    /// Activity label (string).
    pub activity: String,
    /// Stress level (0.0–1.0).
    pub stress_level: f32,
    /// True if exercising during this sample.
    pub exercise_flag: bool,
}

impl SensorDataRow {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            ts_unix_sec: row.get("ts_unix_sec")?,
            hr: row.get("hr")?,
            hrv_rmssd: row.get("hrv_rmssd")?,
            eda_mus: row.get("eda_mus")?,
            temp_c: row.get("temp_c")?,
            accel_mag_g: row.get("accel_mag_g")?,
            activity: row.get("activity")?,
            stress_level: row.get("stress_level")?,
            exercise_flag: row.get("exercise_flag")?,
        })
    }
}

/// Query the latest N sensor data rows for a session.
pub fn query_latest_sensor_data(conn: &Connection, session_id: i64, n: usize) -> Result<Vec<SensorDataRow>> {
    let mut stmt = conn.prepare(
        "SELECT ts_unix_sec, hr, hrv_rmssd, eda_mus, temp_c, accel_mag_g, activity, stress_level, exercise_flag \
         FROM sensor_data WHERE session_id = ?1 ORDER BY ts_unix_sec DESC LIMIT ?2"
    )?;
    let rows = stmt.query_map([session_id, n as i64], |row| SensorDataRow::from_row(row))?;
    Ok(rows.filter_map(Result::ok).collect())
}

/// Query sensor data for a session in a time range (inclusive).
pub fn query_sensor_data_range(conn: &Connection, session_id: i64, start_ts: i64, end_ts: i64) -> Result<Vec<SensorDataRow>> {
    let mut stmt = conn.prepare(
        "SELECT ts_unix_sec, hr, hrv_rmssd, eda_mus, temp_c, accel_mag_g, activity, stress_level, exercise_flag \
         FROM sensor_data WHERE session_id = ?1 AND ts_unix_sec >= ?2 AND ts_unix_sec <= ?3 ORDER BY ts_unix_sec ASC"
    )?;
    let rows = stmt.query_map([session_id, start_ts, end_ts], |row| SensorDataRow::from_row(row))?;
    Ok(rows.filter_map(Result::ok).collect())
}
