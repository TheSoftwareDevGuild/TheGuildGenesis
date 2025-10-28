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

## 3) Database Setup

### Option A: Use existing PostgreSQL (Recommended)
If you have PostgreSQL running locally (via Homebrew, Docker, etc.):
```bash
# Create database and user
psql -h localhost -p 5432 -U $(whoami) -c "CREATE DATABASE guild_genesis;" || true
psql -h localhost -p 5432 -U $(whoami) -c "CREATE USER guild_user WITH PASSWORD 'guild_password';" || true
psql -h localhost -p 5432 -U $(whoami) -c "GRANT ALL PRIVILEGES ON DATABASE guild_genesis TO guild_user;" || true

# Grant schema permissions
psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -c "GRANT ALL PRIVILEGES ON SCHEMA public TO guild_user;"
psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO guild_user;"
```

### Option B: Start local PostgreSQL instance
From the repo root:
```bash
initdb -D .postgres
pg_ctl -D .postgres -l .postgres/postgres.log start

createdb guild_genesis || true
psql -d guild_genesis -c "CREATE USER guild_user WITH PASSWORD 'guild_password';" || true
psql -d guild_genesis -c "GRANT ALL PRIVILEGES ON DATABASE guild_genesis TO guild_user;" || true
```

Stop local Postgres:
```bash
pg_ctl -D .postgres stop
```

## 4) Database Migrations

### Development (Manual Migrations)
For local development, run migrations manually to avoid SQLx compile-time validation issues:
```bash
cd backend
psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -f migrations/001_initial_schema.sql
psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -f migrations/002_add_github_login.sql
psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -f migrations/003_add_nonces.sql

# Then start server with migrations disabled
SKIP_MIGRATIONS=1 cargo run --bin guild-backend
```

### Production (Automatic Migrations)
In production (Heroku, etc.), migrations run automatically on server startup. No additional setup needed.

### Disable SQLx Compile-time Validation (Development Only)
To avoid SQLx compile-time query validation issues during development:
```bash
export SQLX_OFFLINE=true
```
This allows compilation without a database connection, but you lose compile-time query validation.

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
  http://0.0.0.0:3001/profiles
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

### GitHub handle support

Profiles can now include an optional GitHub username stored as `github_login`.

- The value is stored with its original casing, but uniqueness is enforced case-insensitively ("Alice" conflicts with "alice").
- `github_login` must match the pattern `^[a-zA-Z0-9-]{1,39}$`; otherwise the API returns **400 Bad Request**.
- When the normalized value is already claimed by another profile, the API returns **409 Conflict**.
- Successful creates return **201 Created** and updates return **200 OK**.
- Include the field when creating or updating a profile:

```
curl -X PUT \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{ "github_login": "MyUser123" }' \
  http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```

Integration and automated tests run under `TEST_MODE=1`, which swaps in a test-only auth layer so GitHub handle flows can be exercised without Ethereum signature verification.

## 7) Deployment

### Heroku
1. Set environment variables:
   ```bash
   heroku config:set DATABASE_URL=postgresql://...
   heroku config:set RUST_LOG=guild_backend=info
   ```

2. Deploy:
   ```bash
   git push heroku main
   ```

Migrations run automatically on deployment. No additional setup needed.

### Docker
```bash
docker build -t guild-backend .
docker run -e DATABASE_URL=postgresql://... guild-backend
```

## 8) Troubleshooting

### Database Issues
- **Port already in use**: Check what's running on port 5432: `lsof -i :5432`
- **Permission denied**: Ensure `guild_user` has proper permissions:
  ```bash
  psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -c "GRANT ALL PRIVILEGES ON SCHEMA public TO guild_user;"
  psql -h localhost -p 5432 -U $(whoami) -d guild_genesis -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO guild_user;"
  ```
- **initdb locale error**:
  ```bash
  LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 initdb --locale=en_US.UTF-8 --encoding=UTF8 -D .postgres
  ```

### Server Issues
- **Port 3001 already in use**: Use a different port: `PORT=3002 cargo run --bin guild-backend`
- **SQLx compile errors**: 
  - For development: Set `SQLX_OFFLINE=true` and run migrations manually
  - For production: Ensure database is accessible during compilation
- **Migration conflicts**: Use `SKIP_MIGRATIONS=1` to disable automatic migrations
- **Rust edition 2024 error**: Repo pins `base64ct = 1.7.3`. If still present, `rustup update` or `rustup override set nightly` in `backend/`.

## 9) Structure
- `src/main.rs`: boot server (automatic migrations in production, manual in dev)
- `src/bin/migrate.rs`: standalone migrator
- `src/presentation`: routes, handlers, middlewares
- `src/infrastructure`: Postgres repository, Ethereum verification
- `src/domain`: entities, repository traits, services
- `src/application`: commands and DTOs
- `migrations/`: SQLx migration files
