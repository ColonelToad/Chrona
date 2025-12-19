//! Example: ingest synthetic sensor data into SQLite.

use data_layer::profile::presets;
use data_layer::sqlite;
use rusqlite::Connection;
mod simulator;
use simulator::SensorSimulator;
use data_layer::parquet::write_sensor_data_parquet;
use std::time::{SystemTime, UNIX_EPOCH};
mod llm_context;
use data_layer::query::query_latest_sensor_data;
use llm_context::LlmSensorContext;

fn main() -> anyhow::Result<()> {
    // Open or create SQLite DB
    let conn = Connection::open("test_sensor_data.sqlite3")?;
    sqlite::init_db(&conn)?;

    // List of preset profiles to cycle through
    let profiles = vec![
        ("BusinessProfessional", presets::business_professional()),
        ("CollegeStudent", presets::college_student()),
        ("ProAthlete", presets::pro_athlete()),
        ("ShiftWorker", presets::shift_worker()),
        ("RemoteWorker", presets::remote_worker()),
    ];

    for (profile_name, profile) in profiles {
        let user_id = profile_name;
        let simulation_date = "2025-12-19";
        let seed = 42;
        let session_id = sqlite::insert_user_session(&conn, user_id, profile_name, simulation_date, seed)?;

        // Start time: midnight UTC
        let start_ts = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(0);
        let start_ts_ms = start_ts.duration_since(UNIX_EPOCH)?.as_millis() as i64;
        let mut sim = SensorSimulator::new(profile, start_ts_ms);

        // Prepare in-memory vectors for Parquet
        let mut session_ids = Vec::with_capacity(60);
        let mut ts_unix_secs = Vec::with_capacity(60);
        let mut hrs = Vec::with_capacity(60);
        let mut hrv_rmssds = Vec::with_capacity(60);
        let mut eda_muss = Vec::with_capacity(60);
        let mut temp_cs = Vec::with_capacity(60);
        let mut accel_mag_gs = Vec::with_capacity(60);
        let mut activities = Vec::with_capacity(60);
        let mut stress_levels = Vec::with_capacity(60);
        let mut exercise_flags = Vec::with_capacity(60);

        // Ingest 60 seconds of data as a demo
        for _ in 0..60 {
            let samples = sim.poll_all();
            let ts_unix_sec = sim.ts_ms / 1000;
            // SQLite insert (legacy/optional)
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
            // Parquet batch
            session_ids.push(session_id);
            ts_unix_secs.push(ts_unix_sec);
            hrs.push(samples[0].value);
            hrv_rmssds.push(samples[1].value);
            eda_muss.push(samples[2].value);
            temp_cs.push(samples[3].value);
            accel_mag_gs.push(samples[4].value);
            activities.push("sitting".to_string());
            stress_levels.push(0.25);
            exercise_flags.push(false);
        }
        // Write to Parquet
        let parquet_path = format!("sensor_data_{}_{}.parquet", session_id, profile_name);
        write_sensor_data_parquet(
            &parquet_path,
            &session_ids,
            &ts_unix_secs,
            &hrs,
            &hrv_rmssds,
            &eda_muss,
            &temp_cs,
            &accel_mag_gs,
            &activities,
            &stress_levels,
            &exercise_flags,
        )?;
        println!("Wrote 60 seconds of synthetic data to Parquet for session_id {} (profile: {})", session_id, profile_name);

        // Query the latest row and build LLM context
        let latest = query_latest_sensor_data(&conn, session_id, 1)?;
        if let Some(row) = latest.first() {
            let ctx = LlmSensorContext::from_sensor_row(row);
            let llm_prompt = ctx.to_llm_context();
            println!("LLM prompt context for {}: {}", profile_name, llm_prompt);
        } else {
            println!("No sensor data found for session {}", profile_name);
        }
    }
    Ok(())
}
