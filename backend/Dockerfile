FROM rust:latest AS base
WORKDIR /app

# Development stage
FROM base AS dev
RUN cargo install cargo-watch
COPY . .
ENV RUST_LOG=debug RUST_BACKTRACE=1
CMD ["cargo", "watch", "-x", "run --bin guild-backend"]

# Test stage
FROM base AS test
COPY . .
CMD ["cargo", "test", "--", "--test-threads=1"]

# Build stage
FROM base AS build
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim AS production
RUN apt-get update && apt-get install -y --no-install-recommends \
    netcat-openbsd \
 && rm -rf /var/lib/apt/lists/*
RUN useradd -m appuser
WORKDIR /app
COPY --from=build /app/target/release/guild-backend /app/guild-backend
ENV RUST_LOG=info PORT=3001
EXPOSE 3001
USER appuser
CMD ["/app/guild-backend"]

