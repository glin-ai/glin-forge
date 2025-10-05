---
title: Building Contracts
description: Complete guide to building ink! smart contracts with GLIN Forge
---

# Building Smart Contracts

This guide covers everything you need to know about building ink! smart contracts with GLIN Forge.

## Overview

Building a contract transforms your Rust source code into WebAssembly bytecode that can run on the GLIN Network blockchain. GLIN Forge uses `cargo-contract` under the hood to compile your contracts.

## Basic Build

### Quick Build (Debug Mode)

For development and testing:

```bash
glin-forge build
```

**Characteristics:**
- Fast compilation (~30 seconds)
- Larger WASM file
- Includes debug symbols
- Good for local testing

### Release Build (Production)

For deployment:

```bash
glin-forge build --release
```

**Characteristics:**
- Slower compilation (~60 seconds)
- Optimized WASM file (50% smaller)
- No debug symbols
- Required for mainnet deployment

## Build Output

### Generated Files

After building, you'll find these files in `target/ink/`:

```
target/ink/
├── my_contract.wasm        # WebAssembly bytecode
├── my_contract.contract    # Bundle (WASM + metadata)
└── metadata.json           # Contract ABI
```

### WASM File

The compiled contract bytecode:

```bash
ls -lh target/ink/*.wasm
```

**Size targets:**
- Excellent: < 25 KB
- Good: 25-50 KB
- Acceptable: 50-100 KB
- Large: > 100 KB (needs optimization)

### Metadata (ABI)

Contract interface definition:

```bash
cat target/ink/metadata.json | jq '.spec.messages[].label'
```

Contains:
- Constructor signatures
- Message (method) signatures
- Event definitions
- Type information

## Build Verification

### Verify Build Output

Add `--verify` to validate the build:

```bash
glin-forge build --release --verify
```

**Checks performed:**
- Code hash calculation
- Metadata structure validation
- Constructor counting
- Message counting
- WASM size analysis

### Example Output

```
Build verification:
  ✓ Code hash: 0x1234567890abcdef...
  ✓ Constructors: 1 (new)
  ✓ Messages: 6 (transfer, approve, transferFrom, balanceOf, totalSupply, allowance)
  ✓ Events: 2 (Transfer, Approval)
  ✓ WASM size: 23.4 KB (good)
```

## Optimization

### Level 1: Release Mode

Always use `--release` for deployment:

```bash
glin-forge build --release
```

**Size reduction:** ~50%

### Level 2: wasm-opt

Further optimize with Binaryen's `wasm-opt`:

```bash
# Install binaryen
sudo apt install binaryen  # Ubuntu/Debian
brew install binaryen      # macOS

# Build first
glin-forge build --release

# Then optimize
wasm-opt -Oz target/ink/my_contract.wasm -o target/ink/my_contract_optimized.wasm
```

**Optimization levels:**
- `-O`: Basic optimization
- `-O2`: Default optimization
- `-O3`: Aggressive optimization
- `-Oz`: Optimize for size (recommended)

**Size reduction:** Additional 10-20%

### Level 3: Code Optimization

Optimize your Rust code:

**Remove unused features:**
```toml
[dependencies]
ink = { version = "5.0", default-features = false }
# Don't include unnecessary features
```

**Use appropriate types:**
```rust
// Instead of
pub fn get_value(&self) -> String {
    self.value.clone()
}

// Use
pub fn get_value(&self) -> &str {
    &self.value
}
```

**Minimize storage:**
```rust
// Instead of storing full strings
value: String

// Use fixed-size arrays when possible
value: [u8; 32]
```

## Build Modes Comparison

| Mode | Build Time | WASM Size | Use Case |
|------|-----------|-----------|----------|
| Debug | 30s | 45 KB | Development, local testing |
| Release | 60s | 23 KB | Deployment, testnet |
| Release + wasm-opt | 70s | 19 KB | Mainnet, production |

## Common Build Scenarios

### Development Workflow

Fast iteration during development:

```bash
# Quick build for testing
glin-forge build

# Run tests
glin-forge test

# If tests pass, build for deployment
glin-forge build --release --verify
```

### Pre-Deployment

Before deploying to testnet or mainnet:

```bash
# Build with optimization
glin-forge build --release --verify

# Further optimize
wasm-opt -Oz target/ink/contract.wasm -o target/ink/contract_opt.wasm

# Check size
ls -lh target/ink/*.wasm

# Deploy
glin-forge deploy --wasm target/ink/contract_opt.wasm --network testnet --account alice
```

### CI/CD Pipeline

