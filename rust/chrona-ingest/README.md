# chrona-ingest

Synthetic data ingestion pipeline for Chrona.

## Features
- Generates synthetic sensor data for multiple user profiles
- Stores session metadata and sensor data in SQLite (legacy)
- Batches and writes sensor data to Parquet files (recommended)

## Usage
Run:

    cargo run -p chrona-ingest

This will generate 60 seconds of synthetic data for each preset profile and write the results to Parquet files in the workspace root.

## Output
- Parquet files: `sensor_data_<session_id>_<profile>.parquet`
- SQLite DB: `test_sensor_data.sqlite3` (for session metadata and legacy queries)

## See Also
- [data-layer/README.md](../data-layer/README.md)
- [../README.md](../README.md)
