use rusqlite::Connection;
use crate::error::AppError;
use crate::models::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }
}