use chat_usage_db::db::Database;
use chat_usage_db::db::repository::UserRepository;

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