# The Guild Genesis - Development Commands

# Start both frontend and backend in development mode
dev:
    @echo "🚀 Starting The Guild Genesis development servers..."
    @echo "Starting frontend and backend in parallel..."
    @just dev-frontend &
    @just dev-backend &
    @wait

# Start frontend development server
dev-frontend:
    @echo "🎨 Starting Astro frontend..."
    cd frontend && npm run dev

# Start backend development server
dev-backend:
    @echo "🦀 Starting Rust backend..."
    cd backend && cargo run --bin guild-backend-dev

# Install all dependencies
install-all:
    @echo "📦 Installing dependencies..."
    cd frontend && npm install
    cd backend && cargo build

# Database commands using Nix PostgreSQL
db-start:
    @echo "🗄️ Starting PostgreSQL database..."
    @if [ ! -d ".postgres" ]; then \
        echo "Initializing PostgreSQL database..."; \
        initdb -D .postgres; \
    fi
    @echo "Starting PostgreSQL server..."
    pg_ctl -D .postgres -l .postgres/postgres.log start
    @echo "✅ PostgreSQL started on localhost:5432"

db-stop:
    @echo "🛑 Stopping PostgreSQL database..."
    pg_ctl -D .postgres stop
    @echo "✅ PostgreSQL stopped"

db-setup:
    @echo "🗄️ Setting up database..."
    @just db-start
    @echo "Creating database and user..."
    createdb guild_genesis || true
    psql -d guild_genesis -c "CREATE USER guild_user WITH PASSWORD 'guild_password';" || true
    psql -d guild_genesis -c "GRANT ALL PRIVILEGES ON DATABASE guild_genesis TO guild_user;" || true
    @echo "Running migrations..."
    cd backend && cargo run --bin migrate
    @echo "✅ Database setup complete"

db-reset:
    @echo "🔄 Resetting database..."
    @just db-stop
    @just db-setup

# Build commands
build:
    @echo "🔨 Building all services..."
    cd frontend && npm run build
    cd backend && cargo build --release

build-frontend:
    @echo "🎨 Building frontend..."
    cd frontend && npm run build

build-backend:
    @echo "🦀 Building backend..."
    cd backend && cargo build --release

# Testing
test:
    @echo "🧪 Running tests..."
    cd frontend && npm run test:run
    cd backend && cargo test --test simple_tests

test-frontend:
    @echo "🧪 Running frontend tests..."
    cd frontend && npm run test:run

test-backend:
    @echo "🧪 Running backend tests..."
    cd backend && cargo test --test simple_tests

# Linting and formatting
lint:
    @echo "🔍 Linting code..."
    cd frontend && npm run lint
    cd backend && cargo clippy

format:
    @echo "✨ Formatting code..."
    cd frontend && npm run format
    cd backend && cargo fmt

# Clean up
clean:
    @echo "🧹 Cleaning up..."
    cd frontend && rm -rf dist node_modules/.cache
    cd backend && cargo clean
    @just db-stop
    rm -rf .postgres

# Help
help:
    @echo "The Guild Genesis - Available Commands:"
    @echo ""
    @echo "Development:"
    @echo "  dev              Start both frontend and backend"
    @echo "  dev-frontend     Start frontend only"
    @echo "  dev-backend      Start backend only"
    @echo "  install-all      Install all dependencies"
    @echo ""
    @echo "Database:"
    @echo "  db-start         Start PostgreSQL database"
    @echo "  db-stop          Stop PostgreSQL database"
    @echo "  db-setup         Set up database with migrations"
    @echo "  db-reset         Reset database completely"
    @echo ""
    @echo "Build:"
    @echo "  build            Build all services"
    @echo "  build-frontend   Build frontend only"
    @echo "  build-backend    Build backend only"
    @echo ""
    @echo "Testing:"
    @echo "  test             Run all tests"
    @echo "  test-frontend    Run frontend tests"
    @echo "  test-backend     Run backend tests"
    @echo ""
    @echo "Code Quality:"
    @echo "  lint             Lint all code"
    @echo "  format           Format all code"
    @echo ""
    @echo "Utilities:"
    @echo "  clean            Clean up build artifacts"
    @echo "  help             Show this help message"