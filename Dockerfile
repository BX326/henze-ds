# Build stage
FROM rust:1.77-bookworm as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml rust-toolchain ./
COPY henze-ds ./henze-ds
COPY henze-ds-cli ./henze-ds-cli
COPY henze-ds-discord ./henze-ds-discord
COPY henze-ds-web ./henze-ds-web
COPY schema-flex ./schema-flex

# Build release binary
RUN cargo build --release --package henze-ds-web

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/henze-ds-web .

# Copy static files and templates
COPY henze-ds-web/static ./static
COPY henze-ds-web/templates ./templates
COPY henze-ds-web/Rocket.toml ./Rocket.toml

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_TEMPLATE_DIR=/app/templates
ENV ROCKET_STATIC_DIR=/app/static
ENV CACHE_DB_PATH=/data/events_cache.sqlite

EXPOSE 8080

CMD ["./henze-ds-web"]
