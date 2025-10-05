---
title: glin-forge upload
description: Upload contract code without instantiation
---

# glin-forge upload

Upload contract WASM code to the blockchain without creating an instance.

## Synopsis

```bash
glin-forge upload [OPTIONS]
```

## Description

The `upload` command uploads contract WASM bytecode to the blockchain and stores it with a unique code hash. Unlike `deploy`, it does not create a contract instance. This is useful for factory patterns where you want to instantiate the same code multiple times with different parameters.

## Options

### `-w, --wasm <PATH>`

**Required.** Path to the WASM file.

- Must be compiled contract bytecode
- Typically in `target/ink/*.wasm`
- Can be auto-detected if in standard location

### `-a, --account <ACCOUNT>`

**Required.** Account to use for uploading.

- Can be development account (`alice`, `bob`, etc.)
- Must have sufficient balance for gas fees
- Account pays for storage deposit

### `-n, --network <NETWORK>`

Target network for upload.

- **Default**: `testnet`
- **Options**: `testnet`, `mainnet`, `local`, or custom

### `-g, --gas-limit <GAS>`

Override automatic gas estimation.

- Format: `refTime,proofSize` or just `refTime`
- **Default**: Auto-estimated

### `-y, --yes`

Skip confirmation prompt.

- Useful for CI/CD pipelines
- Use carefully, especially on mainnet

## Examples

### Basic Upload

```bash
glin-forge upload \
  --wasm ./target/ink/my_token.wasm \
  --account alice \
  --network testnet
```

### Auto-detected WASM

If WASM is in standard location:

```bash
glin-forge build --release
glin-forge upload --account alice
```

### Upload to Mainnet

```bash
glin-forge upload \
  --wasm ./target/ink/my_token.wasm \
  --account production \
  --network mainnet
```

### Scriptable Upload

```bash
# Upload without prompt
CODE_HASH=$(glin-forge upload \
  --account alice \
  --network testnet \
  --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

echo "Uploaded code hash: $CODE_HASH"
```

## Output

### Success Output

```
Uploading contract code...

WASM file: ./target/ink/my_token.wasm (23.4 KB)
Network: testnet
RPC: wss://testnet.glin.network
Account: alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)

Gas estimation:
  refTime: 3,000,000,000
  proofSize: 1,500,000

Proceed with upload? [y/N]: y

Connecting to network...
✓ Connected to wss://testnet.glin.network

Uploading code...
⠋ Waiting for finalization... (block #123456)

✓ Code uploaded successfully!

Code Hash: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
Block: #123456
Transaction: 0xabcd...ef01

Next steps:
  # Instantiate from this code hash
  glin-forge instantiate 0x1234... \
    --account alice \
    --args "1000000,MyToken,MTK"

  # Or use in multiple instantiations
  glin-forge instantiate 0x1234... --args "1000000" --account alice
  glin-forge instantiate 0x1234... --args "2000000" --account bob
```

### Error Cases

**WASM not found:**
```
✗ Error: WASM file not found
  Looked in: ./target/ink/*.wasm

  Build the contract first:
  glin-forge build --release
```

**Insufficient balance:**
```
✗ Error: Insufficient balance
  Account: 5GrwvaEF... (alice)
  Balance: 0.1 GLIN
  Required: ~0.5 GLIN

  Request testnet tokens: https://faucet-testnet.glin.network
```

**Code already exists:**
```
ℹ Info: Code hash already exists on chain
  Code Hash: 0x1234...

  You can still instantiate from this hash:
  glin-forge instantiate 0x1234... --account alice
```

## Use Cases

### Factory Pattern

Upload code once, instantiate many times:

```bash
# 1. Upload code
CODE_HASH=$(glin-forge upload --account alice --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')

# 2. Instantiate multiple instances
glin-forge instantiate $CODE_HASH --args "1000000,TokenA,TKA" --account alice
glin-forge instantiate $CODE_HASH --args "2000000,TokenB,TKB" --account alice
glin-forge instantiate $CODE_HASH --args "3000000,TokenC,TKC" --account alice
```

### Shared Code

Multiple users instantiate from same code:

```bash
# Alice uploads once
glin-forge upload --account alice --network testnet

# Output: Code Hash: 0x1234...

# Bob instantiates
glin-forge instantiate 0x1234... --account bob --args "5000000"

# Charlie instantiates
glin-forge instantiate 0x1234... --account charlie --args "10000000"
```

### Pre-production Verification

Upload to testnet for verification before mainnet:

