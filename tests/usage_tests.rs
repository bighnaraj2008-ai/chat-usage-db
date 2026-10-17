use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, MessageRepository, UserRepository};
use chat_usage_db::services::UsageTracker;

fn setup_db() -> Database {
    Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB")
}

#[test]
fn test_token_aggregation_accuracy() {
    let db = setup_db();
    let conn = db.get_conn().unwrap();
    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();
    let msg_repo = MessageRepository::new();
    let tracker = UsageTracker::new();

    let u1 = user_repo.create(&conn, "u1", "u1@test.com").unwrap();
    let u2 = user_repo.create(&conn, "u2", "u2@test.com").unwrap();

    let c1 = conv_repo.create(&conn, u1.id, "Chat 1", "gpt-4o").unwrap();
    let c2 = conv_repo.create(&conn, u1.id, "Chat 2", "gpt-4o").unwrap();
    let c3 = conv_repo.create(&conn, u2.id, "Chat 3", "claude-3-5-sonnet").unwrap();

    msg_repo.create(&conn, c1.id, "user", "Hello", 100).unwrap();
    msg_repo.create(&conn, c1.id, "assistant", "Hi", 200).unwrap();
    msg_repo.create(&conn, c2.id, "user", "More query", 150).unwrap();
    msg_repo.create(&conn, c3.id, "user", "Other user", 50).unwrap();

    // User 1 tokens: 100 + 200 + 150 = 450
    let u1_tokens = tracker.total_tokens_for_user(&conn, u1.id).unwrap();
    assert_eq!(u1_tokens, 450);

    // User 2 tokens: 50
    let u2_tokens = tracker.total_tokens_for_user(&conn, u2.id).unwrap();
    assert_eq!(u2_tokens, 50);

    // Total tokens all: 500
    let all_tokens = tracker.total_tokens_all(&conn).unwrap();
    assert_eq!(all_tokens, 500);

    // Conversation 1 tokens: 300
    let c1_tokens = tracker.total_tokens_for_conversation(&conn, c1.id).unwrap();
    assert_eq!(c1_tokens, 300);

    // Cost calculation: 450 tokens @ $0.005 / 1k = $0.00225
    let cost = tracker.user_token_cost(&conn, u1.id, 0.005).unwrap();
    assert!((cost - 0.00225).abs() < 1e-6);
}