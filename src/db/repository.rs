use rusqlite::Connection;
use crate::error::AppError;
use crate::models::{Conversation, Message, User};

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

pub struct ConversationRepository;

impl ConversationRepository {
    pub fn new() -> Self {
        ConversationRepository
    }

    pub fn create(&self, conn: &Connection, user_id: i64, title: &str, model_name: &str) -> Result<Conversation, AppError> {
        conn.execute(
            "INSERT INTO conversations (user_id, title, model_name) VALUES (?1, ?2, ?3)",
            rusqlite::params![user_id, title, model_name],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Conversation::new(id, user_id, title.to_string(), model_name.to_string()))
    }

    pub fn find_by_id(&self, conn: &Connection, id: i64) -> Result<Conversation, AppError> {
        let mut stmt = conn.prepare("SELECT id, user_id, title, model_name, created_at, updated_at FROM conversations WHERE id = ?1")?;
        stmt.query_row(rusqlite::params![id], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                model_name: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("Conversation with id {} not found", id)),
            _ => AppError::RusqliteError(e),
        })
    }

    pub fn find_by_user_id(&self, conn: &Connection, user_id: i64) -> Result<Vec<Conversation>, AppError> {
        let mut stmt = conn.prepare("SELECT id, user_id, title, model_name, created_at, updated_at FROM conversations WHERE user_id = ?1 ORDER BY id ASC")?;
        let conv_iter = stmt.query_map(rusqlite::params![user_id], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                model_name: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut conversations = Vec::new();
        for conv in conv_iter {
            conversations.push(conv?);
        }
        Ok(conversations)
    }

    pub fn list_all(&self, conn: &Connection) -> Result<Vec<Conversation>, AppError> {
        let mut stmt = conn.prepare("SELECT id, user_id, title, model_name, created_at, updated_at FROM conversations ORDER BY id ASC")?;
        let conv_iter = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                model_name: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut conversations = Vec::new();
        for conv in conv_iter {
            conversations.push(conv?);
        }
        Ok(conversations)
    }
}

pub struct MessageRepository;

impl MessageRepository {
    pub fn new() -> Self {
        MessageRepository
    }

    pub fn create(&self, conn: &Connection, conversation_id: i64, role: &str, content: &str, token_count: i64) -> Result<Message, AppError> {
        conn.execute(
            "INSERT INTO messages (conversation_id, role, content, token_count) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![conversation_id, role, content, token_count],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Message::new(
            id,
            conversation_id,
            role.to_string(),
            content.to_string(),
            token_count,
        ))
    }

    pub fn find_by_conversation_id(&self, conn: &Connection, conversation_id: i64) -> Result<Vec<Message>, AppError> {
        let mut stmt = conn.prepare("SELECT id, conversation_id, role, content, token_count, created_at FROM messages WHERE conversation_id = ?1 ORDER BY id ASC")?;
        let msg_iter = stmt.query_map(rusqlite::params![conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                token_count: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        let mut messages = Vec::new();
        for msg in msg_iter {
            messages.push(msg?);
        }
        Ok(messages)
    }
}