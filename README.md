# The Guild Genesis

[![CI](https://github.com/your-username/TheGuildGenesis/workflows/CI/badge.svg)](https://github.com/your-username/TheGuildGenesis/actions)

A peer-run organization where software developers certify each other's skills, learn together, and create opportunities. Built on the idea that developers are stronger when united.

## Project Structure

This is a monorepo containing:

- **`frontend/`** - Astro + React frontend with Web3 integration
- **`backend/`** - Rust backend with Axum, SQLx, and SIWE authentication
- **`the-guild-smart-contracts/`** - Foundry-based Solidity smart contracts for badge registry

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

### Smart Contracts
- **Solidity** - Smart contract programming language
- **Foundry** - Fast, portable and modular toolkit for Ethereum application development

## Quick Start

### Prerequisites
- [Docker](https://www.docker.com/)
- [Node.js](https://nodejs.org/) (for frontend development)
- [Astro CLI](https://docs.astro.build/en/getting-started/) (for frontend development)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (for smart contracts)

### Environment Setup

Create a `.env` file in the project root with the following variables:

```bash
# Database
DATABASE_URL=postgresql://guild_user:guild_password@localhost:5433/guild_genesis

# Frontend Environment Variables
PUBLIC_WALLET_CONNECT_PROJECT_ID=your_wallet_connect_project_id
PUBLIC_API_URL=http://localhost:3001
PUBLIC_BADGE_REGISTRY_ADDRESS=0x...
PUBLIC_EAS_CONTRACT_ADDRESS=0x...
PUBLIC_ACTIVITY_TOKEN_ADDRESS=0x...
PUBLIC_SCHEMA_ID=0x...

# Discord Bot (if using)
DISCORD_TOKEN=your_discord_bot_token
DISCORD_CLIENT_ID=your_discord_client_id
DISCORD_GUILD_ID=your_discord_guild_id

# Optional: Skip migrations in development
SKIP_MIGRATIONS=1

# Optional: Disable SQLx compile-time validation
SQLX_OFFLINE=true
```

**Required for Production:**
- `DATABASE_URL` - PostgreSQL connection string
- `PUBLIC_WALLET_CONNECT_PROJECT_ID` - Get from [WalletConnect Cloud](https://cloud.walletconnect.com/)
- Contract addresses - Deploy smart contracts first

**Optional for Development:**
- `SKIP_MIGRATIONS=1` - Skip automatic migrations (run manually)
- `SQLX_OFFLINE=true` - Disable SQLx compile-time validation

### Development Workflow

#### Option 1: Docker Compose (Recommended)

We provide multiple Docker Compose configurations for different environments:

```bash
# Development with hot reload
make dev

# Production build
make prod

# Run tests
make test

# Database only
make db

# Stop all services
make stop
```

#### Option 2: Local Development

**Backend:**
```bash
cd backend
cargo install sqlx-cli --no-default-features --features rustls,postgres  
cargo sqlx prepare -- --bin guild-backend
```

**Frontend:**
```bash
cd frontend
npm install
npm run dev
```

**Database:**
```bash
docker compose up -d postgres
```

### Docker Compose Environments

- **`docker-compose.dev.yml`** - Development with hot reload, volume mounts, and dev tools
- **`docker-compose.prod.yml`** - Production with optimized builds and restart policies
- **`docker-compose.test.yml`** - Testing environment with test-specific configurations
- **`docker-compose.db-only.yml`** - Just PostgreSQL for local development

Use `make` commands or specify files directly:
```bash
docker compose -f docker-compose.dev.yml up --build
```

**Access the applications:**
- Frontend: http://localhost:3000
- Backend API: http://localhost:3001
- PostgreSQL: localhost:5433

#### Smart Contracts Development

```bash
# Navigate to smart contracts directory
cd the-guild-smart-contracts

# Build contracts
forge build

# Run tests
forge test

# Run tests with verbose output
forge test -vv

# Deploy to local network (Anvil)
anvil
# In another terminal:
forge script script/TheGuildBadgeRegistry.s.sol:TheGuildBadgeRegistryScript --rpc-url http://localhost:8545 --private-key <PRIVATEK_KEY> --broadcast

# Deploy to testnet/mainnet
forge script script/TheGuildBadgeRegistry.s.sol:TheGuildBadgeRegistryScript --rpc-url <RPC_URL> --private-key <PRIVATE_KEY> --broadcast
```

## Smart Contracts

The `the-guild-smart-contracts/` directory contains our Solidity smart contracts built with Foundry.

### TheGuildBadgeRegistry

A community-driven badge registry where anyone can create badges with unique names and descriptions.

**Key Features:**
- **Community-driven**: Anyone can create badges
- **Unique names**: No duplicate badge names allowed
- **Immutable**: No owner or upgrade mechanism
- **Gas-efficient**: Simple storage patterns
- **Event-driven**: Emits events for badge creation

**Contract Interface:**
```solidity
// Create a new badge
function createBadge(bytes32 name, bytes32 description) external

// Get badge information
function getBadge(bytes32 name) external view returns (bytes32, bytes32, address)

// Check if badge exists
function exists(bytes32 name) external view returns (bool)

// Get total number of badges
function totalBadges() external view returns (uint256)

// Enumerate badges
function badgeNameAt(uint256 index) external view returns (bytes32)
```

**Events:**
```solidity
event BadgeCreated(bytes32 indexed name, bytes32 description, address indexed creator)
```

## Features

### V0 (Current)
- [x] Monorepo structure
- [x] Astro frontend with React islands
- [x] Rust backend with Axum
- [x] Web3 wallet integration
- [x] Basic profile and badge system
- [x] Smart contracts for on-chain badges
- [x] SIWE authentication
- [ ] Database models and migrations
- [x] API endpoints for profiles and badges

### V1+ (Future)
- [ ] Gasless transactions
- [ ] Badge hierarchy and categories
- [ ] Activity and contribution tokens — see [CONTRIBUTION_TOKEN.md](CONTRIBUTION_TOKEN.md) for the Contribution Token specification
- [ ] DAO governance
- [ ] Social features

## Development Philosophy

- **Simple first, complex later** - Start with MVP, iterate
- **Non-profit, member-driven** - Community ownership
- **Horizontal governance** - Flat organization structure
- **Action over endless talk** - Build and ship
- **We use what we build** - Dogfooding our own tools

## Contributing

This is a community-driven project. Join our [Discord](https://discord.gg/axCqT23Xhj) to discuss features, propose changes, and contribute to the codebase.

See detailed steps in [CONTRIBUTION.md](CONTRIBUTION.md).

## License

See [LICENSE](LICENSE) file for details.
