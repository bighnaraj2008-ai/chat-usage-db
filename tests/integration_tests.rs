use chat_usage_db::db::Database;
use chat_usage_db::db::repository::UserRepository;
use chat_usage_db::services::UsageTracker;

fn setup_db() -> Database {
    Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB")
}

#[test]
fn test_database_lifecycle_and_seed() {
    let db = setup_db();
    db.run_seed().expect("Failed to run seed script");

    let conn = db.get_conn().unwrap();
    let tracker = UsageTracker::new();

    // Verify seed users exist
    let user_repo = UserRepository::new();
    let alice = user_repo.find_by_id(&conn, 1).unwrap();
    assert_eq!(alice.username, "alice");

    let bob = user_repo.find_by_id(&conn, 2).unwrap();
    assert_eq!(bob.username, "bob");

    // Verify token aggregation works on seed data
    let total = tracker.total_tokens_all(&conn).unwrap();
    assert!(total > 0);
}