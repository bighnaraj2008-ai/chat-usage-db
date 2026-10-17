# chat-usage-db

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