## TheGuild Smart Contracts

The idea is to be able to create badges and then send them to other users (denominated by their address). 

### Badges
Anybody can create a badge. The idea is to let the community input whatever they want and see what they come up with. Badges should be reused and have a description and a unique name. 

We will create a smart contract TheGuildBadgeRegistry that will have a list of badges with unique non-duplicate names.

### Badge Ranking
Community members can vote on badge relevancy to filter spam and promote the most relevant badges. The BadgeRanking contract tracks upvotes per badge and prevents duplicate voting from the same address.

### Attestations
Then, we let users create an attestation of a badge to another user. The attestation can contain an optional justification (link to a project, or text explanation).

To do this, we can just use EAS' already deployed contracts. First we will register our schema (using their sdk or their UI): "bytes32 badgeName, bytes32 justification". Then, in the front end, we can use their sdk to create attestation from one user to another, referencing our schema id, the unique badge name, and a justification. We can also use EAS Resolver contract to prevent duplicate badges and reward attestations with Activity Token.

### Integration
For detailed frontend integration instructions, see [INTEGRATION.md](./INTEGRATION.md).

## Deployments

### Amoy 

Salt: "theguild_v_0.1.2"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0x4649490B118389d0Be8F48b8953eFb235d8CB545

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0x94f5F12BE60a338D263882a1A49E81ca8A0c30F4

TheGuildAttestationResolver
https://amoy.polygonscan.com/address/0x8B481fB56b133b590348B2B3B1D3Ae2fce0D4324

TheGuildBadgeRanking
https://amoy.polygonscan.com/address/0x435576DFA5B151a6A1c8a4B5EdDfB4ceEd5A55aF

EAS Schema ID:
0xbcd7561083784f9b5a1c2b3ddb7aa9db263d43c58f7374cfa4875646824a47de

### Amoy for dev
Salt: "theguild_v_0.1.3_dev"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0x82eba5400b4e914a9b5e4573867b9a73c488c1ed 

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0x8ac95734e778322684f1d318fb7633777baa8427 

EAS Schema ID:
0x8ef2fdb896e42534302cc992c2b2daf614ccabf3fc6d78ce15dc35534b8fa481

### Base Sepolia
Salt: "theguild_v_0.1.3"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0xba838e90ca2a84aed0de2119e7e6f53b9174ce42 

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0xc142ab6b4688b7b81cb4cc8b305f517bba3bfd25 

EAS Schema ID:
0x7d76e8bb0a26aa015c4adff8564d236d53689e99c3f4b4e2802d99f0144bce8e

## Foundry Usage

https://book.getfoundry.sh/

### Build

```shell
$ forge build
```

### Dependencies

OpenZeppelin contracts (for ERC20):

```shell
forge install OpenZeppelin/openzeppelin-contracts
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Deterministic Deployments (CREATE2)

Follow the Foundry guide on deterministic deployments using CREATE2: https://getfoundry.sh/guides/deterministic-deployments-using-create2

We configured `foundry.toml` to pin `solc`, set `evm_version`, and disable metadata hashing for stable bytecode, and enabled `always_use_create_2_factory`.

Use the deploy script with a salt (env var `CREATE2_SALT`) to deploy at a deterministic address:

```shell
# Example on Sepolia (replace RPC and PK)
export CREATE2_SALT=1
forge script script/TheGuildBadgeRegistry.s.sol:TheGuildBadgeRegistryScript \
  --rpc-url <your_rpc_url> \
  --private-key <your_private_key> \
  --broadcast

