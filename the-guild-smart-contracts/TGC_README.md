# TheGuild Contribution Token (TGC)

The TheGuild Contribution Token (TGC) is a simple ERC20 token used to reward contributors. It is owner-mintable and supports batch minting with a distribution ID to prevent duplicate distributions.

## Key features

- ERC20 token (symbol: TGC)
- Owner-only minting
- `mintWithReason(to, amount, reason)` — mints and emits `ContributionTokenMinted(to, amount, reason)` where `reason` is a `bytes32` reference (e.g., a ticket id)
- `batchMint(distributionId, recipients[], amounts[], reasons[])` — mints to many recipients in one call and marks `distributionId` as executed so it cannot be re-used. Use this to ensure a CSV or distribution is only applied once.

## `distributionId`

`distributionId` should be a unique identifier for a distribution batch. Recommended approaches:

- `keccak256(bytes(csvContents))` — compute a hash of the CSV file contents and use that as the id.
- `keccak256(abi.encodePacked(fileName, timestamp))` — if you want to include a timestamp.

The contract tracks which `distributionId`s have been executed and will revert if the same id is passed twice.

## CSV format and scripts

Scripts in the `script/` folder support minting via CSV input. CSV format:
```
<address>,<amount>,<hex32_reason>
```
- `address` — recipient address (0x...)
- `amount` — integer amount (the Node helper multiplies by token decimals by default)
- `hex32_reason` — a 0x-prefixed hex string up to 32 bytes. You can also provide a plain string and the Node helper will hex-encode it.

Examples:
- `script/DeployAndMintTGC.s.sol` — deploys a new TGC and batch mints the CSV contents (computes `distributionId = keccak256(csv)`).
- `script/MintTGCExisting.s.sol` — attaches to an existing TGC address and batch mints the CSV contents.
- `script/mint_from_csv.js` — Node helper that performs batched `batchMint` calls. Useful for large lists and retries.

## Tests

There are unit tests under `test/` that verify `mintWithReason`, `batchMint`, and the `distributionId` guard.

## GitHub issue workflow (example)

1. Contributor closes issue #65 and requests reward.
2. Admin prepares a CSV with recipients, amounts, and reasons (include the issue id in reason where useful).
3. Compute `distributionId = keccak256(csvContents)` and run the deploy/mint script.
4. The contract will mark the `distributionId` executed so you cannot accidentally re-run the same distribution.

If you want, I can add an example script which computes `distributionId` and calls the Forge script automatically, or a small GitHub Action that runs the Node helper on a CSV uploaded to the repo.
