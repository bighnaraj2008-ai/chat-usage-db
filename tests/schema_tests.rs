use chat_usage_db::db::Database;

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