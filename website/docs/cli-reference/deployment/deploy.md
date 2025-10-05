---
title: glin-forge deploy
description: Deploy smart contracts to GLIN Network
---

# glin-forge deploy

Deploy a smart contract to GLIN Network (upload code and instantiate in one step).

## Synopsis

```bash
glin-forge deploy [OPTIONS]
```

## Description

The `deploy` command uploads your contract's WASM bytecode to the blockchain and creates an instance in a single transaction. This is the most common deployment method, equivalent to running `upload` followed by `instantiate`.

## Options

### Contract Options

#### `-w, --wasm <PATH>`

Path to the WASM file.

- **Auto-detected** from `target/ink/*.wasm` if not specified
- Must be compiled contract bytecode

#### `-m, --metadata <PATH>`

Path to the metadata JSON file.

- **Auto-detected** from `target/ink/metadata.json` if not specified
- Contains contract ABI and constructor information

### Constructor Options

#### `-c, --args <ARGS>`

Constructor arguments (comma-separated).

- Format: `arg1,arg2,arg3`
- Types parsed from metadata
- Use quotes for strings: `"MyToken"`

**Examples:**
```bash
--args "1000000"                    # Single argument
--args "1000000,MyToken,MTK"        # Multiple arguments
--args "true,5GrwvaEF...,100"      # Mixed types
```

#### `-v, --value <VALUE>`

Value to send with deployment (in GLIN).

- **Default**: `0`
- Required if constructor is payable
- Format: decimal number

**Examples:**
```bash
--value 10        # Send 10 GLIN
--value 0.5       # Send 0.5 GLIN
--value 0         # Send nothing (default)
```

### Network Options

#### `-n, --network <NETWORK>`

Target network for deployment.

- **Default**: `testnet`
- **Options**: `testnet`, `mainnet`, `local`, or custom network name

#### `-a, --account <ACCOUNT>`

Account to use for signing the deployment transaction.

- **Required**
- Can be development account (`alice`, `bob`, etc.) or custom account
- Must have sufficient balance for gas and value

### Gas Options

#### `-g, --gas-limit <GAS>`

Override automatic gas estimation.

- Format: `refTime,proofSize` or just `refTime`
- **Default**: Auto-estimated (5B refTime, 2M proofSize)
- Use if auto-estimation fails

**Examples:**
```bash
--gas-limit 5000000000              # Just refTime
--gas-limit 5000000000,2000000      # Both refTime and proofSize
```

### Advanced Options

#### `--salt <SALT>`

Salt for deterministic deployment.

- Hex string
- Same code + salt = same address
- Useful for factory patterns

**Example:**
```bash
--salt 0x1234567890abcdef
```

### Interaction Options

#### `-y, --yes`

Skip confirmation prompt and deploy immediately.

- Useful for scripts and CI/CD
- **Warning**: Use carefully, especially on mainnet

## Examples

### Basic Deployment

Deploy with auto-detected artifacts:

```bash
glin-forge deploy --network testnet --account alice
```

### Deploy with Arguments

Deploy ERC20 token with initial supply:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"
```

### Deploy with Value

Deploy payable contract and send 10 GLIN:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --value 10
```

### Deploy Specific Files

Deploy specific WASM and metadata:

```bash
glin-forge deploy \
  --wasm ./target/ink/my_token.wasm \
  --metadata ./target/ink/metadata.json \
  --account alice
```

### Skip Confirmation

Deploy without prompts (for scripts):

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000" \
  --yes
```

### Deploy to Mainnet

Deploy to production network:

```bash
glin-forge deploy \
  --network mainnet \
  --account production-account \
  --args "1000000,MyToken,MTK" \
  --value 0
```

### Deterministic Deployment

Deploy with salt for deterministic address:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000" \
  --salt 0x0000000000000000000000000000000000000000000000000000000000000001
```

## Output

### Interactive Prompt

```
Deploying contract...

Contract artifacts:
  WASM: ./target/ink/my_token.wasm (23.4 KB)
  Metadata: ./target/ink/metadata.json

Deployment details:
  Network: testnet
  RPC: wss://testnet.glin.network
  Account: alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)
  Constructor: new
  Arguments: [1000000, "MyToken", "MTK"]
  Value: 0 GLIN

Gas estimation:
  refTime: 5,000,000,000
  proofSize: 2,000,000

Estimated deployment cost: ~0.005 GLIN

Proceed with deployment? [y/N]:
```

