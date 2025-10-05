---
title: Common Errors
description: Solutions to common issues when using GLIN Forge
---

# Common Errors & Solutions

Quick solutions to common issues you might encounter while using GLIN Forge.

## Installation Issues

### cargo-contract Not Found

**Error:**
```
Error: cargo-contract not found
Install it with: cargo install cargo-contract --force
```

**Solution:**
```bash
cargo install cargo-contract --force
```

**Verification:**
```bash
cargo contract --version
```

### GLIN Forge Not in PATH

**Error:**
```
command not found: glin-forge
```

**Solution:**

Add Cargo's bin directory to your PATH:

```bash
# For bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Verification:**
```bash
glin-forge --version
```

### Rust Version Too Old

**Error:**
```
error: package `glin-forge` requires rustc 1.70 or later
```

**Solution:**
```bash
rustup update stable
rustup default stable
```

**Verification:**
```bash
rustc --version
```

## Build Errors

### Missing Dependencies

**Error:**
```
error[E0432]: unresolved import `ink::storage::Mapping`
```

**Solution:**

Check your `Cargo.toml` has correct ink! version:

```toml
[dependencies]
ink = { version = "5.0", default-features = false }
```

Then update:
```bash
cargo update
glin-forge build
```

### Build Fails After Update

**Error:**
```
error: failed to compile
```

**Solution:**

Clean and rebuild:

```bash
cargo clean
glin-forge build --release
```

### Out of Memory During Build

**Error:**
```
error: linking with `rust-lld` failed: signal: 9, SIGKILL
```

**Solution:**

1. Close other applications
2. Build in debug mode first (uses less memory):
   ```bash
   glin-forge build
   ```
3. Then try release mode:
   ```bash
   glin-forge build --release
   ```

### WASM File Not Found

**Error:**
```
Error: WASM file not found at ./target/ink/*.wasm
```

**Solution:**

Build the contract first:

```bash
glin-forge build --release
```

Then proceed with deployment:

```bash
glin-forge deploy --network testnet --account alice
```

## Deployment Errors

### Insufficient Balance

**Error:**
```
Error: Insufficient balance
  Required: ~5.0 GLIN
  Available: 0.5 GLIN
```

**Solution:**

Request testnet tokens from the faucet:

1. Get your address:
   ```bash
   glin-forge account show alice
   ```

2. Visit faucet:
   https://faucet-testnet.glin.network

3. Enter your address and request tokens

4. Verify balance:
   ```bash
   glin-forge balance alice --network testnet
   ```

### Network Connection Failed

**Error:**
```
Error: Failed to connect to network
  Network: testnet
  RPC: wss://testnet.glin.network
```

**Solution:**

1. Check your internet connection

2. Verify network status (might be temporarily down)

3. Try again after a few minutes:
   ```bash
   glin-forge deploy --network testnet --account alice
   ```

### Contract Deployment Hangs

**Problem:**
Stuck at "Waiting for finalization..."

**Solution:**

1. Wait up to 30 seconds (normal finalization time: 6-12 seconds)

2. If still hanging, cancel (Ctrl+C) and try again:
   ```bash
   glin-forge deploy --network testnet --account alice
   ```

3. Check network status

### Constructor Error

**Error:**
```
Transaction failed!
  Reason: contracts::ContractTrapped
```

**Solution:**

Constructor panicked. Check:

1. Are arguments correct type and order?
   ```bash
   # Correct
   glin-forge deploy --args "1000000,MyToken,MTK"

   # Wrong (missing argument)
   glin-forge deploy --args "1000000,MyToken"
   ```

2. Does constructor logic panic?
   - Review constructor code
   - Check for division by zero
   - Check for overflows

3. Test locally first:
   ```bash
   glin-forge test
   ```

### Invalid Arguments

**Error:**
```
Error: Invalid constructor arguments
  Expected: (u128, String, String)
  Provided: (1000000)
```

**Solution:**

Provide all required arguments:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"
```

Check metadata for constructor signature:

```bash
cat target/ink/metadata.json | jq '.spec.constructors'
```

## Interaction Errors

### Method Not Found

**Error:**
```
Error: Method 'balanceof' not found
  Available methods: balanceOf, totalSupply, allowance
```

**Solution:**

Method names are case-sensitive. Use correct capitalization:

```bash
# Correct
glin-forge query $CONTRACT balanceOf $ACCOUNT

# Wrong
glin-forge query $CONTRACT balanceof $ACCOUNT
```

List available methods:

```bash
cat target/ink/metadata.json | jq '.spec.messages[].label'
```

### Query Returns Error

**Error:**
```
Error: Failed to decode result
```

**Solution:**

1. Metadata might be outdated. Rebuild:
   ```bash
   glin-forge build --release
   glin-forge query $CONTRACT method
   ```

2. Verify contract address is correct:
   ```bash
   # Check on explorer
   https://explorer-testnet.glin.network/contract/$CONTRACT
   ```

### Transaction Reverted

**Error:**
```
Transaction failed!
  Status: Failed
  Reason: InsufficientBalance
```

**Solution:**

Contract rejected the transaction. Common reasons:

1. **Insufficient balance:**
   ```bash
   # Check balance first
   glin-forge query $CONTRACT balanceOf $ACCOUNT

   # Then transfer
   glin-forge call $CONTRACT transfer $RECIPIENT 100 --account alice
   ```

2. **Not authorized:**
   ```bash
   # Error: NotOwner
   # Use the owner account
   glin-forge call $CONTRACT adminMethod --account owner
   ```

3. **Contract paused:**
   ```bash
   # Contract is paused
   # Unpause first (if you're owner)
   glin-forge call $CONTRACT unpause --account owner
   ```

### Out of Gas

**Error:**
```
Transaction failed!
  Reason: contracts::OutOfGas
```

**Solution:**

Increase gas limit:

```bash
glin-forge call $CONTRACT method \
  --account alice \
  --gas-limit 5000000000,2000000
```

## Type Generation Errors

### Metadata Not Found

**Error:**
```
Error: Metadata file not found at ./target/ink/metadata.json
```

**Solution:**

Build the contract first:

```bash
glin-forge build --release
glin-forge typegen --output ./types --hooks
```

### Invalid JSON

**Error:**
```
Error: Failed to parse metadata
  Invalid JSON format
```

**Solution:**

Metadata file is corrupted. Rebuild:

```bash
rm -rf target/
glin-forge build --release
```

### TypeScript Errors in Frontend

**Problem:**
Generated types cause TypeScript compilation errors.

**Solution:**

1. Ensure SDK is installed:
   ```bash
   npm install @glin-ai/sdk @glin-ai/sdk-react
   ```

2. Regenerate types:
   ```bash
   glin-forge typegen --output ./src/contracts --hooks
   ```

3. Check TypeScript version (need >= 4.5):
   ```bash
   npx tsc --version
   ```

## Account Issues

### Account Not Found

**Error:**
```
Error: Account 'myaccount' not found
```

**Solution:**

Create or import the account:

```bash
# Generate new account
glin-forge account generate myaccount

# Or import existing
glin-forge account import myaccount --seed "word1 word2 ..."
```

List available accounts:

```bash
glin-forge account list
```

### Permission Denied

**Error:**
```
Error: Permission denied when accessing keystore
```

**Solution:**

Fix file permissions:

```bash
chmod 600 ~/.glin/keystore/*
```

## Verification Errors

### Code Hash Mismatch

**Error:**
```
Error: Code hash mismatch
  Expected: 0x1234...
  Found: 0x5678...
```

**Solution:**

1. Ensure you're verifying the correct WASM file:
   ```bash
   glin-forge verify $CONTRACT \
     --wasm ./target/ink/my_contract.wasm \
     --metadata ./target/ink/metadata.json \
     --network testnet
   ```

2. Rebuild and verify again:
   ```bash
   glin-forge build --release
   glin-forge verify $CONTRACT --network testnet
   ```

### Contract Not Found On-Chain

**Error:**
```
Error: Contract not found at address
```

**Solution:**

1. Verify the address is correct

2. Check you're on the right network:
   ```bash
   # If deployed to testnet
   glin-forge verify $CONTRACT --network testnet

   # Not mainnet
   glin-forge verify $CONTRACT --network mainnet
   ```

3. Check contract on explorer:
   ```
   https://explorer-testnet.glin.network/contract/$CONTRACT
   ```

## Performance Issues

### Slow Builds

**Problem:**
Builds take several minutes.

**Solution:**

1. Use incremental builds (default):
   ```bash
   glin-forge build  # Fast second time
   ```

2. Avoid `cargo clean` unless necessary

3. Use debug mode during development:
   ```bash
   glin-forge build  # Faster than --release
   ```

4. Enable parallel compilation:
   ```bash
   export CARGO_BUILD_JOBS=4
   glin-forge build
   ```

### Large WASM Files

**Problem:**
```
⚠ WASM size: 156.7 KB (large)
```

**Solution:**

1. Use release mode:
   ```bash
   glin-forge build --release
   ```

2. Apply wasm-opt:
   ```bash
   wasm-opt -Oz target/ink/contract.wasm -o optimized.wasm
   ```

3. Remove unused dependencies:
   ```bash
   cargo tree  # Review dependencies
   # Remove unused from Cargo.toml
   ```

4. Optimize code:
   - Use `&str` instead of `String`
   - Minimize storage
   - Remove debug prints

## Getting Further Help

### Debug Mode

Enable verbose output:

```bash
glin-forge deploy --verbose --network testnet --account alice
```

### Check Versions

Ensure all tools are up-to-date:

```bash
rustc --version
cargo --version
cargo contract --version
glin-forge --version
```

### Community Support

- **Discord:** https://discord.gg/glin
- **GitHub Issues:** https://github.com/glin-ai/glin-forge/issues
- **Forum:** https://forum.glin.ai/
- **Documentation:** https://docs.glin.ai/forge

### Report a Bug

When reporting issues, include:

1. **GLIN Forge version:**
   ```bash
   glin-forge --version
   ```

2. **Full error message:**
   ```bash
   glin-forge command 2>&1 | tee error.log
   ```

3. **Environment:**
   - OS: Ubuntu 22.04 / macOS 13.0 / Windows 11
   - Rust version
   - cargo-contract version

4. **Reproduction steps:**
   - What you were trying to do
   - Command you ran
   - Expected vs actual behavior

## Quick Reference

### Most Common Fixes

| Error | Quick Fix |
|-------|-----------|
| cargo-contract not found | `cargo install cargo-contract --force` |
| WASM not found | `glin-forge build --release` |
| Insufficient balance | Visit faucet: https://faucet-testnet.glin.network |
| Method not found | Check case sensitivity |
| Network connection failed | Check internet, try again |
| Build fails | `cargo clean && glin-forge build` |
| Metadata not found | `glin-forge build --release` |

### Emergency Commands

```bash
# Start fresh
cargo clean
rm -rf target/
glin-forge build --release

# Reset account
glin-forge account import alice --seed "bottom drive obey lake curtain smoke basket hold race lonely fit walk"

# Check everything
glin-forge --version
cargo contract --version
rustc --version
```

## Related Resources

- [CLI Reference](../cli-reference/overview) - Command documentation
- [Building Guide](../guides/building-contracts) - Build best practices
- [Deployment Guide](../guides/deploying-testnet) - Deployment help
- [Discord Community](https://discord.gg/glin) - Live help
