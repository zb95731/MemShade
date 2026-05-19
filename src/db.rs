use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct ImportRecord {
    pub id: String,
    pub file_name: String,
    pub hash: String,
    pub import_time: String,
    pub export_path: String,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("photo_organizer.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS import_records (
            id TEXT PRIMARY KEY,
            file_name TEXT NOT NULL,
            hash TEXT NOT NULL UNIQUE,
            import_time TEXT NOT NULL,
            export_path TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn check_duplicate(conn: &Connection, hash: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM import_records WHERE hash = ?1",
        [hash],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

pub fn insert_record(conn: &Connection, record: &ImportRecord) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO import_records (id, file_name, hash, import_time, export_path)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &record.id,
            &record.file_name,
            &record.hash,
            &record.import_time,
            &record.export_path,
        ),
    )?;
    
    Ok(())
}