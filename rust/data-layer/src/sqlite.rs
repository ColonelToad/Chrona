//! SQLite ingestion and persistence for user_sessions and sensor_data.

use rusqlite::{params, Connection, Result};

/// Initialize the SQLite database with required tables and index.
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS user_sessions (
            session_id INTEGER PRIMARY KEY,
            user_id TEXT NOT NULL,
            profile_name TEXT,
            simulation_date TEXT,
            seed INTEGER,
            created_at TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS sensor_data (
            id INTEGER PRIMARY KEY,
            session_id INTEGER,
            ts_unix_sec INTEGER,
            hr REAL, hrv_rmssd REAL, eda_mus REAL, temp_c REAL, accel_mag_g REAL,
            activity TEXT,
            stress_level REAL,
            exercise_flag BOOLEAN,
            created_at TIMESTAMP,
            FOREIGN KEY(session_id) REFERENCES user_sessions(session_id)
        );
        CREATE INDEX IF NOT EXISTS idx_session_ts ON sensor_data(session_id, ts_unix_sec);
        "#
    )
}

/// Insert a new user session and return its session_id.
pub fn insert_user_session(
    conn: &Connection,
    user_id: &str,
    profile_name: &str,
    simulation_date: &str,
    seed: i64,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO user_sessions (user_id, profile_name, simulation_date, seed, created_at) VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)",
        params![user_id, profile_name, simulation_date, seed],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Insert a sensor data row.
pub fn insert_sensor_data(
    conn: &Connection,
    session_id: i64,
    ts_unix_sec: i64,
    hr: f32,
    hrv_rmssd: f32,
    eda_mus: f32,
    temp_c: f32,
    accel_mag_g: f32,
    activity: &str,
    stress_level: f32,
    exercise_flag: bool,
) -> Result<()> {
    conn.execute(
        "INSERT INTO sensor_data (session_id, ts_unix_sec, hr, hrv_rmssd, eda_mus, temp_c, accel_mag_g, activity, stress_level, exercise_flag, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, CURRENT_TIMESTAMP)",
        params![session_id, ts_unix_sec, hr, hrv_rmssd, eda_mus, temp_c, accel_mag_g, activity, stress_level, exercise_flag],
    )?;
    Ok(())
}
