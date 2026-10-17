use std::time::Instant;
use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, UserRepository};

fn main() {
    println!("Running database query performance benchmarks...");
    let db = Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB");
    let conn = db.get_conn().unwrap();
    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();

    let start = Instant::now();
    let user = user_repo.create(&conn, "bench_user", "bench@test.com").unwrap();
    let _conv = conv_repo.create(&conn, user.id, "Benchmark Conversation", "gpt-4o").unwrap();
    println!("Setup completed in: {:?}", start.elapsed());
}