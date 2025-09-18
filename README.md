# The Guild Genesis

[![CI](https://github.com/your-username/TheGuildGenesis/workflows/CI/badge.svg)](https://github.com/your-username/TheGuildGenesis/actions)

A peer-run organization where software developers certify each other's skills, learn together, and create opportunities. Built on the idea that developers are stronger when united.

## Project Structure

This is a monorepo containing:

- **`frontend/`** - Astro + React frontend with Web3 integration
- **`backend/`** - Rust backend with Axum, SQLx, and SIWE authentication
- **`indexer/`** - Rust indexing service for querying Ethereum logs and storing in PostgreSQL
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

### Indexer
- **Rust** - Systems programming language
- **Axum** - Web framework for HTTP API
- **Alloy** - Ethereum RPC client for querying blockchain data
- **SQLx** - Async SQL toolkit for PostgreSQL storage
- **PostgreSQL** - Database for storing indexed logs

### Smart Contracts
- **Solidity** - Smart contract programming language
- **Foundry** - Fast, portable and modular toolkit for Ethereum application development

## Quick Start

### Prerequisites
- [Docker](https://www.docker.com/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (for smart contracts)

### Development Workflow

#### Quick Start

```bash
cd backend
cargo install sqlx-cli --no-default-features --features rustls,postgres  
cargo sqlx prepare -- --bin guild-backend
```

```bash
docker-compose up -d
```

**Access the applications:**
- Frontend: http://localhost:4321
- Backend API: http://localhost:3001
- Indexer API: http://localhost:3002
- PostgreSQL: localhost:5432

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

#### Indexer Service

The indexer service provides HTTP endpoints to trigger blockchain indexing:

```bash
# Health check
curl http://localhost:3002/health

# Index logs from latest block
curl -X POST http://localhost:3002/index/logs \
  -H "Content-Type: application/json" \
  -d '{
    "rpc_url": "https://reth-ethereum.ithaca.xyz/rpc",
    "chain_id": 1
  }'

# Index logs with specific filter
curl -X POST http://localhost:3002/index/logs/filter \
  -H "Content-Type: application/json" \
  -d '{
    "rpc_url": "https://reth-ethereum.ithaca.xyz/rpc",
    "from_block": 19000000,
    "to_block": 19000010,
    "address": "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
    "event_signature": "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
  }'

# Get indexing status
curl http://localhost:3002/status/1
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
- [x] Rust indexer service with Alloy
- [x] Web3 wallet integration
- [x] Basic profile and badge system
- [x] Smart contracts for on-chain badges
- [x] Ethereum log indexing and storage
- [ ] SIWE authentication
- [ ] Database models and migrations
- [ ] API endpoints for profiles and badges

### V1+ (Future)
- [ ] Gasless transactions
- [ ] Badge hierarchy and categories
- [ ] Activity and contribution tokens
- [ ] DAO governance
- [ ] Social features

## Development Philosophy

- **Simple first, complex later** - Start with MVP, iterate
- **Non-profit, member-driven** - Community ownership
- **Horizontal governance** - Flat organization structure
- **Action over endless talk** - Build and ship
- **We use what we build** - Dogfooding our own tools

## Contributing

This is a community-driven project. Join our [Discord](https://discord.gg/pg4UgaTr) to discuss features, propose changes, and contribute to the codebase.

## License

See [LICENSE](LICENSE) file for details.
