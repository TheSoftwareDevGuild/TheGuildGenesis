FROM rust:latest AS build
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    netcat-openbsd \
 && rm -rf /var/lib/apt/lists/*
RUN useradd -m appuser
WORKDIR /app
COPY --from=build /app/target/release/guild-indexer /app/app
ENV RUST_LOG=info PORT=3002
EXPOSE 3002
USER appuser
CMD ["/app/app"]

