---
title: Deploying to Testnet
description: Step-by-step guide to deploying smart contracts on GLIN testnet
---

# Deploying to Testnet

Learn how to deploy your smart contracts to GLIN testnet for testing before mainnet launch.

## Prerequisites

Before deploying, ensure you have:

- ✅ Built your contract (`glin-forge build --release`)
- ✅ GLIN Forge installed
- ✅ Testnet account with tokens
- ✅ Contract tested locally

## Overview

The testnet deployment process:

1. **Build** - Compile optimized contract
2. **Verify** - Check build artifacts
3. **Fund** - Ensure account has testnet tokens
4. **Deploy** - Upload and instantiate
5. **Test** - Verify deployment worked
6. **Verify** - Submit to block explorer

## Step-by-Step Guide

### Step 1: Build the Contract

Build in release mode for optimal size:

```bash
glin-forge build --release --verify
```

**Expected output:**
```
✓ Contract built successfully!

Output files:
  WASM: ./target/ink/my_contract.wasm (23.4 KB)
  Metadata: ./target/ink/metadata.json

Build verification:
  ✓ Code hash: 0x1234...
  ✓ WASM size: 23.4 KB (good)
```

### Step 2: Get Testnet Tokens

Check your balance:

```bash
glin-forge balance alice --network testnet
```

If balance is low, request testnet tokens:

**Testnet Faucet:** https://faucet-testnet.glin.network

1. Visit the faucet
2. Enter your address
3. Complete captcha
4. Receive 100 tGLIN

### Step 3: Deploy the Contract

Deploy using the `deploy` command:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"
```

**Breakdown:**
- `--network testnet` - Deploy to testnet
- `--account alice` - Use Alice account
- `--args "..."` - Constructor arguments

### Step 4: Confirm Deployment

Review the deployment details:

```
Deploying contract...

Deployment details:
  Network: testnet
  RPC: wss://testnet.glin.network
  Account: alice (5GrwvaEF...)
  Constructor: new
  Arguments: [1000000, "MyToken", "MTK"]
  Value: 0 GLIN

Gas estimation:
  refTime: 5,000,000,000
  proofSize: 2,000,000

Estimated deployment cost: ~0.005 GLIN

