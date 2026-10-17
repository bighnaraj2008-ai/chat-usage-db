# chat-usage-db

[![CI](https://github.com/bighnaraj2008-ai/chat-usage-db/actions/workflows/ci.yml/badge.svg)](https://github.com/bighnaraj2008-ai/chat-usage-db/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.70+](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![SQLite: Bundled](https://img.shields.io/badge/sqlite-bundled-blue.svg)](https://www.sqlite.org)

A production-grade SQLite database layer in Rust modeling users, conversations, messages, and token consumption.

## Architectural Overview
`chat-usage-db` provides a robust, zero-dependency embedded database abstraction built specifically for tracking Large Language Model (LLM) interaction history and token consumption.
- Fully atomic SQLite transactions with Write-Ahead Logging (WAL) enabled.
- Automatic versioned schema migrations embedded directly into the binary.
- Data access repository layer enforcing relational consistency and foreign key cascades.
- Modular token usage tracking and cost calculation service.

## Installation & Build

```bash
# Clone the repository
git clone https://github.com/bighnaraj2008-ai/chat-usage-db.git
cd chat-usage-db

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release

# Run test suite (using in-memory hermetic SQLite)
cargo test
```

## Relational Schema

### `users`
| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | Unique user identifier |
| `username` | TEXT | UNIQUE NOT NULL | Account handle |
| `email` | TEXT | UNIQUE NOT NULL | Verified contact email |
| `created_at` | DATETIME | DEFAULT CURRENT_TIMESTAMP | Registration timestamp |
| `status` | TEXT | DEFAULT 'active' | Account lifecycle state |

### `conversations`
| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | Unique thread ID |
| `user_id` | INTEGER | NOT NULL, FK -> users(id) ON DELETE CASCADE | Thread owner |
| `title` | TEXT | NOT NULL | Thread title |
| `model_name` | TEXT | NOT NULL | Target AI model identifier |
| `created_at` | DATETIME | DEFAULT CURRENT_TIMESTAMP | Session initialization |
| `updated_at` | DATETIME | DEFAULT CURRENT_TIMESTAMP | Last activity |

### `messages`
| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | INTEGER | PRIMARY KEY AUTOINCREMENT | Unique message identifier |
| `conversation_id` | INTEGER | NOT NULL, FK -> conversations(id) ON DELETE CASCADE | Parent conversation |
| `role` | TEXT | CHECK(role IN ('system','user','assistant')) NOT NULL | Message sender role |
| `content` | TEXT | NOT NULL | Prompt or response text |
| `token_count` | INTEGER | CHECK(token_count >= 0) NOT NULL | Billed token count |
| `created_at` | DATETIME | DEFAULT CURRENT_TIMESTAMP | Message timestamp |

### Indexes
- `idx_conversations_user_id` on `conversations(user_id)`
- `idx_messages_conversation_id` on `messages(conversation_id)`
- `idx_users_email` on `users(email)`

## Sample Analytical Queries

```sql
-- Total tokens consumed per user
SELECT u.username, SUM(m.token_count) AS total_tokens
FROM users u
JOIN conversations c ON c.user_id = u.id
JOIN messages m ON m.conversation_id = c.id
GROUP BY u.id
ORDER BY total_tokens DESC;

-- All messages in a conversation ordered chronologically
SELECT role, content, token_count, created_at
FROM messages
WHERE conversation_id = 1
ORDER BY created_at ASC;

-- Conversation statistics with message counts
SELECT c.id, c.title, c.model_name, COUNT(m.id) AS msg_count
FROM conversations c
LEFT JOIN messages m ON m.conversation_id = c.id
GROUP BY c.id;

-- Users with highest active thread counts
SELECT u.username, COUNT(c.id) AS conv_count
FROM users u
LEFT JOIN conversations c ON c.user_id = u.id
GROUP BY u.id
ORDER BY conv_count DESC;
```

## Docker Containerization

You can package and execute `chat-usage-db` in a lightweight Docker container:

```bash
# Build Docker container image
docker build -t chat-usage-db:latest .

# Run containerized CLI commands
docker run --rm -v $(pwd)/data:/data chat-usage-db --db-path /data/chat.db stats
```

## Testing & Verification

```bash
# Execute comprehensive integration tests
cargo test

# Run query performance benchmark harness
cargo run --bin chat-usage-db -- stats
```