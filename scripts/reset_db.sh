#!/usr/bin/env bash
set -euo pipefail

echo "Resetting local database files..."
rm -f chat_usage.db chat_usage.db-shm chat_usage.db-wal
cargo run -- migrate
cargo run -- seed
echo "Database successfully reset and seeded."