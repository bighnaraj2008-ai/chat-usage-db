-- Test DB Fixture
INSERT INTO users (username, email) VALUES ('fixture_alice', 'fixture_alice@example.com');
INSERT INTO users (username, email) VALUES ('fixture_bob', 'fixture_bob@example.com');
INSERT INTO conversations (user_id, title, model_name) VALUES (1, 'Test Session', 'gpt-4o');
INSERT INTO messages (conversation_id, role, content, token_count) VALUES (1, 'user', 'Ping fixture', 12);
INSERT INTO messages (conversation_id, role, content, token_count) VALUES (1, 'assistant', 'Pong fixture', 18);