```yaml
# .github/workflows/build.yml
name: Build Contract

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install cargo-contract
        run: cargo install cargo-contract --force

      - name: Install GLIN Forge
        run: cargo install glin-forge

      - name: Build Contract
        run: glin-forge build --release --verify

      - name: Check WASM size
        run: |
          SIZE=$(stat -f%z target/ink/*.wasm)
          if [ $SIZE -gt 102400 ]; then
            echo "::warning::WASM size is large: $SIZE bytes"
          fi

      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: contract-artifacts
          path: target/ink/*
```

## Build Configuration

### Cargo.toml Settings

Optimize your `Cargo.toml`:

```toml
[profile.release]
panic = "abort"           # Reduce binary size
lto = true               # Link-time optimization
codegen-units = 1        # Better optimization
opt-level = "z"          # Optimize for size
strip = true             # Strip symbols
```

### Feature Flags

Control features during build:

```toml
[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
```

Build without default features:

```bash
cargo contract build --no-default-features
```

## Troubleshooting Builds

### cargo-contract Not Found

**Error:**
```
Error: cargo-contract not found
```

**Solution:**
```bash
cargo install cargo-contract --force
```

### Build Fails

**Error:**
```
error[E0425]: cannot find function `transfer` in this scope
```

**Solution:**
1. Fix the compilation error in your code
2. Check function names and imports
3. Ensure all dependencies are correct

### Out of Memory

**Error:**
```
error: linking with `rust-lld` failed: signal: 9, SIGKILL
```

**Solution:**
1. Close other applications
2. Build in debug mode first
3. Increase swap space
4. Use a machine with more RAM

### Slow Builds

**Problem:**
Builds take too long.

**Solutions:**

1. **Use incremental builds** (default):
   ```bash
   # Subsequent builds are faster
   glin-forge build
   glin-forge build  # Much faster second time
   ```

2. **Parallel compilation:**
   ```bash
   export CARGO_BUILD_JOBS=4
   glin-forge build
   ```

3. **Clean only when necessary:**
   ```bash
   # Avoid running cargo clean unless needed
   # cargo clean
   glin-forge build
   ```

### Large WASM Files

**Problem:**
```
⚠ WASM size: 156.7 KB (large)
```

**Solutions:**

1. **Use release mode:**
   ```bash
   glin-forge build --release
   ```

2. **Apply wasm-opt:**
   ```bash
   wasm-opt -Oz target/ink/contract.wasm -o optimized.wasm
   ```

3. **Review dependencies:**
   ```bash
   # Check what's included
   cargo tree

   # Remove unused dependencies from Cargo.toml
   ```

4. **Optimize code:**
   - Remove unused imports
   - Use `&str` instead of `String` where possible
   - Minimize string literals
   - Use compact data structures

## Build Artifacts

### Understanding Code Hash

The code hash uniquely identifies your contract code:

```
Code hash: 0x1234567890abcdef...
```

**Properties:**
- Blake2-256 hash of WASM bytecode
- Same code = same hash
- Used for verification
- Required for instantiation

### Metadata Structure

The metadata JSON contains:

```json
{
  "contract": {
    "name": "my_contract",
    "version": "0.1.0",
    "authors": ["Your Name"]
  },
  "spec": {
    "constructors": [
      {
        "label": "new",
        "selector": "0x9bae9d5e",
        "args": [...]
      }
    ],
    "messages": [...],
    "events": [...]
  },
  "storage": {...},
  "types": [...]
}
```

### Contract Bundle

The `.contract` file combines WASM and metadata:

```bash
# View bundle structure
unzip -l target/ink/my_contract.contract
```

Contains:
- `my_contract.wasm`
- `metadata.json`

## Best Practices

### 1. Always Build in Release for Deployment

```bash
# Development
glin-forge build

# Deployment
glin-forge build --release --verify
```

### 2. Verify Before Deploying

```bash
glin-forge build --release --verify
```

Review the output to ensure everything looks correct.

### 3. Track WASM Size

Monitor contract size over time:

```bash
# Log size after each build
echo "$(date): $(stat -f%z target/ink/*.wasm) bytes" >> wasm_size.log
```

### 4. Use Version Control

Commit metadata with your code:

```bash
git add target/ink/metadata.json
git commit -m "Update contract metadata"
```

### 5. Document Build Requirements

In your README:

```markdown
## Building

Requirements:
- Rust 1.70+
- cargo-contract 3.0+
- glin-forge 0.1.0+

Build command:
\`\`\`bash
glin-forge build --release
\`\`\`
```

## Next Steps

After building your contract:

1. [Run Tests](../cli-reference/project/test) - Validate functionality
2. [Deploy to Testnet](./deploying-testnet) - Test on-chain
3. [Generate Types](./frontend-integration) - Create TypeScript types
4. [Optimize Gas](./gas-optimization) - Reduce deployment costs

## Related Topics

- [Building Command Reference](../cli-reference/project/build)
- [Testing Guide](./testing-strategies)
- [Gas Optimization](./gas-optimization)
- [Deploying to Testnet](./deploying-testnet)
