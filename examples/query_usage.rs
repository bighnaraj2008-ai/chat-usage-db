use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, MessageRepository, UserRepository};
use chat_usage_db::services::UsageTracker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chat Usage DB - Example Usage");
    let db = Database::new(std::path::Path::new(":memory:"))?;
    let conn = db.get_conn()?;

    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();
    let msg_repo = MessageRepository::new();
    let tracker = UsageTracker::new();

    let alice = user_repo.create(&conn, "alice", "alice@example.com")?;
    let conv = conv_repo.create(&conn, alice.id, "Coding Assistant", "gpt-4o")?;

    msg_repo.create(&conn, conv.id, "user", "How do I implement SQLite in Rust?", 25)?;
    msg_repo.create(&conn, conv.id, "assistant", "Use rusqlite with bundled SQLite.", 85)?;

    let total = tracker.total_tokens_for_user(&conn, alice.id)?;
    println!("Total tokens for {}: {}", alice.username, total);

    let cost = tracker.user_token_cost(&conn, alice.id, 0.005)?;
    println!("Estimated cost: ${:.4}", cost);

    Ok(())
}