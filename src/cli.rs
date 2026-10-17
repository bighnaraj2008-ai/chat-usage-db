use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "chat-usage-db", author, version, about = "High-performance SQLite database layer for chat token usage analytics", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "chat_usage.db")]
    pub db_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run database migrations
    Migrate,
    /// Seed the database with sample data
    Seed,
    /// Display usage statistics
    Stats {
        #[arg(short, long)]
        user_id: Option<i64>,
    },
    /// Query token usage breakdown
    Query {
        #[arg(short, long)]
        conversation_id: Option<i64>,
    },
}