# TheGuild Contribution Token (TGC)

A standard ERC20 token designed for rewarding contributions to The Guild community. The token features owner-controlled minting, batch operations for gas efficiency, and a maximum supply cap.

## Features

- **Standard ERC20**: Full compatibility with ERC20 standard
- **Ownable**: Only the owner can mint tokens
- **Batch Minting**: Gas-efficient batch operations for multiple recipients
- **Maximum Supply**: Capped at 1 billion tokens to prevent infinite inflation
- **CSV Support**: Scripts designed to work with CSV data for token distribution
- **Comprehensive Testing**: Full test suite covering all functionality

## Contract Details

- **Name**: TheGuild Contribution Token
- **Symbol**: TGC
- **Decimals**: 18 (standard)
- **Max Supply**: 1,000,000,000 TGC (1 billion tokens)
- **Initial Supply**: 0 (all tokens must be explicitly minted)

## Deployment

### Deploy New Contract

```bash
# Deploy with initial owner
forge script script/DeployTGC.s.sol --rpc-url <RPC_URL> --private-key <PRIVATE_KEY> --broadcast

# Deploy with initial minting (modify recipients/amounts in script first)
forge script script/DeployTGC.s.sol:DeployTGC --rpc-url <RPC_URL> --private-key <PRIVATE_KEY> --broadcast
```

### Mint Tokens on Existing Contract

```bash
# Set the contract address
export TGC_TOKEN_ADDRESS=0x...

# Mint tokens (modify recipients/amounts in script first)
forge script script/MintTGC.s.sol --rpc-url <RPC_URL> --private-key <PRIVATE_KEY> --broadcast
```

## CSV Data Format

Create a CSV file with the following format for basic minting:

```csv
address,amount
0x1234567890123456789012345678901234567890,1000
0x2345678901234567890123456789012345678901,2000
```

Or with reasons for GitHub issue tracking:

```csv
address,amount,reason
0x1234567890123456789012345678901234567890,1000,GitHub-Issue-123
0x2345678901234567890123456789012345678901,2000,GitHub-Issue-456
```

- **address**: Ethereum address of the recipient
- **amount**: Amount in whole tokens (will be converted to wei automatically)
- **reason**: GitHub issue ID, ticket reference, or contribution identifier

## Usage Examples

### 1. Deploy Contract Only

```solidity
// In DeployTGC.s.sol, modify the run() function:
function run() external {
    address deployer = vm.addr(vm.envUint("PRIVATE_KEY"));
    deployToken(deployer);
}
```

### 2. Deploy with Initial Distribution

```solidity
// Prepare your recipient data
address[] memory recipients = new address[](3);
uint256[] memory amounts = new uint256[](3);

recipients[0] = 0x1234567890123456789012345678901234567890;
recipients[1] = 0x2345678901234567890123456789012345678901;
recipients[2] = 0x3456789012345678901234567890123456789012;

amounts[0] = 1000; // 1000 TGC
amounts[1] = 2000; // 2000 TGC
amounts[2] = 500;  // 500 TGC

// Deploy with initial minting
deployTokenWithInitialMint(deployer, recipients, amounts);
```

### 3. Mint on Existing Contract

```bash
# Set environment variables
export TGC_TOKEN_ADDRESS=0x... # Your deployed contract address
export PRIVATE_KEY=0x...       # Owner's private key

# Run minting script
forge script script/MintTGC.s.sol --rpc-url <RPC_URL> --broadcast
```

## Testing

Run the complete test suite:

```bash
# Run all tests
forge test

# Run tests with verbose output
forge test -vv

# Run specific test file
forge test --match-path test/TheGuildContributionToken.t.sol

# Run with gas reporting
forge test --gas-report

# Run fuzz tests
forge test --fuzz-runs 1000
```

## Contract Functions

### Owner Functions

- `mint(address to, uint256 amount)`: Mint tokens to a single recipient
- `mintWithReason(address to, uint256 amount, bytes32 reason)`: Mint tokens with a reason (e.g., GitHub issue reference)
- `batchMint(address[] recipients, uint256[] amounts)`: Mint tokens to multiple recipients
- `batchMintWithReasons(address[] recipients, uint256[] amounts, bytes32[] reasons)`: Mint tokens to multiple recipients with reasons
- `transferOwnership(address newOwner)`: Transfer contract ownership
- `renounceOwnership()`: Renounce ownership permanently

### View Functions

- `maxSupply()`: Returns the maximum total supply (1 billion tokens)
- `remainingSupply()`: Returns how many tokens can still be minted
- `totalSupply()`: Returns current total supply
- `balanceOf(address account)`: Returns account balance
- `owner()`: Returns current owner address

### Standard ERC20 Functions

- `transfer(address to, uint256 amount)`: Transfer tokens
- `approve(address spender, uint256 amount)`: Approve spending
- `transferFrom(address from, address to, uint256 amount)`: Transfer on behalf

## Events

- `Mint(address indexed to, uint256 amount)`: Emitted when tokens are minted to a single recipient
- `ContributionTokenMinted(address indexed recipient, uint256 amount, bytes32 indexed reason)`: Emitted when tokens are minted with a reason
- `BatchMint(address indexed owner, uint256 totalAmount, uint256 recipientCount)`: Emitted during batch minting
- Standard ERC20 events: `Transfer`, `Approval`

## Security Features

- **Ownable**: Only the contract owner can mint tokens
- **Zero Address Protection**: Cannot mint to the zero address
- **Zero Amount Protection**: Cannot mint zero tokens
- **Maximum Supply Protection**: Cannot exceed 1 billion total tokens
- **Array Validation**: Batch operations validate array lengths and contents

## Gas Optimization

- **Batch Operations**: Use `batchMint()` for multiple recipients to save gas
- **Efficient Storage**: Minimal storage usage for optimal gas costs
- **Event Optimization**: Events designed for efficient indexing

## Development Notes

- The contract uses OpenZeppelin's battle-tested implementations
- All functions include proper input validation
- Events are emitted for all important state changes
- The contract is designed to be upgrade-safe (no storage conflicts)

## Integration with The Guild

This token is designed to integrate with The Guild's contribution tracking system:

1. **Deployment**: Deploy the contract with The Guild multisig as owner
2. **GitHub Integration**: Use `mintWithReason()` to reference specific GitHub issues/PRs
3. **Database Updates**: Listen for `ContributionTokenMinted` events to update ticket status to "rewarded"
4. **Distribution**: Use CSV files to distribute tokens based on contribution metrics
5. **Tracking**: The `reason` field allows automatic correlation with GitHub issues
6. **Governance**: Token holders can participate in Guild governance (future feature)

### GitHub Issue Tracking Workflow

1. **Contributor completes task**: GitHub issue #123 is closed
2. **Guild admin rewards**: `mintWithReason(contributor, 1000e18, keccak256("GitHub-Issue-123"))`
3. **Event emitted**: `ContributionTokenMinted(contributor, 1000e18, "GitHub-Issue-123")`
4. **Database updated**: Backend service listens for event and marks issue as "rewarded"
5. **Transparency**: All rewards are publicly verifiable on-chain

## Support

For questions or issues:
- GitHub Issues: [Repository Issues](https://github.com/tusharrrr1/TheGuildGenesis/issues)
- Discord: [The Guild Discord](https://discord.gg/axCqT23Xhj)