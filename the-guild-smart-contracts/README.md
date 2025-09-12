## TheGuild Smart Contracts

The idea is to be able to create badges and then send them to other users (denominated by their address). 

### Badges
Anybody can create a badge. The idea is to let the community input whatever they want and see what they come up with. Badges should be reused and have a description and a unique name. 

We will create a smart contract TheGuildBadgeRegistry that will have a list of badges with unique non-duplicate names.

### Attestations
Then, we let users create an attestation of a badge to another user. The attestation can contain an optional justification (link to a project, or text explanation).

To do this, we can just use EAS' already deployed contracts. First we will register our schema (using their sdk or their UI): "bytes32 badgeName, bytes32 justification". Then, in the front end, we can use their sdk to create attestation from one user to another, referencing our schema id, the unique badge name, and a justification. We can also use EAS Resolver contract to prevent duplicate badges and reward attestations with Activity Token.

### Integration
For detailed frontend integration instructions, see [INTEGRATION.md](./INTEGRATION.md).

## Deployments

### Amoy 

Salt: "theguild2"

TheGuildActivityToken
https://amoy.polygonscan.com/address/0x5a79Dd0F66E2C1203948dD49634E506b3D8723A0 

TheGuildBadgeRegistry
https://amoy.polygonscan.com/address/0x8baA0d5135D241bd22a9eB35915300aCfB286307 

EAS Schema ID:
0x6228d4b9508624b41476a77a09b7582ddc056b92a725f645df6e337be113c9b1

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
