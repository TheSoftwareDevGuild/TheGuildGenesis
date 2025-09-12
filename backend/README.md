# Guild Backend (Rust + Axum + SQLx)

This service exposes a REST API for managing profiles, backed by PostgreSQL.

- HTTP: 0.0.0.0:3001
- DB: PostgreSQL (SQLx)
- Migrations: SQLx migrator (on startup and via `bin/migrate`)

## 1) Prerequisites
- Rust (latest stable recommended)
- PostgreSQL 14+ (`initdb`, `pg_ctl`, `psql` available)

## 2) Environment
Create `backend/.env`:
```
DATABASE_URL=postgresql://guild_user:guild_password@localhost:5432/guild_genesis
RUST_LOG=guild_backend=debug,tower_http=debug
```
The server requires `DATABASE_URL` at runtime.

## 3) Start local Postgres
From the repo root, using Just:
```
just db-setup
```
This will:
- init `.postgres/` if missing
- start Postgres on localhost:5432
- create DB `guild_genesis` and user `guild_user/guild_password`
- run backend migrations

Manual alternative (repo root):
```
initdb -D .postgres
pg_ctl -D .postgres -l .postgres/postgres.log start

createdb guild_genesis || true
psql -d guild_genesis -c "CREATE USER guild_user WITH PASSWORD 'guild_password';" || true
psql -d guild_genesis -c "GRANT ALL PRIVILEGES ON DATABASE guild_genesis TO guild_user;" || true
```
Stop Postgres:
```
pg_ctl -D .postgres stop
```

## 4) Run migrations (optional)
Migrations run on server startup. To run explicitly:
```
cd backend
cargo run --bin guild-backend
```

## 5) Launch the API
```
cd backend
cargo run
```
The server listens on `http://0.0.0.0:3001`.

## 6) API quickstart
All endpoints require Ethereum header-based auth.

Create profile:
```
curl -X POST \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{
    "name": "My profile",
    "description": "Hello world",
    "avatar_url": "https://example.com/avatar.png"
  }' \
  http://0.0.0.0:3001/profiles/
```
Get profile:
```
curl -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
     -H 'x-eth-signature: 0x00000000000000' \
     -H 'x-siwe-message: LOGIN_NONCE' \
     http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```
Update profile:
```
curl -X PUT \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{ "name": "New name", "description": "New desc" }' \
  http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```

## 7) Troubleshooting
- initdb locale error:
```
LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 initdb --locale=en_US.UTF-8 --encoding=UTF8 -D .postgres
```
- Permission denied on schema `public`:
```
psql -U postgres -d guild_genesis -c "ALTER SCHEMA public OWNER TO guild_user;"
psql -U postgres -d guild_genesis -c "GRANT USAGE, CREATE ON SCHEMA public TO guild_user;"
```
- Rust edition 2024 error: repo pins `base64ct = 1.7.3`. If still present, `rustup update` or `rustup override set nightly` in `backend/`.

## 8) Structure
- `src/main.rs`: boot server, run migrations
- `src/bin/migrate.rs`: standalone migrator
- `src/presentation`: routes, handlers, middlewares
- `src/infrastructure`: Postgres repository, Ethereum verification
- `src/domain`: entities, repository traits, services
- `src/application`: commands and DTOs
- `migrations/`: SQLx migrations
