use std::time::Instant;
use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, MessageRepository, UserRepository};
use chat_usage_db::services::UsageTracker;

fn main() {
    println!("Running database query performance benchmarks...");
    let db = Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB");
    let conn = db.get_conn().unwrap();
    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();
    let msg_repo = MessageRepository::new();
    let tracker = UsageTracker::new();

    let user = user_repo.create(&conn, "bench_user", "bench@test.com").unwrap();
    let conv = conv_repo.create(&conn, user.id, "Benchmark Conversation", "gpt-4o").unwrap();

    let insert_start = Instant::now();
    for i in 0..1000 {
        msg_repo.create(&conn, conv.id, "user", &format!("Message {}", i), 15).unwrap();
    }
    let insert_duration = insert_start.elapsed();
    println!("Inserted 1,000 messages in: {:?}", insert_duration);

    let query_start = Instant::now();
    let total = tracker.total_tokens_for_conversation(&conn, conv.id).unwrap();
    let query_duration = query_start.elapsed();
    println!("Aggregated {} tokens across conversation in: {:?}", total, query_duration);
}