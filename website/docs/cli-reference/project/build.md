---
title: glin-forge build
description: Build and compile ink! smart contracts
---

# glin-forge build

Build and compile your ink! smart contract to WebAssembly.

## Synopsis

```bash
glin-forge build [OPTIONS]
```

## Description

The `build` command compiles your ink! smart contract using `cargo-contract`. It generates WebAssembly bytecode, metadata (ABI), and a contract bundle ready for deployment.

Internally, it runs `cargo contract build` with appropriate flags and displays the output in a user-friendly format.

## Options

### `-p, --path <PATH>`

Path to the contract project directory.

- **Default**: `.` (current directory)
- Must contain a `Cargo.toml` file
- Can be absolute or relative path

### `--release`

Build in release mode with optimizations.

- **Recommended for deployment**
- Smaller WASM size
- Lower gas costs
- Longer compile time

### `--verify`

Verify the contract after building.

Performs additional checks:
- Code hash calculation
- Metadata structure validation
- WASM size analysis
- Constructor and message counting
- Size warnings

### `-h, --help`

Print help information.

## Examples

### Basic Build

Build in debug mode:

```bash
glin-forge build
```

### Release Build

Build with optimizations (recommended):

```bash
glin-forge build --release
```

### Build with Verification

Build and verify the output:

```bash
glin-forge build --release --verify
```

### Build Specific Project

Build a contract in a different directory:

```bash
glin-forge build --path ./contracts/my-token --release
```

## Output

### Success Output

```
Building contract...
   Compiling my-token v0.1.0
   Compiling ink_metadata v5.0.0
   Compiling ink_env v5.0.0
   Compiling ink v5.0.0
    Finished release [optimized] target(s) in 45.2s

✓ Contract built successfully!

Output files:
  WASM: ./target/ink/my_token.wasm (23.4 KB)
  Metadata: ./target/ink/metadata.json
  Bundle: ./target/ink/my_token.contract
```

### With --verify Flag

```
Building contract...
    Finished release [optimized] target(s) in 45.2s

✓ Contract built successfully!

Output files:
  WASM: ./target/ink/my_token.wasm (23.4 KB)
  Metadata: ./target/ink/metadata.json
  Bundle: ./target/ink/my_token.contract

Build verification:
  ✓ Code hash: 0x1234567890abcdef...
  ✓ Constructors: 1 (new)
  ✓ Messages: 6 (transfer, approve, transferFrom, balanceOf, totalSupply, allowance)
  ✓ Events: 2 (Transfer, Approval)
  ✓ WASM size: 23.4 KB (good)

  ℹ Tip: WASM size is optimal for deployment
```

### Size Warnings

If WASM is large:

```
Build verification:
  ...
  ⚠ WASM size: 156.7 KB (large)

  ℹ Tip: Consider optimizing your contract:
    - Remove unused dependencies
    - Use storage efficiently
    - Minimize string literals
    - Run: wasm-opt -Oz contract.wasm -o optimized.wasm
```

### Error Output

```
Building contract...
   Compiling my-token v0.1.0
error[E0425]: cannot find function `transferr` in this scope
  --> lib.rs:42:13
   |
42 |             self.transferr(to, amount)?;
   |             ^^^^^^^^^^ help: a function with a similar name exists: `transfer`

✗ Build failed!

Fix the errors above and try again.
```

## Generated Files

### target/ink/*.wasm

The compiled WebAssembly bytecode.

- Binary format
- Deployed to blockchain
- Size matters (affects gas costs)

**View size:**
```bash
ls -lh target/ink/*.wasm
```

### target/ink/metadata.json

Contract metadata (ABI).

- JSON format
- Describes contract interface
- Used for interaction and type generation

**View metadata:**
```bash
cat target/ink/metadata.json | jq
```

**Structure:**
```json
{
  "contract": {
    "name": "my_token",
    "version": "0.1.0"
  },
  "spec": {
    "constructors": [...],
    "messages": [...],
    "events": [...],
    "types": [...]
  }
}
```

### target/ink/*.contract

Bundle file containing both WASM and metadata.

- Single file for deployment
- Used by cargo-contract and some tools
- GLIN Forge can deploy from WASM + metadata separately

## Build Modes

### Debug Build

```bash
glin-forge build
```

**Characteristics:**
- Faster compilation
- Larger file size
- Debug symbols included
- Good for development and testing

**Use when:**
- Developing locally
- Running tests
- Iterating quickly

### Release Build

```bash
glin-forge build --release
```