# Use a specific salt value (hex string -> uint)
export CREATE2_SALT=123456
forge script script/TheGuildBadgeRegistry.s.sol:TheGuildBadgeRegistryScript --rpc-url <RPC> --private-key <PK> --broadcast
```

### Tokens & Resolver

- `TheGuildActivityToken` (symbol `TGA`) is a plain ERC20 with standard 18 decimals. The deployer is the initial owner and can mint. See `src/TheGuildActivityToken.sol`.
- `TheGuildAttestationResolver` is an EAS `SchemaResolver` that mints TGA to the attester on successful attestations and enforces basic validity rules. It takes the global `IEAS`, the deployed `TheGuildActivityToken`, and the deployed `TheGuildBadgeRegistry` in its constructor. See `src/TheGuildAttestationResolver.sol`.

#### EAS Resolver behavior (TheGuildAttestationResolver)

- Inherits `SchemaResolver` and implements:
  - `onAttest(attestation, value)`:
    - Decodes schema data `"bytes32 badgeName, bytes justification"`.
    - Requires `badgeName` exists in `TheGuildBadgeRegistry`.
    - Rejects duplicate attestations for the same `(attester, recipient, badgeName)` using a single-slot `keccak256(attester, recipient, badgeName)` mapping.
    - If valid, mints `10 * 10^decimals()` to `attestation.attester` and returns `true`.
  - `onRevoke(...)`: no-op, returns `true`.
- Deployment wiring:
  - Deploy `TheGuildActivityToken`.
  - Deploy `TheGuildBadgeRegistry` (if not already deployed).
  - Deploy `TheGuildAttestationResolver` with `(IEAS, token, badgeRegistry)`.
  - Transfer ownership of the token to the resolver so it can mint: `token.transferOwnership(resolver)`.
- Register your EAS Schema with `resolver` set to the resolver address (not the token!). When EAS processes an attestation for that schema, it calls the resolver which validates and mints tokens.
- Learn more about EAS resolvers: [Resolver Contracts](https://docs.attest.org/docs/core--concepts/resolver-contracts).

Quick steps:

1. Deploy TGA, Badge Registry, and the Resolver; transfer token ownership to the resolver.
2. Register your schema in EAS with `resolver` set to the resolver address.
3. Create attestations against that schema. Each valid attestation mints 10 TGA to the attester automatically; unknown badges or duplicates are rejected.

Deploy (example):

```shell
forge script script/TheGuildAttestationResolver.s.sol:TheGuildActivityTokenScript \
  --rpc-url <your_rpc_url> \
  --private-key <your_private_key> \
  --broadcast
```

### Contribution Token (TGC)

`TheGuildContributionToken` (symbol `TGC`) is an **upgradeable ERC20** used to reward contributions.

- **Contract**: `src/TheGuildContributionToken.sol`
  - Upgradable via UUPS (`UUPSUpgradeable`)
  - Standard 18 decimals
  - `mint(address to, uint256 amount)` – owner-only mint
  - `mintWithReason(address to, uint256 amount, bytes reason)` – owner-only mint that emits `MintedWithReason`
  - `batchMint(address[] recipients, uint256[] amounts)` – owner-only batch mint
  - `batchMintWithReason(address[] recipients, uint256[] amounts, bytes[] reasons)` – owner-only batch mint with reasons
- **Tests**: `test/TheGuildContributionToken.t.sol`
  - Covers metadata, ownership, minting, `MintedWithReason` event, and batch mint helpers.

#### Deploying the upgradable TGC proxy

Use `script/DeployTGC.s.sol` to deploy the implementation + ERC1967 proxy and call `initialize()` on the proxy:

```shell
export PRIVATE_KEY=your_private_key
forge script script/DeployTGC.s.sol:DeployTGC \
  --rpc-url <your_rpc_url> \
  --broadcast
```

The script logs both the proxy (TGC) address and the implementation address.

#### Batch minting TGC from JSON

Use `script/MintTGCFromJson.s.sol` to batch-mint TGC using `mintWithReason` from a JSON file.

JSON format:

```json
{
  "mints": [
    {
      "recipient": "0x...",
      "amount": "1000000000000000000",
      "reason": "0x..." 
    }
  ]
}
```

- `recipient`: recipient address
- `amount`: amount as a uint256 (string-encoded in JSON)
- `reason`: ABI-encoded bytes explaining the reason (e.g. `abi.encodePacked("issue-123")`)

Usage:

```shell
export PRIVATE_KEY=your_private_key
export TGC_PROXY_ADDRESS=0xYourTGCProxy