### Success Output

```
Proceed with deployment? [y/N]: y

Connecting to network...
✓ Connected to wss://testnet.glin.network

✓ Using account: alice
  Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Balance: 100.0 GLIN

Deploying contract...
⠋ Waiting for finalization... (block #123456)

✓ Contract deployed successfully!

Contract info:
  Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Code Hash: 0x1234567890abcdef...
  Block: #123456
  Transaction: 0xabcd...ef01

Explorer:
  https://explorer-testnet.glin.network/contract/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty

Next steps:
  # Query the contract
  glin-forge query 5FHneW46... <method> [args...]

  # Call a method
  glin-forge call 5FHneW46... <method> [args...] --account alice

  # Generate TypeScript types
  glin-forge typegen --contract 5FHneW46... --output ./types

  # Verify on explorer
  glin-forge verify 5FHneW46... --network testnet
```

### Error Cases

**Insufficient balance:**
```
✗ Error: Insufficient balance
  Account: 5GrwvaEF... (alice)
  Balance: 0.5 GLIN
  Required: ~5.0 GLIN (gas + value)

  Request testnet tokens: https://faucet-testnet.glin.network
```

**Constructor not found:**
```
✗ Error: Constructor 'init' not found in metadata
  Available constructors: new, default
```

**Invalid arguments:**
```
✗ Error: Invalid constructor arguments
  Expected: (u128, String, String)
  Provided: (1000000, MyToken)

  Usage: --args "1000000,MyToken,MTK"
```

**Network connection failed:**
```
✗ Error: Failed to connect to network
  Network: testnet
  RPC: wss://testnet.glin.network
  Reason: Connection timeout

  Check your internet connection and try again.
```

## Deployment Flow

The `deploy` command performs these steps:

1. **Detect Artifacts**: Find WASM and metadata files
2. **Parse Metadata**: Extract constructor information
3. **Connect to Network**: Establish RPC connection
4. **Get Account**: Load signing account (dev or custom)
5. **Encode Arguments**: Parse and encode constructor args
6. **Estimate Gas**: Calculate required gas limits
7. **Show Confirmation**: Display deployment details (unless `--yes`)
8. **Submit Transaction**: Send `instantiate_with_code` extrinsic
9. **Wait for Finalization**: Monitor block inclusion
10. **Parse Events**: Extract contract address from events
11. **Display Results**: Show contract address and explorer link

## Gas Estimation

### Automatic Estimation

By default, GLIN Forge uses:
- **refTime**: 5,000,000,000 (5 billion)
- **proofSize**: 2,000,000 (2 million)

These are conservative estimates that work for most contracts.

### Custom Gas Limits

Override if needed:

```bash
glin-forge deploy \
  --account alice \
  --gas-limit 10000000000,3000000
```

### Gas Tips

The output includes helpful tips:

```
Gas Estimation:
  → refTime: 5,000,000,000
  → proofSize: 2,000,000
  ℹ Using auto-estimated gas limit
    Tip: Add 20% buffer for safety
```

### Gas Costs

Deployment costs depend on:
- WASM size (larger = more expensive)
- Constructor complexity
- Initial storage writes
- Network congestion

**Typical range**: 0.001 - 0.01 GLIN on testnet

## Constructor Arguments

### Argument Parsing

Arguments are parsed based on the constructor signature in metadata:

```rust
// Constructor in contract
#[ink(constructor)]
pub fn new(
    total_supply: Balance,
    name: String,
    symbol: String,
) -> Self { ... }
```

```bash
# Corresponding CLI arguments
glin-forge deploy --args "1000000,MyToken,MTK"
```

### Supported Types

| Rust Type | CLI Format | Example |
|-----------|------------|---------|
| `u8`, `u32`, `u64`, `u128` | Number | `1000000` |
| `String` | Quoted string | `"MyToken"` |
| `bool` | `true` or `false` | `true` |
| `AccountId` | SS58 address | `5GrwvaEF...` |
| `Option<T>` | Value or `null` | `100` or `null` |
| `Vec<T>` | Comma-separated | `1,2,3,4` |

