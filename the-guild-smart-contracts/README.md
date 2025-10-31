## TheGuild Smart Contracts

The idea is to be able to create badges and then send them to other users (denominated by their address). 

### Badges
Anybody can create a badge. The idea is to let the community input whatever they want and see what they come up with. Badges should be reused and have a description and a unique name. 

We will create a smart contract TheGuildBadgeRegistry that will have a list of badges with unique non-duplicate names.

### Badge Ranking
Community members can vote on badge relevancy to filter spam and promote the most relevant badges. The BadgeRanking contract tracks upvotes per badge and prevents duplicate voting from the same address.

### Attestations
Then, we let users create an attestation of a badge to another user. The attestation can contain an optional justification (link to a project, or text explanation).

To do this, we can just use EAS' already deployed contracts. First we will register our schema (using their sdk or their UI): "bytes32 badgeName, bytes justification". Then, in the front end, we can use their sdk to create attestation from one user to another, referencing our schema id, the unique badge name, and a justification. We can also use EAS Resolver contract to prevent duplicate badges and reward attestations with Activity Token.

### Integration
For detailed frontend integration instructions, see [INTEGRATION.md](./INTEGRATION.md).

## Deployments

### Amoy 

Salt: "theguild_v_0.1.0"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0x5db978bc69e54250f577ed343273508baea136cd 

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0xc142ab6b4688b7b81cb4cc8b305f517bba3bfd25 

EAS Schema ID:
0xb167f07504166f717f2a2710dbcfbfdf8fad6e8c6128c1a7fa80768f61b1d0b2

### Amoy for dev
Salt: "theguild_v_0.1.1_dev"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0x82eba5400b4e914a9b5e4573867b9a73c488c1ed 

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0x8ac95734e778322684f1d318fb7633777baa8427 

EAS Schema ID:
0x8ef2fdb896e42534302cc992c2b2daf614ccabf3fc6d78ce15dc35534b8fa481

### Base Sepolia
Salt: "theguild_v_0.1.1"

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

### Tokens

- `TheGuildActivityToken` (symbol `TGA`) is an ERC20 with standard 18 decimals. The deployer is the owner and can mint. See `src/TheGuildActivityToken.sol`.
- The token also acts as an EAS Resolver: it inherits `SchemaResolver` and mints 10 TGA to the attester address on each successful attestation.

#### EAS Resolver behavior (TheGuildActivityToken)

- Inherits `SchemaResolver` and implements:
  - `onAttest(attestation, value)`: mints `10 * 10^decimals()` to `attestation.attester` and returns `true`.
  - `onRevoke(...)`: no-op, returns `true`.
- Constructor requires the global `IEAS` address. Deployment script `script/TheGuildActivityToken.s.sol` auto-selects the EAS address by `chainid` (Base/Optimism/Arbitrum/Polygon, plus testnets) or falls back to `EAS_ADDRESS` env var.
- To use it as a resolver, set this contract address as the `resolver` when registering your EAS Schema. When EAS processes an attestation for that schema, it will call `attest()` on the resolver, which delegates to `onAttest`.
- Learn more about EAS resolvers in the official docs: [Resolver Contracts](https://docs.attest.org/docs/core--concepts/resolver-contracts).

Quick steps:

1. Deploy TGA (resolver):
   - Uses `IEAS` in constructor; see the deploy script for per-chain EAS addresses.
2. Register your schema in EAS with `resolver` set to the deployed TGA address.
3. Create attestations against that schema. Each attestation mints 10 TGA to the attester automatically.

Deploy:

```shell
forge script script/TheGuildActivityToken.s.sol:TheGuildActivityTokenScript \
  --rpc-url <your_rpc_url> \
  --private-key <your_private_key> \
  --broadcast
```

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