# Optional: override JSON path (default: tgc-mints.json)
export JSON_PATH=contribution-tokens-latest.json

# Dry run
export DRY_RUN=true
forge script script/MintTGCFromJson.s.sol:MintTGCFromJson \
  --rpc-url <your_rpc_url>

# Production run
unset DRY_RUN
forge script script/MintTGCFromJson.s.sol:MintTGCFromJson \
  --rpc-url <your_rpc_url> \
  --broadcast
```

Environment variables:

- `PRIVATE_KEY`: signer that owns the TGC proxy
- `TGC_PROXY_ADDRESS`: address of the deployed TGC proxy
- `JSON_PATH`: path to the JSON file (default: `tgc-mints.json`)
- `DRY_RUN`: set to `true` to simulate without broadcasting (default: `false`)

#### Upgrading the TGC implementation

Use `script/UpgradeTGCImplementation.s.sol` to deploy a new implementation and upgrade the existing proxy.

```shell
export PRIVATE_KEY=your_private_key
export TGC_PROXY_ADDRESS=0xYourTGCProxy

forge script script/UpgradeTGCImplementation.s.sol:UpgradeTGCImplementation \
  --rpc-url <your_rpc_url> \
  --broadcast
```

The script:
- Deploys a new `TheGuildContributionToken` implementation
- Calls `upgradeToAndCall` on the proxy (with empty data)
- Logs the proxy and new implementation addresses

### Badge Ranking

`TheGuildBadgeRanking` enables voting/ranking of badges for relevancy. Features:

- Tracks upvotes per badge
- Prevents duplicate voting from the same address on the same badge
- Integrates with `TheGuildBadgeRegistry` to validate badges before voting
- Emits `BadgeUpvoted` event for off-chain indexing

**Key Functions:**
- `upvoteBadge(bytes32 badgeName)`: Vote for a badge (one vote per address per badge)
- `getUpvotes(bytes32 badgeName)`: Get total upvote count for a badge
- `hasVotedForBadge(bytes32 badgeName, address voter)`: Check if address has voted for a badge

Deploy individually:

```shell
# Set BADGE_REGISTRY_ADDRESS to your deployed registry, or use default (Amoy dev)
forge script script/TheGuildBadgeRanking.s.sol:TheGuildBadgeRankingScript \
  --rpc-url <your_rpc_url> \
  --private-key <your_private_key> \
  --broadcast
```

Or use environment variable:

```shell
export BADGE_REGISTRY_ADDRESS=0x8ac95734e778322684f1d318fb7633777baa8427
forge script script/TheGuildBadgeRanking.s.sol:TheGuildBadgeRankingScript \
  --rpc-url <your_rpc_url> \
  --private-key <your_private_key> \
  --broadcast
```

### Batch Badge Creation

The `CreateBadgesFromJson.s.sol` script allows batch creation of badges from JSON data.

#### JSON Format

Prepare your badges data in JSON format:
```json
{
  "badges": [
    {
      "name": "Rust",
      "description": "Know how to code in Rust"
    },
    {
      "name": "Solidity",
      "description": "Know how to code in Solidity"
    }
  ]
}
```

- `name`: Name of the badge (max 32 characters, will be converted to bytes32)
- `description`: Description of the badge (max 32 characters, will be converted to bytes32)

#### Usage

```shell
# Using the helper script (recommended)
export PRIVATE_KEY=your_private_key
export RPC_URL=https://polygon-amoy.drpc.org
export BADGE_REGISTRY_ADDRESS=0x8ac95734e778322684f1d318fb7633777baa8427

# Dry run first (uses badges-latest.json by default)
./run_batch_badges.sh true

# Dry run with custom JSON file
./run_batch_badges.sh badges.json true

# Production run (uses badges-latest.json by default)
./run_batch_badges.sh false

