---
title: glin-forge instantiate
description: Create a contract instance from uploaded code
---

# glin-forge instantiate

Instantiate a contract from previously uploaded code hash.

## Synopsis

```bash
glin-forge instantiate <CODE_HASH> [OPTIONS]
```

## Description

The `instantiate` command creates a new contract instance from code that was previously uploaded using `glin-forge upload`. This is the second step of a two-step deployment process, commonly used in factory patterns where the same code is deployed multiple times with different parameters.

## Arguments

### `<CODE_HASH>`

**Required.** The code hash from a previous upload.

- 32-byte hex string with `0x` prefix
- Obtained from `glin-forge upload` output
- Must exist on the blockchain

**Example:**
```
0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
```

## Options

### `-a, --account <ACCOUNT>`

**Required.** Account to use for instantiation.

- Can be development account (`alice`, `bob`, etc.)
- Must have sufficient balance
- Becomes contract creator

### `-c, --args <ARGS>`

Constructor arguments (comma-separated).

- Format depends on constructor signature
- Use quotes for strings
- Parsed from metadata if provided

### `-v, --value <VALUE>`

Value to send with instantiation (in GLIN).

- **Default**: `0`
- Required if constructor is payable

### `-m, --metadata <PATH>`

Path to contract metadata JSON.

- Used to parse constructor arguments
- Auto-detected from `target/ink/metadata.json`
- Required for complex argument types

### `-n, --network <NETWORK>`

Target network.

- **Default**: `testnet`
- Options: `testnet`, `mainnet`, `local`

### `--salt <SALT>`

Salt for deterministic deployment.

- Hex string
- Same code + salt = same address
- Optional for predictable addresses

### `-y, --yes`

Skip confirmation prompt.

## Examples

### Basic Instantiation

```bash
glin-forge instantiate 0x1234... \
  --account alice \
  --args "1000000,MyToken,MTK"
```

### With Value

```bash
glin-forge instantiate 0x1234... \
  --account alice \
  --value 10 \
  --args "1000000"
```

### Multiple Instances

```bash
# Same code, different parameters
glin-forge instantiate 0x1234... --args "1000000,TokenA,TKA" --account alice
glin-forge instantiate 0x1234... --args "2000000,TokenB,TKB" --account alice
glin-forge instantiate 0x1234... --args "3000000,TokenC,TKC" --account alice
```

### Deterministic Address

```bash
glin-forge instantiate 0x1234... \
  --account alice \
  --args "1000000" \
  --salt 0x0000000000000000000000000000000000000000000000000000000000000001
```

## Output

### Success Output

```
Instantiating contract from code hash...

Code Hash: 0x1234567890abcdef...
Network: testnet
Account: alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)
Constructor: new
Arguments: [1000000, "MyToken", "MTK"]
Value: 0 GLIN

Proceed with instantiation? [y/N]: y

✓ Connected to wss://testnet.glin.network

Instantiating contract...
⠋ Waiting for finalization... (block #123457)

✓ Contract instantiated successfully!

Contract Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
Block: #123457
Transaction: 0xabcd...ef02

Explorer:
  https://explorer-testnet.glin.network/contract/5FHneW46...

Next steps:
  # Query the contract
  glin-forge query 5FHneW46... <method> [args...]

  # Call a method
  glin-forge call 5FHneW46... <method> [args...] --account alice
```

## Factory Pattern Workflow

Complete two-step deployment:

```bash
# Step 1: Upload code once
CODE_HASH=$(glin-forge upload \
  --wasm ./target/ink/my_token.wasm \
  --account alice \
  --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

echo "Code uploaded: $CODE_HASH"

# Step 2: Instantiate multiple times
for i in {1..5}; do
  CONTRACT=$(glin-forge instantiate $CODE_HASH \
    --account alice \
    --args "1000000,Token$i,TK$i" \
    --yes 2>&1 | grep "Contract Address:" | awk '{print $3}')

  echo "Deployed Token$i: $CONTRACT"
done
```

## Deterministic Deployment

Create contracts with predictable addresses:

