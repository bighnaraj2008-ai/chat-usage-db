use rusqlite::Connection;
use crate::error::AppError;
use crate::models::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    pub fn create(&self, conn: &Connection, username: &str, email: &str) -> Result<User, AppError> {
        User::validate_fields(username, email)?;
        conn.execute(
            "INSERT INTO users (username, email) VALUES (?1, ?2)",
            rusqlite::params![username, email],
        )?;
        let id = conn.last_insert_rowid();
        Ok(User::new(id, username.to_string(), email.to_string()))
    }

    pub fn find_by_id(&self, conn: &Connection, id: i64) -> Result<User, AppError> {
        let mut stmt = conn.prepare("SELECT id, username, email, created_at, status FROM users WHERE id = ?1")?;
        stmt.query_row(rusqlite::params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
                status: row.get(4)?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("User with id {} not found", id)),
            _ => AppError::RusqliteError(e),
        })
    }

    pub fn find_by_email(&self, conn: &Connection, email: &str) -> Result<User, AppError> {
        let mut stmt = conn.prepare("SELECT id, username, email, created_at, status FROM users WHERE email = ?1")?;
        stmt.query_row(rusqlite::params![email], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
                status: row.get(4)?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("User with email {} not found", email)),
            _ => AppError::RusqliteError(e),
        })
    }

    pub fn list_all(&self, conn: &Connection) -> Result<Vec<User>, AppError> {
        let mut stmt = conn.prepare("SELECT id, username, email, created_at, status FROM users ORDER BY id ASC")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
                status: row.get(4)?,
            })
        })?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        Ok(users)
    }
}