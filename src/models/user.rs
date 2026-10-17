use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
    pub status: Option<String>,
}

impl User {
    pub fn new(id: i64, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
            created_at: None,
            status: Some("active".to_string()),
        }
    }

    pub fn validate(&self) -> Result<(), AppError> {
        Self::validate_fields(&self.username, &self.email)
    }

    pub fn validate_fields(username: &str, email: &str) -> Result<(), AppError> {
        if username.trim().is_empty() {
            return Err(AppError::ValidationError("Username cannot be empty".to_string()));
        }
        if !email.contains('@') || !email.contains('.') {
            return Err(AppError::ValidationError(format!("Invalid email address: {}", email)));
        }
        Ok(())
    }
}