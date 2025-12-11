#![deny(missing_docs)]

//! Shared types for tiers, signals, and metadata.

/// Supported device tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    /// 8 GB configuration.
    Mini8,
    /// 16 GB configuration.
    Standard16,
    /// 32 GB configuration.
    Pro32,
}

/// A basic time-series sample.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sample {
    /// Milliseconds since epoch.
    pub ts_ms: i64,
    /// Numeric value for the stream.
    pub value: f32,
}
