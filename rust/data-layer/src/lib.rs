#![deny(missing_docs)]

//! Data access layer abstractions (SQLite/DuckDB/etc.).

use core_types::Sample;

/// Minimal interface for time-series storage.
pub trait TimeSeriesStore {
    /// Write a sample into a named series.
    fn write(&mut self, series: &str, sample: Sample);
    /// Read the latest `count` samples from a series.
    fn read_latest(&self, series: &str, count: usize) -> Vec<Sample>;
}

/// No-op store for early wiring and tests.
pub struct NoopStore;

impl TimeSeriesStore for NoopStore {
    fn write(&mut self, _series: &str, _sample: Sample) {}

    fn read_latest(&self, _series: &str, _count: usize) -> Vec<Sample> {
        Vec::new()
    }
}
