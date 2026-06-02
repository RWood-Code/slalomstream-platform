use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

use crate::error::{CoreError, Result};

const MIGRATIONS: &str = include_str!("migrations/001_init.sql");

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                CoreError::Validation(format!("create data dir: {e}"))
            })?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| {
            CoreError::Validation("database lock poisoned".into())
        })?;
        conn.execute_batch(MIGRATIONS)?;
        Ok(())
    }

    pub fn with_conn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.conn.lock().map_err(|_| {
            CoreError::Validation("database lock poisoned".into())
        })?;
        f(&conn)
    }

    pub fn append_event(&self, event_type: &str, payload: &str) -> Result<i64> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO event_log (event_type, payload, created_at) VALUES (?1, ?2, datetime('now'))",
                rusqlite::params![event_type, payload],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }
}
