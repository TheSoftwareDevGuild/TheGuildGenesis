# Guild Backend (Rust + Axum + SQLx)

This service exposes a REST API for managing profiles and projects, backed by PostgreSQL.

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

## 5) SQLx Offline Mode & Query Validation

### Overview
SQLx uses compile-time verification of SQL queries. This requires either:
- A live database connection during compilation, OR
- Pre-generated `.sqlx/*.json` metadata files for offline compilation

### Generate SQLx Metadata Files
After adding new database queries or modifying the schema:

```bash
cd backend

# Ensure database is running and migrations are applied
sqlx migrate run

# Generate .sqlx metadata for offline compilation
cargo sqlx prepare -- --bin guild-backend

# Or if you have multiple targets:
cargo sqlx prepare -- --all-targets
```

This creates `.sqlx/*.json` files containing validated query metadata.

### Building Without Database Access
Once `.sqlx` files are generated, you can build without a database connection:

```bash
SQLX_OFFLINE=true cargo build
```

### When to Regenerate `.sqlx` Files
Regenerate whenever you:
- ✅ Add new migrations
- ✅ Add new SQL queries
- ✅ Modify existing queries
- ✅ Change database schema

### Regeneration Workflow
```bash
cd backend

# 1. Apply any new migrations
sqlx migrate run

# 2. Regenerate metadata
cargo sqlx prepare -- --bin guild-backend

# 3. Commit the updated files
git add .sqlx/
git commit -m "Update SQLx offline data"
```

### Troubleshooting SQLx Issues

#### "Query data not found for query"
**Cause:** `.sqlx` files don't contain metadata for your query.

**Solution:**
```bash
cargo sqlx prepare -- --bin guild-backend
```

#### "Password authentication failed"
**Cause:** Wrong `DATABASE_URL` in `.env`.

**Solution:** Update `.env` with correct port and credentials:
```bash
# Check which port PostgreSQL is running on
sudo ss -tlnp | grep postgres

# Update .env
DATABASE_URL=postgresql://guild_user:guild_password@localhost:5432/guild_genesis
```

#### "Connection refused"
**Cause:** PostgreSQL isn't running.

**Solution:**
```bash
# Start PostgreSQL
sudo systemctl start postgresql

# Or with Homebrew
brew services start postgresql

# Or with Docker
docker start <postgres-container-name>
```

### CI/CD Integration
For CI/CD pipelines where database isn't available:

```yaml
# .github/workflows/rust.yml
env:
  SQLX_OFFLINE: true

steps:
  - name: Build
    run: cargo build --release
```

**Note:** Make sure `.sqlx/` files are committed to your repository!

## 6) Launch the API
```
cd backend
cargo run
```
The server listens on `http://0.0.0.0:3001`.

## 7) API Documentation

### Authentication
All protected endpoints require Ethereum signature-based authentication with these headers:
- `x-eth-address`: Your Ethereum wallet address
- `x-eth-signature`: Signature of the payload
- `x-siwe-message`: Login nonce

### Profile Endpoints

#### Create Profile (Protected)
```bash
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

#### Get Profile (Public)
```bash
curl http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```

#### Update Profile (Protected)
```bash
curl -X PUT \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{ "name": "New name", "description": "New desc" }' \
  http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```

#### GitHub Handle Support
Profiles can include an optional GitHub username stored as `github_login`:
- Stored with original casing, uniqueness enforced case-insensitively
- Must match pattern `^[a-zA-Z0-9-]{1,39}$`
- Returns **400 Bad Request** for invalid format
- Returns **409 Conflict** if already taken

```bash
curl -X PUT \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{ "github_login": "MyUser123" }' \
  http://0.0.0.0:3001/profiles/0x2581aAa94299787a8A588B2Fceb161A302939E28
```

### Project Endpoints

#### List All Projects (Public)
```bash
# Get all projects
curl http://0.0.0.0:3001/projects

# Filter by status
curl http://0.0.0.0:3001/projects?status=ongoing

# Filter by creator
curl http://0.0.0.0:3001/projects?creator=0x2581aAa94299787a8A588B2Fceb161A302939E28

# Pagination
curl http://0.0.0.0:3001/projects?limit=10&offset=0
```

**Query Parameters:**
- `status` - Filter by status (proposal, ongoing, rejected)
- `creator` - Filter by creator address
- `limit` - Max results (default: all, max: 100)
- `offset` - Skip N results

#### Get Project by ID (Public)
```bash
curl http://0.0.0.0:3001/projects/123e4567-e89b-12d3-a456-426614174000
```

#### Get User's Projects (Public)
```bash
curl http://0.0.0.0:3001/users/0x2581aAa94299787a8A588B2Fceb161A302939E28/projects
```

#### Create Project (Protected)
```bash
curl -X POST \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{
    "name": "Guild Treasury Management",
    "description": "A system for managing guild funds",
    "status": "proposal"
  }' \
  http://0.0.0.0:3001/projects
```

**Valid statuses:** `proposal`, `ongoing`, `rejected`

#### Update Project (Protected, Creator Only)
```bash
curl -X PATCH \
  -H 'Content-Type: application/json' \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  -d '{
    "status": "ongoing",
    "description": "Updated description"
  }' \
  http://0.0.0.0:3001/projects/123e4567-e89b-12d3-a456-426614174000
```

**Note:** Only the project creator can update projects.

