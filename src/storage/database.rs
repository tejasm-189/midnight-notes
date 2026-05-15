use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

use super::migrations;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("migration error: {0}")]
    Migration(#[from] migrations::MigrationError),
}

pub struct Database {
    conn: Mutex<rusqlite::Connection>,
    encryption_key: Mutex<Option<[u8; 32]>>,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Database {
    /// Open or create a SQLite database at the given path.
    /// Note content is encrypted at the application layer via `crypto::cipher`.
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        let conn = rusqlite::Connection::open(path)?;

        // Enable WAL mode for better concurrent access
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        migrations::run(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
            encryption_key: Mutex::new(None),
        })
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> Result<Self, DatabaseError> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        migrations::run(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            encryption_key: Mutex::new(None),
        })
    }

    /// Get a reference to the underlying connection.
    pub fn conn(&self) -> std::sync::MutexGuard<'_, rusqlite::Connection> {
        self.conn.lock().unwrap()
    }

    /// Set the encryption key used for encrypting/decrypting note content.
    pub fn set_encryption_key(&self, key: [u8; 32]) {
        let mut k = self.encryption_key.lock().unwrap();
        *k = Some(key);
    }

    /// Get the encryption key, if set.
    pub fn encryption_key(&self) -> [u8; 32] {
        let k = self.encryption_key.lock().unwrap();
        k.unwrap_or([0u8; 32])
    }

    /// Check if encryption is enabled.
    pub fn is_encryption_enabled(&self) -> bool {
        self.encryption_key.lock().unwrap().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_open_and_create() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        let conn = db.conn();
        let count: i64 = conn
            .query_row("SELECT count(*) FROM notes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory().unwrap();
        let conn = db.conn();
        let count: i64 = conn
            .query_row("SELECT count(*) FROM notes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_open_twice() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test2.db");
        Database::open(&db_path).unwrap();
        Database::open(&db_path).unwrap(); // second open should work
    }
}
