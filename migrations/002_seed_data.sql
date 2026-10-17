-- Seed Data

-- Insert Users
INSERT INTO users (username, email) VALUES ('alice', 'alice@example.com');
INSERT INTO users (username, email) VALUES ('bob', 'bob@example.com');

-- Insert Conversations
INSERT INTO conversations (user_id, title, model_name) VALUES (1, 'Rust Help', 'gpt-4o');
INSERT INTO conversations (user_id, title, model_name) VALUES (2, 'SQL Advice', 'claude-3-5-sonnet');

-- Insert Messages
INSERT INTO messages (conversation_id, role, content, token_count) VALUES (1, 'user', 'How do I use lifetimes?', 50);
INSERT INTO messages (conversation_id, role, content, token_count) VALUES (1, 'assistant', 'Lifetimes...', 150);
INSERT INTO messages (conversation_id, role, content, token_count) VALUES (2, 'user', 'Explain foreign keys.', 80);