Proceed with deployment? [y/N]:
```

Type `y` and press Enter.

### Step 5: Wait for Finalization

The deployment will finalize:

```
✓ Connected to wss://testnet.glin.network
⠋ Waiting for finalization... (block #123456)

✓ Contract deployed successfully!

Contract info:
  Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Code Hash: 0x1234...
  Block: #123456
  Transaction: 0xabcd...

Explorer:
  https://explorer-testnet.glin.network/contract/5FHneW46...
```

**Save your contract address!**

```bash
export CONTRACT_ADDR="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
```

### Step 6: Test the Deployment

Verify the contract works:

```bash
# Query total supply
glin-forge query $CONTRACT_ADDR totalSupply

# Check deployer's balance
glin-forge query $CONTRACT_ADDR balanceOf 5GrwvaEF...

# Transfer tokens
glin-forge call $CONTRACT_ADDR transfer 5FrLwHLX... 100 \
  --account alice \
  --wait
```

### Step 7: Verify on Explorer

Submit for verification:

```bash
glin-forge verify $CONTRACT_ADDR --network testnet
```

This allows others to view your source code on the block explorer.

## Deployment Strategies

### One-Step Deployment

Quick deployment (upload + instantiate):

```bash
glin-forge deploy --network testnet --account alice
```

**Best for:**
- Single contract deployments
- Development testing
- Quick iterations

### Two-Step Deployment

Upload once, instantiate multiple times:

```bash
# Step 1: Upload code
CODE_HASH=$(glin-forge upload \
  --wasm target/ink/my_contract.wasm \
  --account alice \
  --network testnet)

echo "Code hash: $CODE_HASH"

# Step 2: Instantiate multiple instances
glin-forge instantiate $CODE_HASH \
  --args "1000000" \
  --account alice \
  --network testnet

glin-forge instantiate $CODE_HASH \
  --args "2000000" \
  --account bob \
  --network testnet
```

**Best for:**
- Factory patterns
- Multiple instances of same contract
- Gas optimization (upload once)

## Constructor Arguments

### ERC20 Token

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"
```

Arguments:
1. `1000000` - Initial supply
2. `"MyToken"` - Token name
3. `"MTK"` - Token symbol

### ERC721 NFT

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "MyNFT,MNFT"
```

Arguments:
1. `"MyNFT"` - Collection name
2. `"MNFT"` - Collection symbol

### DAO Contract

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "100,86400"
```

Arguments:
1. `100` - Minimum votes required
2. `86400` - Voting period (seconds)

### Payable Constructor

Send GLIN with deployment:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --value 10
```

## Gas Management

### Default Gas Limits

GLIN Forge uses conservative defaults:

- **refTime**: 5,000,000,000 (5 billion)
- **proofSize**: 2,000,000 (2 million)

### Custom Gas Limits

Override if needed:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --gas-limit 10000000000,3000000
```

### Gas Costs

Typical testnet deployment costs:

| Contract Size | Gas Cost | GLIN Cost |
|--------------|----------|-----------|
| < 25 KB | Low | ~0.001 tGLIN |
| 25-50 KB | Medium | ~0.005 tGLIN |
| > 50 KB | High | ~0.01 tGLIN |

## Deterministic Deployment

Deploy to predictable address using salt:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --salt 0x0000000000000000000000000000000000000000000000000000000000000001
```

**Same code + same salt = same address**

Useful for:
- Upgradeable contracts
- Cross-chain deployments
- Factory patterns

## Troubleshooting

### Insufficient Balance

**Problem:**
```
✗ Error: Insufficient balance
  Required: ~5.0 GLIN
  Available: 0.5 GLIN
```

**Solution:**
Request testnet tokens from faucet:
https://faucet-testnet.glin.network

### Network Connection Failed

**Problem:**
```
✗ Error: Failed to connect to network
  Network: testnet
  RPC: wss://testnet.glin.network
```

**Solutions:**
1. Check internet connection
2. Try again (network might be temporarily down)
3. Use custom RPC endpoint:
   ```bash
   glin-forge deploy --network testnet --account alice
   # If fails, try local node or alternative RPC
   ```

### Deployment Hangs

**Problem:**
Stuck at "Waiting for finalization..."

**Solutions:**
1. Wait longer (can take 10-30 seconds)
2. Check network status
3. Try redeploying

### Constructor Error

**Problem:**
```
✗ Transaction failed!
  Reason: contracts::ContractTrapped
```

**Solution:**
Constructor panicked. Check:
1. Are arguments correct?
2. Does constructor logic panic?
3. Test constructor locally first

### WASM Not Found

**Problem:**
```
✗ Error: WASM file not found
```

**Solution:**
Build first:
```bash
glin-forge build --release
glin-forge deploy --network testnet --account alice
```

## Post-Deployment

### Save Contract Information

Create a deployment record:

```bash
cat > deployment.json <<EOF
{
  "network": "testnet",
  "address": "$CONTRACT_ADDR",
  "deployer": "alice",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "codeHash": "0x1234...",
  "blockNumber": 123456
}
EOF
```

### Test Contract Interaction

Create a test script:

```bash
#!/bin/bash
# test-contract.sh

CONTRACT="5FHneW46..."

echo "Testing contract $CONTRACT..."

# Query total supply
echo "Total supply:"
glin-forge query $CONTRACT totalSupply

# Check balance
echo "Alice balance:"
glin-forge query $CONTRACT balanceOf 5GrwvaEF...

# Transfer tokens
echo "Transferring 100 tokens..."
glin-forge call $CONTRACT transfer 5FrLwHLX... 100 \
  --account alice \
  --yes \
  --wait

echo "Done!"
```

### Generate Frontend Types

```bash
glin-forge typegen \
  --contract $CONTRACT_ADDR \
  --network testnet \
  --output ./frontend/src/contracts \
  --hooks
```

### Monitor Events

Watch contract activity:

```bash
glin-forge watch $CONTRACT_ADDR --follow
```

## Best Practices

### 1. Test Locally First

```bash
# Run all tests before deploying
glin-forge test

# If tests pass, deploy
glin-forge build --release
glin-forge deploy --network testnet --account alice
```

### 2. Use Descriptive Names

```bash
# Good
glin-forge new glin-governance-token --template erc20

# Bad
glin-forge new t1 --template erc20
```

### 3. Verify Immediately

```bash
# Deploy and verify in one session
glin-forge deploy --network testnet --account alice
glin-forge verify $CONTRACT_ADDR --network testnet
```

### 4. Document Deployments

Keep a log of all deployments:

```bash
# deployment-log.md
## Testnet Deployments

### 2025-01-15: MyToken v0.1.0
- Address: 5FHneW46...
- Code Hash: 0x1234...
- Constructor: new(1000000, "MyToken", "MTK")
- Explorer: https://explorer-testnet.glin.network/contract/5FHneW46...
```

### 5. Version Your Contracts

```rust
#[ink::contract]
mod my_contract {
    pub const VERSION: &str = "0.1.0";

    #[ink(message)]
    pub fn version(&self) -> &'static str {
        VERSION
    }
}
```

## Automation

### CI/CD Deployment

```yaml
# .github/workflows/deploy-testnet.yml
name: Deploy to Testnet

