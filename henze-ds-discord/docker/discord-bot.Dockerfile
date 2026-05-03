# Build stage
FROM rust:1.77-bookworm AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock rust-toolchain ./
COPY henze-ds ./henze-ds
COPY henze-ds-cli ./henze-ds-cli
COPY henze-ds-discord ./henze-ds-discord
COPY henze-ds-web ./henze-ds-web
COPY schema-flex ./schema-flex

# Build release binary for the Discord bot
RUN cargo build --release --package henze-ds-discord

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/henze-ds-discord ./henze-ds-discord

# Run as non-root for better isolation
RUN useradd --system --create-home --uid 10001 botuser && \
    mkdir -p /data && chown -R botuser:botuser /app /data

USER botuser

ENV RUST_LOG=henze_ds_discord=info
ENV DATABASE_PATH=/data/henze_bets.db
ENV SETTLEMENT_INTERVAL_SECS=300
ENV CRON_SCHEDULE="0 0 8 * * *"

CMD ["./henze-ds-discord"]
