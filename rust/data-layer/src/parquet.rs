//! Parquet/Arrow ingestion and persistence for sensor data.

use arrow::array::{Float32Array, Int64Array, StringArray, BooleanArray, ArrayRef};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;

/// Write a batch of sensor data to a Parquet file.
pub fn write_sensor_data_parquet(
    path: &str,
    session_id: &[i64],
    ts_unix_sec: &[i64],
    hr: &[f32],
    hrv_rmssd: &[f32],
    eda_mus: &[f32],
    temp_c: &[f32],
    accel_mag_g: &[f32],
    activity: &[String],
    stress_level: &[f32],
    exercise_flag: &[bool],
) -> anyhow::Result<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("session_id", DataType::Int64, false),
        Field::new("ts_unix_sec", DataType::Int64, false),
        Field::new("hr", DataType::Float32, false),
        Field::new("hrv_rmssd", DataType::Float32, false),
        Field::new("eda_mus", DataType::Float32, false),
        Field::new("temp_c", DataType::Float32, false),
        Field::new("accel_mag_g", DataType::Float32, false),
        Field::new("activity", DataType::Utf8, false),
        Field::new("stress_level", DataType::Float32, false),
        Field::new("exercise_flag", DataType::Boolean, false),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(session_id.to_vec())),
            Arc::new(Int64Array::from(ts_unix_sec.to_vec())),
            Arc::new(Float32Array::from(hr.to_vec())),
            Arc::new(Float32Array::from(hrv_rmssd.to_vec())),
            Arc::new(Float32Array::from(eda_mus.to_vec())),
            Arc::new(Float32Array::from(temp_c.to_vec())),
            Arc::new(Float32Array::from(accel_mag_g.to_vec())),
            Arc::new(StringArray::from(activity.to_vec())),
            Arc::new(Float32Array::from(stress_level.to_vec())),
            Arc::new(BooleanArray::from(exercise_flag.to_vec())),
        ],
    )?;
    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}
