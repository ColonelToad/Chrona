//! Example: End-to-end 8GB tier pipeline using MHEALTH data
//! 
//! This demonstrates:
//! 1. Load MHEALTH CSV file
//! 2. Classify activities using sensor data
//! 3. Predict HR from activity intensity
//! 4. Build LLM context
//! 5. Generate simple status prompts

use data_layer::MhealthRecord;
use logic::{ActivityContext, build_mini_prompt};
use std::fs;

fn main() {
    println!("=== 8GB Tier: MHEALTH Activity + LLM Pipeline ===\n");

    // Load MHEALTH CSV (try multiple possible paths)
    let possible_paths = vec![
        "C:\\Users\\legot\\Chrona\\data\\mini\\mhealth_raw_data.csv",
        "data\\mini\\mhealth_raw_data.csv",
        "..\\..\\data\\mini\\mhealth_raw_data.csv",
    ];

    let csv_content = possible_paths
        .iter()
        .find_map(|p| fs::read_to_string(p).ok())
        .expect("Could not find MHEALTH CSV in any expected location");

    let lines: Vec<&str> = csv_content.lines().collect();
    println!("Loaded {} records from MHEALTH\n", lines.len());

    // Process first 50Hz window (1 second of data at 50Hz sampling)
    let window_size = 50;
    let test_window = &lines[0..window_size.min(lines.len())];

    println!("Processing {} samples (1 second window):\n", test_window.len());

    let mut records = Vec::new();
    for (idx, line) in test_window.iter().enumerate() {
        if let Some(record) = MhealthRecord::from_csv_line(line) {
            if idx < 3 || idx == test_window.len() - 1 {
                println!(
                    "  Sample {}: {} (subject {})",
                    idx, record.activity_name(), record.subject
                );
            } else if idx == 3 {
                println!("  ... {} more samples ...", test_window.len() - 4);
            }
            records.push(record);
        }
    }

    println!("\n--- Activity Classification ---");

    // Create activity context
    let baseline_hr = 70; // Typical resting HR
    if let Some(first_record) = records.first() {
        let context = ActivityContext::from_record(first_record, baseline_hr);

        println!("Activity: {}", context.activity_name);
        println!("Confidence: {:.1}%", context.confidence * 100.0);
        println!("Intensity: {:.1}%", context.intensity * 100.0);
        println!("Predicted HR: {} bpm", context.predicted_hr);
        println!();

        println!("--- LLM Context ---");
        let llm_context = context.to_llm_context(baseline_hr);
        println!("{}", llm_context);
        println!();

        println!("--- 8GB LLM Prompt (128 tokens max) ---");
        let (system, user) = build_mini_prompt(&context, baseline_hr);
        println!("System: {}", system);
        println!("User: {}", user);
    }

    println!("\n=== Test Complete ===");
    println!("Next: Wire this into the UI and LLM runtime");
}