```bash
# Using salt for deterministic address
glin-forge instantiate 0x1234... \
  --account alice \
  --args "1000000" \
  --salt 0x$(printf '%064x' 1)

# Same code + same salt + same deployer = same address
# Useful for:
# - Predictable contract addresses
# - CREATE2-style deployment
# - Address verification before deployment
```

## Error Handling

### Code Hash Not Found

**Problem:**
```
✗ Error: Code hash not found on chain
  Code Hash: 0x1234...
```

**Solution:**
Upload the code first:
```bash
glin-forge upload --wasm ./contract.wasm --account alice
```

### Invalid Arguments

**Problem:**
```
✗ Error: Invalid constructor arguments
  Expected: (u128, String, String)
  Provided: (1000000)
```

**Solution:**
Provide all required arguments:
```bash
glin-forge instantiate 0x1234... \
  --args "1000000,MyToken,MTK" \
  --account alice
```

### Salt Already Used

**Problem:**
```
✗ Error: Contract already exists at this address
  This code hash + salt + deployer combination is already used
```

**Solution:**
Use a different salt or deployer:
```bash
glin-forge instantiate 0x1234... \
  --args "1000000" \
  --salt 0x$(printf '%064x' 2) \
  --account alice
```

## Comparison: instantiate vs deploy

| Feature | `instantiate` | `deploy` |
|---------|---------------|----------|
| Requires uploaded code | ✅ | ❌ |
| Uploads code | ❌ | ✅ |
| Creates instance | ✅ | ✅ |
| Use case | Factory pattern | Single deployment |
| Steps | 1 (after upload) | 2 (upload + instantiate) |
| Reuses code | ✅ | ❌ |

## Use Cases

### Token Factory

Create multiple tokens from same code:

```bash
# Upload ERC20 code
CODE_HASH=$(glin-forge upload --account factory --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

# Deploy multiple tokens
glin-forge instantiate $CODE_HASH --args "1000000,USDT,USDT" --account factory
glin-forge instantiate $CODE_HASH --args "2000000,USDC,USDC" --account factory
glin-forge instantiate $CODE_HASH --args "5000000,DAI,DAI" --account factory
```

### Multi-Network Deployment

Deploy same code on multiple networks:

```bash
# Upload on testnet
CODE_HASH_TEST=$(glin-forge upload --network testnet --account alice --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

# Upload on mainnet (same code = same hash)
CODE_HASH_MAIN=$(glin-forge upload --network mainnet --account production --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

# Verify hashes match
if [ "$CODE_HASH_TEST" == "$CODE_HASH_MAIN" ]; then
  echo "✓ Code hashes match"

  # Instantiate on both networks
  glin-forge instantiate $CODE_HASH_TEST --network testnet --account alice --args "1000000"
  glin-forge instantiate $CODE_HASH_MAIN --network mainnet --account production --args "1000000"
fi
```

### Upgradeable Contracts

Use code hash for proxy patterns:

```bash
# Upload new implementation
NEW_CODE_HASH=$(glin-forge upload --account alice --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

# Call proxy to upgrade
glin-forge call $PROXY_CONTRACT upgrade $NEW_CODE_HASH --account admin
```

## Troubleshooting

### Metadata Not Found

**Problem:**
Cannot parse arguments without metadata.

**Solution:**
Provide metadata explicitly:
```bash
glin-forge instantiate 0x1234... \
  --metadata ./target/ink/metadata.json \
  --args "1000000,MyToken,MTK" \
  --account alice
```

### Insufficient Gas

**Problem:**
Instantiation fails due to gas limits.

**Solution:**
Increase gas limits:
```bash
glin-forge instantiate 0x1234... \
  --account alice \
  --args "1000000" \
  --gas-limit 10000000000,3000000
```

## Related Commands

- [`upload`](./upload) - Upload code to get hash
- [`deploy`](./deploy) - Upload and instantiate in one step
- [`call`](../interaction/call) - Call contract methods
- [`query`](../interaction/query) - Query contract state

## See Also

- [Factory Pattern Guide](../../advanced/factory-pattern) - Factory contract patterns
- [Deterministic Deployment](../../advanced/deterministic-deploy) - Predictable addresses
- [Deploying to Mainnet](../../guides/deploying-mainnet) - Production deployment guide
