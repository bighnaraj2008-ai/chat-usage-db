use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, UserRepository};

fn setup_db() -> Database {
    Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB")
}

#[test]
fn test_user_creation_and_duplicates() {
    let db = setup_db();
    let conn = db.get_conn().unwrap();
    let repo = UserRepository::new();

    let user = repo.create(&conn, "john_doe", "john@example.com").unwrap();
    assert_eq!(user.username, "john_doe");
    assert_eq!(user.email, "john@example.com");

    let found = repo.find_by_id(&conn, user.id).unwrap();
    assert_eq!(found.username, "john_doe");

    // Duplicate username should fail
    let dup_user = repo.create(&conn, "john_doe", "another@example.com");
    assert!(dup_user.is_err());

    // Duplicate email should fail
    let dup_email = repo.create(&conn, "jane_doe", "john@example.com");
    assert!(dup_email.is_err());
}

#[test]
fn test_conversation_foreign_key_binding() {
    let db = setup_db();
    let conn = db.get_conn().unwrap();
    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();

    let user = user_repo.create(&conn, "conv_user", "conv_user@example.com").unwrap();
    let conv = conv_repo.create(&conn, user.id, "My Chat", "claude-3-5-sonnet").unwrap();
    assert_eq!(conv.user_id, user.id);
    assert_eq!(conv.title, "My Chat");

    let user_convs = conv_repo.find_by_user_id(&conn, user.id).unwrap();
    assert_eq!(user_convs.len(), 1);
    assert_eq!(user_convs[0].title, "My Chat");

    // Creating conversation for non-existent user should fail due to foreign key
    let invalid_conv = conv_repo.create(&conn, 99999, "Invalid", "gpt-4o");
    assert!(invalid_conv.is_err());
}