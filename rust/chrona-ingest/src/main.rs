//! Example: ingest synthetic sensor data into SQLite.

use data_layer::profile::presets;
use data_layer::sqlite;
use rusqlite::Connection;
mod simulator;
use simulator::SensorSimulator;
use std::time::{SystemTime, UNIX_EPOCH};
mod llm_context;
use data_layer::query::query_latest_sensor_data;
use llm_context::LlmSensorContext;

fn main() -> anyhow::Result<()> {
    // Open or create SQLite DB
    let conn = Connection::open("test_sensor_data.sqlite3")?;
    sqlite::init_db(&conn)?;

    // Create a user session
    let user_id = "user1";
    let profile_name = "BusinessProfessional";
    let simulation_date = "2025-12-19";
    let seed = 42;
    let session_id = sqlite::insert_user_session(&conn, user_id, profile_name, simulation_date, seed)?;

    // Start time: midnight UTC
    let start_ts = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(0);
    let start_ts_ms = start_ts.duration_since(UNIX_EPOCH)?.as_millis() as i64;
    let mut sim = SensorSimulator::new(presets::business_professional(), start_ts_ms);

    // Ingest 60 seconds of data as a demo
    for _ in 0..60 {
        let samples = sim.poll_all();
        // Order: HR, HRV, EDA, Temp, Accel
        let ts_unix_sec = sim.ts_ms / 1000;
        sqlite::insert_sensor_data(
            &conn,
            session_id,
            ts_unix_sec,
            samples[0].value, // hr
            samples[1].value, // hrv_rmssd
            samples[2].value, // eda_mus
            samples[3].value, // temp_c
            samples[4].value, // accel_mag_g
            "sitting",       // activity (placeholder)
            0.25,             // stress_level (placeholder)
            false,            // exercise_flag (placeholder)
        )?;
    }
    println!("Ingested 60 seconds of synthetic data for session_id {}", session_id);

    // Query the latest row and build LLM context
    let latest = query_latest_sensor_data(&conn, session_id, 1)?;
    if let Some(row) = latest.first() {
        let ctx = LlmSensorContext::from_sensor_row(row);
        let llm_prompt = ctx.to_llm_context();
        println!("LLM prompt context: {}", llm_prompt);
    } else {
        println!("No sensor data found for session");
    }
    Ok(())
}
