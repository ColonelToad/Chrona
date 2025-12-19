//! Data access layer abstractions (SQLite/DuckDB/etc.).
#![deny(missing_docs)]
/// Data access layer abstractions (SQLite/DuckDB/etc.).
pub mod query;
pub mod sqlite;
pub mod profile;
pub use profile::*;

// ...existing code...

