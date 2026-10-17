use serde::{Deserialize, Serialize};

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
}