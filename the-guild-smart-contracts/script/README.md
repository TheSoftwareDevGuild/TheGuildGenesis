Local usage for TGC scripts

This folder contains scripts and helpers for deploying and minting TheGuild Contribution Token (TGC).

Files
- DeployAndMintTGC.s.sol - Forge script that deploys TGC and mints entries from `script/data/initial_tgc_mints.csv`.
- MintTGCExisting.s.sol - Forge script that attaches to an existing TGC contract and mints entries from `script/data/initial_tgc_mints.csv`.
- data/initial_tgc_mints.csv - Example CSV data. Format: `address,amount,hex32_reason` (amount in token base units, reason as 0x-prefixed hex up to 32 bytes).
- mint_from_csv.js - (optional) Node.js script that reads a CSV and calls `mintWithReason` on an already-deployed TGC contract via ethers.js.

Running locally

Prerequisites
- Node.js >= 18 (for the helper script)
- Foundry (forge & cast) installed for running forge scripts
- An Ethereum RPC URL (local node, ganache, anvil, or public testnet)
- Private key with ETH for gas (or use anvil's default accounts)

1) Run tests

In the `the-guild-smart-contracts` folder:

```bash
forge test -v
```

2) Deploy & mint using Forge scripts

Deploy new TGC and mint using the CSV (reads `script/data/initial_tgc_mints.csv`):

```bash
forge script script/DeployAndMintTGC.s.sol:DeployAndMintTGC --rpc-url <RPC_URL> --private-key <KEY> --broadcast -vvvv
```

Mint to an existing deployed token address:

```bash
forge script script/MintTGCExisting.s.sol:MintTGCExisting --rpc-url <RPC_URL> --private-key <KEY> --broadcast -vvvv --sig "run(address)" --json-args '[["0xYourTokenAddress"]]'
```

(Forge and versions differ; `--sig`/`--json-args` usage might require different quoting depending on shell.)

3) Off-chain helper (Node.js)

The repo includes `mint_from_csv.js` to mint using ethers.js. You can use it instead of the on-chain CSV parser for large lists.

Install deps and run:

```bash
cd the-guild-smart-contracts/script
npm install
node mint_from_csv.js --rpc-url <RPC_URL> --private-key <KEY> --token <TOKEN_ADDRESS> --csv data/initial_tgc_mints.csv
```

Notes & improvements
- The on-chain CSV parser in Forge scripts is simplistic and intended for small example files. For large lists, use the Node helper (or batch the calls).
- Event `ContributionTokenMinted(address,uint256,bytes32)` is emitted on mintWithReason. Consider indexing the `bytes32` reason for easier log filtering.
- If CSV amounts are in tokens (e.g., 1 means 1 TGC), you may want to multiply by 10**decimals in the helper script.

If you'd like, I can add a NPM package manifest and install script dependencies for you, or change the Node helper to support batching and retries.