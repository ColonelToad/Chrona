//! Example: ingest synthetic sensor data into SQLite.

// ...existing code...
use data_layer::sqlite;
use rusqlite::Connection;
// ...existing code...

fn main() -> anyhow::Result<()> {
    // Open or create SQLite DB
    let conn = Connection::open("test_sensor_data.sqlite3")?;
    sqlite::init_db(&conn)?;
    println!("Database initialized. Sensor simulation is now handled in chrona-ingest.");
    Ok(())
}