on:
  push:
    tags:
      - 'v*'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install dependencies
        run: |
          cargo install cargo-contract --force
          cargo install glin-forge

      - name: Build contract
        run: glin-forge build --release --verify

      - name: Deploy to testnet
        env:
          DEPLOY_SEED: ${{ secrets.TESTNET_DEPLOY_SEED }}
        run: |
          # Import account
          glin-forge account import deployer --seed "$DEPLOY_SEED"

          # Deploy
          glin-forge deploy \
            --network testnet \
            --account deployer \
            --yes \
            > deploy-output.txt

          # Extract address
          CONTRACT_ADDR=$(grep "Address:" deploy-output.txt | awk '{print $2}')
          echo "CONTRACT_ADDR=$CONTRACT_ADDR" >> $GITHUB_ENV

      - name: Verify contract
        run: |
          glin-forge verify $CONTRACT_ADDR --network testnet
```

### Deployment Script

```bash
#!/bin/bash
# deploy.sh

set -e

echo "Building contract..."
glin-forge build --release --verify

echo "Deploying to testnet..."
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "$@" \
  --yes \
  > deploy-output.txt

# Extract contract address
CONTRACT_ADDR=$(grep "Address:" deploy-output.txt | awk '{print $2}')

echo "Contract deployed: $CONTRACT_ADDR"

# Save to file
echo $CONTRACT_ADDR > .contract-address

# Verify
echo "Verifying contract..."
glin-forge verify $CONTRACT_ADDR --network testnet

echo "Deployment complete!"
echo "Contract address: $CONTRACT_ADDR"
echo "Explorer: https://explorer-testnet.glin.network/contract/$CONTRACT_ADDR"
```

Usage:

```bash
./deploy.sh "1000000,MyToken,MTK"
```

## Next Steps

After deploying to testnet:

1. [Interact with Contract](./contract-interaction) - Test all functionality
2. [Generate Types](./frontend-integration) - Create TypeScript types
3. [Build Frontend](../examples/frontend-dapp) - Create a dApp
4. [Deploy to Mainnet](./deploying-mainnet) - Production deployment

## Resources

- [Deploy Command Reference](../cli-reference/deployment/deploy)
- [Query Guide](./contract-interaction)
- [Verification Guide](../advanced/contract-verification)
- [Testnet Faucet](https://faucet-testnet.glin.network)
- [Testnet Explorer](https://explorer-testnet.glin.network)
