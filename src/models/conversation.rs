use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub model_name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Conversation {
    pub fn new(id: i64, user_id: i64, title: String, model_name: String) -> Self {
        Self {
            id,
            user_id,
            title,
            model_name,
            created_at: None,
            updated_at: None,
        }
    }
}