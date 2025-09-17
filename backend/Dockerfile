# syntax=docker/dockerfile:1

FROM rust:1.80-bullseye AS builder
WORKDIR /app

# Cache dependencies
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY backend/migrations ./migrations
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/guild-backend /usr/local/bin/guild-backend
ENV RUST_LOG=info
EXPOSE 3001
CMD ["/usr/local/bin/guild-backend"]


