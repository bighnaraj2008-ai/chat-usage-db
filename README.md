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