```bash
# 1. Upload to testnet
glin-forge upload --account alice --network testnet

# 2. Test instantiation
glin-forge instantiate <hash> --account alice --network testnet

# 3. If successful, upload to mainnet
glin-forge upload --account production --network mainnet
```

## Storage Deposit

### What is Storage Deposit?

When uploading code, you pay a refundable deposit for blockchain storage:

- **Deposit amount**: Based on WASM size
- **Refundable**: Get it back when code is removed
- **Required**: Account must have sufficient balance

### Calculating Deposit

```bash
# Deposit ≈ WASM_SIZE_BYTES * DEPOSIT_PER_BYTE

# Example: 24 KB WASM
# Deposit ≈ 24,000 * 0.00001 = 0.24 GLIN
```

### Reclaiming Deposit

Code storage deposit can be reclaimed if code is removed from chain (rare).

## Code Hash

### What is Code Hash?

A unique identifier (32-byte hash) for uploaded WASM code:

- Generated from WASM bytecode using Blake2-256
- Same code = same hash
- Used to instantiate contracts

### Using Code Hash

Save the code hash for later:

```bash
# Upload and save hash
glin-forge upload --account alice --network testnet > upload.log
CODE_HASH=$(grep "Code Hash:" upload.log | awk '{print $3}')

# Save to file
echo $CODE_HASH > code_hash.txt

# Use later
glin-forge instantiate $(cat code_hash.txt) --account alice
```

### Verifying Code Hash

Check if code hash exists on chain:

```bash
# Query chain storage
# (This would require additional glin-forge functionality)
glin-forge query-code-hash 0x1234...
```

## Upload vs Deploy

| Feature | `upload` | `deploy` |
|---------|----------|----------|
| Uploads code | ✅ | ✅ |
| Creates instance | ❌ | ✅ |
| Returns | Code hash | Contract address |
| Use case | Factory pattern | Single deployment |
| Gas cost | Lower | Higher |
| Steps | 1 | 2 (upload + instantiate) |

### When to use `upload`

- Creating multiple instances from same code
- Factory contracts
- Shared code libraries
- Saving gas on repeated deployments

### When to use `deploy`

- Single contract deployment
- Quick development workflow
- Simple use cases

## Two-Step Deployment Workflow

Complete workflow using upload + instantiate:

```bash
# Step 1: Build
glin-forge build --release

# Step 2: Upload code
CODE_HASH=$(glin-forge upload --account alice --yes 2>&1 | grep "Code Hash:" | awk '{print $3}')
echo "Code uploaded: $CODE_HASH"

# Step 3: Instantiate (can do multiple times)
CONTRACT_1=$(glin-forge instantiate $CODE_HASH \
  --args "1000000,TokenA,TKA" \
  --account alice \
  --yes 2>&1 | grep "Address:" | awk '{print $2}')

CONTRACT_2=$(glin-forge instantiate $CODE_HASH \
  --args "2000000,TokenB,TKB" \
  --account alice \
  --yes 2>&1 | grep "Address:" | awk '{print $2}')

echo "Contracts deployed:"
echo "  Token A: $CONTRACT_1"
echo "  Token B: $CONTRACT_2"
```

## Troubleshooting

### WASM Too Large

**Problem:**
```
✗ Error: WASM file too large
  Size: 512 KB
  Max: 256 KB
```

**Solution:**
Optimize contract size:
```bash
# Use release mode
glin-forge build --release

# Review contract for optimization opportunities
# - Remove unused dependencies
# - Use #[ink(inline)] where appropriate
# - Minimize storage
```

### Upload Fails

**Problem:**
Transaction fails during upload.

**Solution:**
1. Check gas limits:
   ```bash
   glin-forge upload --account alice --gas-limit 5000000000,2000000
   ```

2. Verify WASM is valid:
   ```bash
   glin-forge build --verify
   ```

3. Check account balance:
   ```bash
   glin-forge balance alice --network testnet
   ```

## Related Commands

- [`instantiate`](./instantiate) - Create instance from uploaded code
- [`deploy`](./deploy) - Upload and instantiate in one step
- [`build`](../project/build) - Build WASM before upload
- [`verify`](../verification/verify) - Verify uploaded code

## See Also

- [Factory Pattern Guide](../../advanced/factory-pattern) - Factory contract patterns
- [Deterministic Deployment](../../advanced/deterministic-deploy) - Predictable addresses
- [Gas Optimization](../../guides/gas-optimization) - Reduce deployment costs