#### Delete Project (Protected, Creator Only)
```bash
curl -X DELETE \
  -H 'x-eth-address: 0x2581aAa94299787a8A588B2Fceb161A302939E28' \
  -H 'x-eth-signature: 0x00000000000000' \
  -H 'x-siwe-message: LOGIN_NONCE' \
  http://0.0.0.0:3001/projects/123e4567-e89b-12d3-a456-426614174000
```

**Note:** Only the project creator can delete projects.

### API Response Codes
- **200 OK** - Successful GET/PUT/PATCH
- **201 Created** - Resource created
- **204 No Content** - Successful DELETE
- **400 Bad Request** - Invalid input
- **401 Unauthorized** - Missing/invalid authentication
- **403 Forbidden** - Not authorized (e.g., not project creator)
- **404 Not Found** - Resource doesn't exist
- **409 Conflict** - Duplicate resource (e.g., GitHub handle taken)

## 8) Testing

### Automated Test Scripts
Test scripts are available in the `scripts/` directory:

```bash
# Test authentication and profile endpoints
./scripts/test_auth_login.sh

# Test project endpoints (requires keys.json)
./scripts/test_projects_api.sh
```

**Note:** Test scripts require `keys.json` in the project root:
```json
{
  "publicKey": "0x...",
  "privateKey": "0x..."
}
```

### Running Unit Tests
```bash
cd backend
cargo test
```

### Integration Tests (Test Mode)
Integration tests run under `TEST_MODE=1`, which uses a test-only auth layer:
```bash
TEST_MODE=1 cargo test
```

## 9) Deployment

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

## 10) Troubleshooting

### Database Issues
- **Port already in use**: Check what's running on port 5432: `lsof -i :5432`
- **Port mismatch**: Ensure `.env` uses correct port (5432, not 5433)
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
  - For production: Run `cargo sqlx prepare` to generate metadata
- **Migration conflicts**: Use `SKIP_MIGRATIONS=1` to disable automatic migrations
- **Rust edition 2024 error**: Repo pins `base64ct = 1.7.3`. If still present, `rustup update` or `rustup override set nightly` in `backend/`.

## 11) Structure
- `src/main.rs`: boot server (automatic migrations in production, manual in dev)
- `src/bin/migrate.rs`: standalone migrator
- `src/presentation`: routes, handlers, middlewares
- `src/infrastructure`: Postgres repositories, Ethereum verification
- `src/domain`: entities, repository traits, services, value objects
- `src/application`: commands, queries, and DTOs
- `migrations/`: SQLx migration files
- `.sqlx/`: SQLx offline query metadata (committed to repo)

## 12) GitHub Issue Ingestion

The backend can sync GitHub issues into the database via an admin-protected endpoint.

### Required Environment Variables

Add these to `backend/.env`:

```
GITHUB_TOKEN=ghp_your_personal_access_token
GITHUB_OWNER=TheSoftwareDevGuild
GITHUB_API_URL=https://api.github.com
```

| Variable | Required | Description |
|---|---|---|
| `GITHUB_TOKEN` | Yes | GitHub personal access token (PAT) with `repo` scope |
| `GITHUB_OWNER` | Yes | GitHub organization or user that owns the repos |
| `GITHUB_API_URL` | No | API base URL (defaults to `https://api.github.com`) |

### Trigger Sync (Admin)

The sync endpoint is protected by admin authentication. You need a wallet address listed in the `ADMIN_ADDRESSES` environment variable.

```bash
# Sync issues for one or more repositories
curl -X POST http://localhost:3001/admin/github/sync \
  -H "Content-Type: application/json" \
  -H "x-eth-address: <YOUR_ADMIN_ADDRESS>" \
  -d '{
    "repos": ["TheGuildGenesis"],
    "since": "2025-01-01T00:00:00Z"
  }'
```

**Request body**:
- `repos` (required): List of repository names under `GITHUB_OWNER` to sync
- `since` (optional): ISO 8601 timestamp — only sync issues updated after this date

**Response**:
```json
{
  "synced": 42,
  "repos": ["TheGuildGenesis"]
}
```

### How It Works
- Fetches issues via `{GITHUB_API_URL}/repos/{GITHUB_OWNER}/{repo}/issues`
- Ignores pull requests (GitHub returns PRs in the issues endpoint)
- Derives `points` from labels matching the pattern `Npts` (e.g. `3pts`, `10pts`, case-insensitive)
- Normalizes all labels to lowercase
- Upserts using composite key `(repo_id, github_issue_id)` for idempotency
- Preserves `rewarded_sepolia` and `distribution_id` across re-syncs

### Fetch Synced Issues (Public)

After syncing, query the stored issues to verify:

```bash
# List all synced issues for a repo
curl http://localhost:3001/github/issues?repo=TheGuildGenesis

# Filter by state
curl "http://localhost:3001/github/issues?repo=TheGuildGenesis&state=closed"
```

**Response** (array of `GithubIssue`):
```json
[
  {
    "repo_id": 123456,
    "github_issue_id": 789,
    "repo": "TheGuildGenesis",
    "issue_number": 42,
    "title": "Implement feature X",
    "state": "open",
    "labels": ["bug", "3pts"],
    "points": 3,
    "assignee_logins": ["alice"],
    "url": "https://github.com/TheSoftwareDevGuild/TheGuildGenesis/issues/42",
    "created_at": "2025-01-15T10:00:00Z",
    "closed_at": null,
    "rewarded_sepolia": false,
    "distribution_id": null,
    "updated_at": "2025-01-20T12:00:00Z"
  }
]
```
