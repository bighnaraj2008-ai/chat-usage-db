use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use rusqlite::Connection;
use crate::error::AppError;

const MIGRATIONS: [(&str, &str); 1] = [("001_initial_schema", include_str!("../../migrations/001_initial_schema.sql"))];
const SEED_DATA: &str = include_str!("../../migrations/002_seed_data.sql");

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;

        let database = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        database.run_migrations()?;
        Ok(database)
    }

    pub fn open(path: &Path) -> Result<Self, AppError> {
        Self::new(path)
    }

    pub fn run_migrations(&self) -> Result<(), AppError> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|error| AppError::MigrationError(format!("failed to lock database: {error}")))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY NOT NULL,
                applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )?;

        let transaction = conn.transaction()?;
        for (version, sql) in MIGRATIONS {
            let applied = transaction.query_row(
                "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?1)",
                [version],
                |row| row.get::<_, bool>(0),
            )?;

            if !applied {
                transaction.execute_batch(sql)?;
                transaction.execute("INSERT INTO schema_migrations (version) VALUES (?1)", [version])?;
            }
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn run_seed(&self) -> Result<(), AppError> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|error| AppError::MigrationError(format!("failed to lock database: {error}")))?;
        let transaction = conn.transaction()?;
        transaction.execute_batch(SEED_DATA)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn get_conn(&self) -> Result<MutexGuard<'_, Connection>, AppError> {
        self.conn
            .lock()
            .map_err(|error| AppError::GenericError(format!("failed to lock database connection: {error}")))
    }
}