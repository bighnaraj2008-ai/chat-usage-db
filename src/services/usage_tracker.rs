use rusqlite::Connection;
use crate::error::AppError;

pub struct UsageTracker;

impl UsageTracker {
    pub fn new() -> Self {
        UsageTracker
    }

    pub fn total_tokens_for_user(&self, conn: &Connection, user_id: i64) -> Result<i64, AppError> {
        let total: Option<i64> = conn.query_row(
            "SELECT SUM(m.token_count) FROM messages m JOIN conversations c ON m.conversation_id = c.id WHERE c.user_id = ?1",
            rusqlite::params![user_id],
            |row| row.get(0),
        )?;
        Ok(total.unwrap_or(0))
    }

    pub fn total_tokens_all(&self, conn: &Connection) -> Result<i64, AppError> {
        let total: Option<i64> = conn.query_row(
            "SELECT SUM(token_count) FROM messages",
            [],
            |row| row.get(0),
        )?;
        Ok(total.unwrap_or(0))
    }
}