# Production run with custom JSON file
./run_batch_badges.sh badges.json false

# Manual approach
# Set environment variables
export PRIVATE_KEY=your_private_key
export JSON_PATH=./badges.json
export BADGE_REGISTRY_ADDRESS=0x8ac95734e778322684f1d318fb7633777baa8427

# Dry run (recommended first)
export DRY_RUN=true
forge script script/CreateBadgesFromJson.s.sol:CreateBadgesFromJson \
  --rpc-url <your_rpc_url>

# Production run
unset DRY_RUN
forge script script/CreateBadgesFromJson.s.sol:CreateBadgesFromJson \
  --rpc-url <your_rpc_url> \
  --broadcast
```

#### Environment Variables

- `PRIVATE_KEY`: Private key for transaction signing
- `JSON_PATH`: Path to the JSON file (default: `badges-latest.json`)
- `BADGE_REGISTRY_ADDRESS`: Address of the deployed TheGuildBadgeRegistry contract (required)
- `DRY_RUN`: Set to `true` for validation without broadcasting (default: `false`)
- `RPC_URL`: RPC endpoint URL (for helper script)

The script will:
1. Read the JSON file directly using `vm.readFile()`
2. Parse and validate all badges
3. Check if each badge already exists (skips if it does)
4. Create badges that don't exist yet
5. Log creation summary with success and skip counts

### Batch Attestations

The `EmitAttestationsJson.s.sol` script allows batch creation of attestations from JSON data using EAS's `multiAttest()` function for gas efficiency.

#### JSON Format

Prepare your attestations data in JSON format:
```json
{
  "attestations": [
    {
      "recipient": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
      "badgeName": "Rust",
      "justification": "Outstanding Rust contributions to the project"
    },
    {
      "recipient": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
      "badgeName": "Solidity",
      "justification": "Excellent Solidity smart contract development"
    }
  ]
}
```

- `recipient`: Ethereum address of the recipient
- `badgeName`: Name of the badge to attest (will be converted to bytes32)
- `justification`: Text explanation or link justifying the attestation

#### Usage

```shell
# Using the helper script (recommended)
export PRIVATE_KEY=your_private_key
export RPC_URL=https://polygon-amoy.drpc.org

# Dry run first (uses attestations-latest.json by default)
./run_batch_attestations.sh true

# Dry run with custom JSON file
./run_batch_attestations.sh attestations.json true

# Production run (uses attestations-latest.json by default)
./run_batch_attestations.sh false

# Production run with custom JSON file
./run_batch_attestations.sh attestations.json false

# Manual approach
# Set environment variables
export PRIVATE_KEY=your_private_key
export JSON_PATH=./attestations.json
export SCHEMA_ID=0xbcd7561083784f9b5a1c2b3ddb7aa9db263d43c58f7374cfa4875646824a47de

# Dry run (recommended first)
export DRY_RUN=true
forge script script/EmitAttestationsJson.s.sol:EmitAttestationsJson \
  --rpc-url <your_rpc_url>

# Production run
unset DRY_RUN
forge script script/EmitAttestationsJson.s.sol:EmitAttestationsJson \
  --rpc-url <your_rpc_url> \
  --broadcast
```

#### Environment Variables

- `PRIVATE_KEY`: Private key for transaction signing
- `JSON_PATH`: Path to the JSON file (default: `attestations-latest.json`)
- `SCHEMA_ID`: EAS schema ID to use (default: Amoy production schema)
- `DRY_RUN`: Set to `true` for validation without broadcasting (default: `false`)
- `EAS_ADDRESS`: EAS contract address (auto-detected for Amoy/Base Sepolia, required for other networks)
- `RPC_URL`: RPC endpoint URL (for helper script)

The script will:
1. Read the JSON file directly using `vm.readFile()`
2. Parse and validate all attestations
3. Create `MultiAttestationRequest` batches (max 50 per batch)
4. Call EAS `multiAttest()` for gas-efficient batch processing
5. Log all attestation UIDs upon completion

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
