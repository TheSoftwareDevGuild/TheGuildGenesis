# The Guild Genesis

[![CI](https://github.com/your-username/TheGuildGenesis/workflows/CI/badge.svg)](https://github.com/your-username/TheGuildGenesis/actions)

A peer-run organization where software developers certify each other's skills, learn together, and create opportunities. Built on the idea that developers are stronger when united.

## Project Structure

This is a monorepo containing:

- **`frontend/`** - Astro + React frontend with Web3 integration
- **`backend/`** - Rust backend with Axum, SQLx, and SIWE authentication

## Tech Stack

### Frontend
- **Astro** - Fast static site generator with React islands
- **React** - For interactive Web3 components
- **Tailwind CSS** - Utility-first CSS framework
- **wagmi** - React hooks for Ethereum
- **viem** - TypeScript interface for Ethereum
- **RainbowKit** - Wallet connection UI
- **TanStack Query** - Data fetching and caching
- **TanStack Router** - Type-safe routing

### Backend
- **Rust** - Systems programming language
- **Axum** - Web framework
- **SQLx** - Async SQL toolkit with compile-time checked queries
- **PostgreSQL** - Database
- **SIWE** - Sign-In with Ethereum authentication

## Quick Start

### Prerequisites
- [Nix](https://nixos.org/download.html) with flakes enabled
- [direnv](https://direnv.net/) (optional, for automatic environment loading)
- [just](https://github.com/casey/just) (command runner)

### Setup

1. **Clone and enter the development environment:**
   ```bash
   git clone <repository-url>
   cd TheGuildGenesis
   
   # If using direnv (recommended)
   direnv allow
   
   # Or manually enter the Nix shell
   nix develop
   ```

2. **Install dependencies:**
   ```bash
   just install-all
   ```

### Development Workflow

#### Quick Start

```bash
# Set up the database
just db-setup

# Start both frontend and backend
just dev
```

**Access the applications:**
- Frontend: http://localhost:4321
- Backend API: http://localhost:3001
- PostgreSQL: localhost:5432

#### Individual Services

```bash
# Start database only
just db-start

# Start frontend only
just dev-frontend

# Start backend only
just dev-backend

# Stop database
just db-stop
```

#### Database Management

```bash
# Set up database with migrations
just db-setup

# Reset database completely
just db-reset

# Stop database
just db-stop
```

### Available Commands

Run `just help` to see all available commands:

- **Development:** `just dev`, `just dev-frontend`, `just dev-backend`
- **Database:** `just db-start`, `just db-stop`, `just db-setup`, `just db-reset`
- **Build:** `just build`, `just build-frontend`, `just build-backend`
- **Testing:** `just test`, `just test-frontend`, `just test-backend`
- **Code Quality:** `just lint`, `just format`
- **Utilities:** `just clean`, `just help`

## Features

### V0 (Current)
- [x] Monorepo structure
- [x] Astro frontend with React islands
- [x] Rust backend with Axum
- [x] Web3 wallet integration
- [x] Basic profile and badge system
- [ ] SIWE authentication
- [ ] Database models and migrations
- [ ] API endpoints for profiles and badges

### V1+ (Future)
- [ ] Smart contracts for on-chain badges
- [ ] Gasless transactions
- [ ] Badge hierarchy and categories
- [ ] Activity and contribution tokens
- [ ] DAO governance
- [ ] Social features

## Development Philosophy

- **Nix-first development** - Reproducible environments without Docker overhead
- **Simple first, complex later** - Start with MVP, iterate
- **Non-profit, member-driven** - Community ownership
- **Horizontal governance** - Flat organization structure
- **Action over endless talk** - Build and ship
- **We use what we build** - Dogfooding our own tools

### Why Nix?

This project uses Nix for development instead of Docker because:

- **Reproducible environments** - Everyone gets identical toolchains
- **No container overhead** - Direct process execution, faster builds
- **Simpler setup** - One command (`nix develop`) gets you everything
- **Better performance** - No Docker daemon, faster file system access
- **True reproducibility** - Nix ensures exact same versions across all systems

## Contributing

This is a community-driven project. Join our Discord to discuss features, propose changes, and contribute to the codebase.

## License

See [LICENSE](LICENSE) file for details.