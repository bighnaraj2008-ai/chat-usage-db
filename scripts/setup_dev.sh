#!/usr/bin/env bash
set -euo pipefail

echo "Setting up development environment for chat-usage-db..."
rustc --version
cargo --version

rustup component add clippy rustfmt || true
echo "Development dependencies installed successfully."