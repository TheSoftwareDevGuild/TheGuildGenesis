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
- [ ] SIWE authentication
- [ ] Database models and migrations
- [ ] API endpoints for profiles and badges

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
