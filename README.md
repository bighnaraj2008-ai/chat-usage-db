# chat-usage-db

A production-grade SQLite database layer in Rust modeling users, conversations, messages, and token consumption.

## Architectural Overview
`chat-usage-db` provides a robust, zero-dependency embedded database abstraction built specifically for tracking Large Language Model (LLM) interaction history and token consumption.
- Fully atomic SQLite transactions with Write-Ahead Logging (WAL) enabled.
- Automatic versioned schema migrations embedded directly into the binary.
- Data access repository layer enforcing relational consistency and foreign key cascades.
- Modular token usage tracking and cost calculation service.