### Complex Arguments

For arrays or nested types:

```bash
# Array of numbers
--args "[1,2,3,4,5]"

# Optional value
--args "Some(100)"
--args "None"

# Tuple
--args "(alice,1000)"
```

## Account Balance

Before deploying, ensure your account has sufficient balance:

```bash
glin-forge balance alice --network testnet
```

If balance is low, request testnet tokens:
- **Testnet Faucet**: https://faucet-testnet.glin.network

## Saving Contract Address

Save the deployed contract address for later use:

```bash
# Deploy and extract address
CONTRACT_ADDR=$(glin-forge deploy --network testnet --account alice --yes 2>&1 | grep "Address:" | awk '{print $2}')

echo $CONTRACT_ADDR
# 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty

# Use in subsequent commands
glin-forge query $CONTRACT_ADDR totalSupply
glin-forge call $CONTRACT_ADDR transfer $RECIPIENT 1000 --account alice
```

## Deployment Strategies

### One-Step Deployment (deploy)

Best for:
- Simple contracts
- One-time deployments
- Development and testing

```bash
glin-forge deploy --account alice
```

### Two-Step Deployment (upload + instantiate)

Best for:
- Factory patterns
- Multiple instances from same code
- Saving on repeated deployments

```bash
# Upload once
CODE_HASH=$(glin-forge upload --account alice)

# Instantiate many times
glin-forge instantiate $CODE_HASH --args "1000000" --account alice
glin-forge instantiate $CODE_HASH --args "2000000" --account bob
```

See:
- [`upload`](./upload) - Upload code only
- [`instantiate`](./instantiate) - Create instance from code hash

## Troubleshooting

### Build Before Deploy

Always build before deploying:

```bash
glin-forge build --release
glin-forge deploy --account alice
```

### WASM Not Found

**Problem:**
```
✗ Error: WASM file not found
  Looked in: ./target/ink/*.wasm
```

**Solution:**
Build the contract first:
```bash
glin-forge build --release
```

Or specify path:
```bash
glin-forge deploy --wasm ./path/to/contract.wasm
```

### Metadata Not Found

**Problem:**
```
✗ Error: Metadata file not found
```

**Solution:**
```bash
glin-forge build --release
# Generates both WASM and metadata
```

### Deployment Hangs

**Problem:**
Deployment hangs at "Waiting for finalization..."

**Solution:**
1. Check network connection
2. Try different RPC endpoint
3. Check if network is syncing
4. Wait longer (can take 10-30 seconds)

### Transaction Failed

**Problem:**
```
✗ Transaction failed!
  Reason: Module error: contracts::ContractTrapped
```

**Solution:**
Constructor panicked. Check your contract logic:
- Review constructor code
- Test locally first
- Check for overflows or invalid conditions

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Deploy to Testnet
  env:
    DEPLOY_ACCOUNT_SEED: ${{ secrets.TESTNET_ACCOUNT_SEED }}
  run: |
    # Import account
    glin-forge account import ci-deployer --seed "$DEPLOY_ACCOUNT_SEED"

    # Deploy contract
    glin-forge deploy \
      --network testnet \
      --account ci-deployer \
      --args "1000000,MyToken,MTK" \
      --yes \
      > deploy_output.txt

    # Extract contract address
    CONTRACT_ADDR=$(grep "Address:" deploy_output.txt | awk '{print $2}')
    echo "CONTRACT_ADDR=$CONTRACT_ADDR" >> $GITHUB_ENV
```

## Related Commands

- [`build`](../project/build) - Build contract before deploying
- [`upload`](./upload) - Upload code without instantiating
- [`instantiate`](./instantiate) - Create instance from code hash
- [`verify`](../verification/verify) - Verify deployed contract
- [`query`](../interaction/query) - Query deployed contract
- [`call`](../interaction/call) - Call deployed contract methods

## See Also

- [Deploying to Testnet Guide](../../guides/deploying-testnet) - Complete deployment guide
- [Deploying to Mainnet Guide](../../guides/deploying-mainnet) - Production deployment
- [Gas Optimization](../../guides/gas-optimization) - Reduce deployment costs
- [Quick Start](../../getting-started/quick-start) - End-to-end example
