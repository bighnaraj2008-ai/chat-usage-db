use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::error::AppError;
use crate::db::Database;
use crate::services::UsageTracker;

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

pub fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let db = Database::new(&cli.db_path)?;

    match cli.command {
        Commands::Migrate => {
            println!("Migrations applied successfully.");
        }
        Commands::Seed => {
            db.run_seed()?;
            println!("Database seeded successfully.");
        }
        Commands::Stats { user_id } => {
            let conn = db.get_conn()?;
            let usage_tracker = UsageTracker::new();

            match user_id {
                Some(id) => {
                    let total = usage_tracker.total_tokens_for_user(&conn, id)?;
                    println!("Total tokens for user {}: {}", id, total);
                }
                None => {
                    let total = usage_tracker.total_tokens_all(&conn)?;
                    println!("Total tokens for all users: {}", total);
                }
            }
        }
        Commands::Query { conversation_id } => {
            let conn = db.get_conn()?;
            let usage_tracker = UsageTracker::new();

            match conversation_id {
                Some(cid) => {
                    let total = usage_tracker.total_tokens_for_conversation(&conn, cid)?;
                    println!("Total tokens for conversation {}: {}", cid, total);
                }
                None => {
                    let total = usage_tracker.total_tokens_all(&conn)?;
                    println!("Total tokens across all conversations: {}", total);
                }
            }
        }
    }
    Ok(())
}