# Multi-stage build for chat-usage-db
FROM rust:1-bookworm AS builder

WORKDIR /usr/src/chat-usage-db
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/chat-usage-db/target/release/chat-usage-db /usr/local/bin/chat-usage-db

ENTRYPOINT ["chat-usage-db"]
CMD ["stats"]