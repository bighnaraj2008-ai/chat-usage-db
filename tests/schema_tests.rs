use chat_usage_db::db::Database;
use chat_usage_db::db::repository::{ConversationRepository, MessageRepository, UserRepository};

fn setup_db() -> Database {
    Database::new(std::path::Path::new(":memory:")).expect("Failed to initialize test DB")
}

#[test]
fn test_tables_exist() {
    let db = setup_db();
    let conn = db.get_conn().unwrap();
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert!(tables.contains(&"users".to_string()));
    assert!(tables.contains(&"conversations".to_string()));
    assert!(tables.contains(&"messages".to_string()));
    assert!(tables.contains(&"schema_migrations".to_string()));
}

#[test]
fn test_foreign_key_cascade_delete() {
    let db = setup_db();
    let conn = db.get_conn().unwrap();
    let user_repo = UserRepository::new();
    let conv_repo = ConversationRepository::new();
    let msg_repo = MessageRepository::new();

    let user = user_repo.create(&conn, "cascade_test", "cascade@test.com").unwrap();
    let conv = conv_repo.create(&conn, user.id, "Test Title", "gpt-4o").unwrap();
    let _msg = msg_repo.create(&conn, conv.id, "user", "Test cascade message", 42).unwrap();

    // Verify records exist
    let convs = conv_repo.find_by_user_id(&conn, user.id).unwrap();
    assert_eq!(convs.len(), 1);
    let msgs = msg_repo.find_by_conversation_id(&conn, conv.id).unwrap();
    assert_eq!(msgs.len(), 1);

    // Delete user, check cascade
    conn.execute("DELETE FROM users WHERE id = ?1", rusqlite::params![user.id]).unwrap();

    let convs_after = conv_repo.find_by_user_id(&conn, user.id).unwrap();
    assert_eq!(convs_after.len(), 0);
    let msgs_after = msg_repo.find_by_conversation_id(&conn, conv.id).unwrap();
    assert_eq!(msgs_after.len(), 0);
}