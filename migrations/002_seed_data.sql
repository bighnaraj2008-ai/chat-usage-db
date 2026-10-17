-- Seed Data

-- Insert Users
INSERT INTO users (username, email) VALUES ('alice', 'alice@example.com');
INSERT INTO users (username, email) VALUES ('bob', 'bob@example.com');

-- Insert Conversations
INSERT INTO conversations (user_id, title, model_name) VALUES (1, 'Rust Help', 'gpt-4o');
INSERT INTO conversations (user_id, title, model_name) VALUES (2, 'SQL Advice', 'claude-3-5-sonnet');