**Characteristics:**
- Slower compilation
- Smaller file size
- Optimized for production
- Lower gas costs

**Use when:**
- Deploying to testnet/mainnet
- Final testing before deployment
- Measuring gas costs

### Size Comparison

| Mode | Example Size | Compile Time |
|------|--------------|--------------|
| Debug | 45.6 KB | 30 seconds |
| Release | 23.4 KB | 60 seconds |

## Optimization

### Using wasm-opt

Further optimize WASM after building:

```bash
# Build first
glin-forge build --release

# Then optimize with wasm-opt
wasm-opt -Oz target/ink/my_token.wasm -o target/ink/my_token_optimized.wasm

# Check size reduction
ls -lh target/ink/my_token*.wasm
```

**Flags:**
- `-Oz` - Optimize for size (most aggressive)
- `-O3` - Optimize for performance
- `-O2` - Balanced optimization

### Install wasm-opt

```bash
# Ubuntu/Debian
sudo apt install binaryen

# macOS
brew install binaryen

# Check installation
wasm-opt --version
```

## Build Process

Under the hood, `glin-forge build` performs these steps:

1. **Check cargo-contract**: Ensures `cargo-contract` is installed
2. **Run cargo build**: Executes `cargo contract build [--release]`
3. **Locate outputs**: Finds WASM and metadata in `target/ink/`
4. **Verify (optional)**: Validates build artifacts if `--verify` is used
5. **Display results**: Shows file paths and sizes

## Verification Details

With `--verify`, GLIN Forge checks:

### Code Hash Calculation

```
✓ Code hash: 0x1234...5678
```

The Blake2-256 hash of the WASM bytecode. This uniquely identifies your contract code on-chain.

### Constructor Analysis

```
✓ Constructors: 2 (new, default)
```

Lists all constructor functions for instantiation.

### Message Analysis

```
✓ Messages: 6 (transfer, approve, ...)
```

Lists all callable methods (queries and transactions).

### Event Analysis

```
✓ Events: 2 (Transfer, Approval)
```

Lists all events the contract can emit.

### Size Analysis

```
✓ WASM size: 23.4 KB (good)
```

Categories:
- **< 50 KB**: Good
- **50-100 KB**: Acceptable
- **100-500 KB**: Large (consider optimizing)
- **> 500 KB**: Very large (optimization needed)

## Troubleshooting

### cargo-contract Not Found

**Problem:**
```
✗ Error: cargo-contract not found
  Install it with: cargo install cargo-contract --force
```

**Solution:**
```bash
cargo install cargo-contract --force
```

### Build Errors

**Problem:**
```
error[E0432]: unresolved import `ink::storage::Mapping`
```

**Solution:**
Check your ink! version in `Cargo.toml`:
```toml
[dependencies]
ink = { version = "5.0", default-features = false }
```

Update if needed:
```bash
cargo update
```

### Out of Memory

**Problem:**
```
error: linking with `rust-lld` failed: signal: 9, SIGKILL
```

**Solution:**
Increase available memory or build on a machine with more RAM. Release builds are more memory-intensive.

### Slow Builds

**Problem:**
Builds take several minutes.

**Solution:**
1. Use incremental compilation (default)
2. Build in debug mode for development:
   ```bash
   glin-forge build  # Faster than --release
   ```
3. Use `cargo-contract check` for quick validation:
   ```bash
   cargo contract check
   ```

## Tips

### Clean Builds

Remove previous builds:

```bash
cargo clean
glin-forge build --release
```

### Watch for Changes

Auto-rebuild on file changes (requires `cargo-watch`):

```bash
cargo install cargo-watch
cargo watch -x "contract build"
```

### Parallel Builds

Speed up by allowing parallel compilation:

```bash
# Set in environment or .cargo/config.toml
export CARGO_BUILD_JOBS=4
```

### Cache Dependencies

Dependencies are cached in `target/` and `~/.cargo/`. Don't delete these between builds.

## Integration with CI/CD

### GitHub Actions

```yaml
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

      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: contract-artifacts
          path: target/ink/*
```

## Related Commands

- [`test`](./test) - Run tests before building
- [`deploy`](../deployment/deploy) - Deploy the built contract
- [`verify`](../verification/verify) - Verify on block explorer

## See Also

- [Building Contracts Guide](../../guides/building-contracts) - Comprehensive building guide
- [Gas Optimization](../../guides/gas-optimization) - Reduce WASM size
- [cargo-contract Docs](https://github.com/paritytech/cargo-contract) - Underlying tool
