# Data Layer

This crate provides persistent storage for Chrona sensor and session data.

## Features
- SQLite storage for session metadata and sensor data (legacy, for compatibility)
- Parquet/Arrow batch writer for efficient, columnar sensor data storage (recommended)

## Usage
- Use `write_sensor_data_parquet` to write batches of sensor data to Parquet files for analytics and scalable storage.
- SQLite is still supported for session metadata and legacy queries.

## Dependencies
- [arrow](https://crates.io/crates/arrow)
- [parquet](https://crates.io/crates/parquet)
- [rusqlite](https://crates.io/crates/rusqlite) (with `bundled` feature for Windows portability)

## Example
See `chrona-ingest` for a working example of generating synthetic data and writing to Parquet.
