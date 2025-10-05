---
title: Prerequisites
description: Set up your development environment for GLIN Forge
---

# Prerequisites

Before you start building smart contracts with GLIN Forge, you need to set up your development environment. This guide will walk you through installing all the required tools.

## Required Software

### 1. Rust Toolchain

GLIN Forge and ink! smart contracts are written in Rust. You'll need the Rust compiler and toolchain.

#### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and choose the default installation.

#### Verify Installation

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

#### Update Rust (if already installed)

```bash
rustup update stable
```

**Minimum Version:** Rust 1.70 or later

### 2. cargo-contract

cargo-contract is the official tool for building ink! smart contracts. GLIN Forge uses it under the hood.

#### Install cargo-contract

```bash
cargo install cargo-contract --force
```

This may take a few minutes to compile.

#### Verify Installation

```bash
cargo contract --version
```

You should see:
```
cargo-contract-contract 4.0.0
```

**Minimum Version:** cargo-contract 3.0 or later

### 3. GLIN Forge

Now install GLIN Forge itself:

```bash
cargo install glin-forge
```

#### Verify Installation

```bash
glin-forge --version
```

Expected output:
```
glin-forge 0.1.0
```

## Optional but Recommended

### binaryen

binaryen provides optimized WebAssembly builds, reducing contract size and gas costs.

#### Install on Linux

```bash
# Ubuntu/Debian
sudo apt install binaryen

# Arch Linux
sudo pacman -S binaryen
```

#### Install on macOS

```bash
brew install binaryen
```

#### Install on Windows (WSL2)

```bash
sudo apt install binaryen
```

#### Verify Installation

```bash
wasm-opt --version
```

### Node.js and npm

Required if you plan to use the TypeScript code generation for frontend development.

#### Install Node.js

**Using nvm (recommended):**
```bash
# Install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Install Node.js LTS
nvm install --lts
nvm use --lts
```

**Direct installation:**
- Download from [nodejs.org](https://nodejs.org/)

#### Verify Installation

```bash
node --version
npm --version
```

**Minimum Version:** Node.js 16 or later

## System Requirements

### Hardware

- **CPU:** Modern x86_64 or ARM64 processor
- **RAM:**
  - 4GB minimum for basic development
  - 8GB+ recommended for building contracts
- **Disk Space:**
  - 2GB for Rust toolchain
  - 1GB for cargo-contract and dependencies
  - Additional space for your projects

### Operating Systems

GLIN Forge supports:

- **Linux:** Ubuntu 20.04+, Debian 11+, Fedora 35+, Arch Linux
- **macOS:** 10.15 (Catalina) or later (Intel and Apple Silicon)
- **Windows:** Windows 10/11 via WSL2 (recommended)

### Internet Connection

Required for:
- Downloading dependencies during installation
- Deploying to testnet/mainnet
- Querying blockchain state
- Contract verification

## Verification Checklist

After installation, verify everything is working:

```bash
# Check Rust
rustc --version
cargo --version

# Check cargo-contract
cargo contract --version

# Check GLIN Forge
glin-forge --version

# Optional: Check binaryen
wasm-opt --version

# Optional: Check Node.js
node --version
npm --version
```

### Expected Output

If everything is installed correctly, you should see version numbers for each tool:

```bash
$ rustc --version
rustc 1.75.0 (82e1608df 2023-12-21)

$ cargo --version
cargo 1.75.0 (1d8b05cdd 2023-11-20)

$ cargo contract --version
cargo-contract-contract 4.0.0

$ glin-forge --version
glin-forge 0.1.0

$ wasm-opt --version
wasm-opt version 112

$ node --version
v20.10.0

$ npm --version
10.2.3
```

## Common Installation Issues

### Problem: cargo-contract build fails

**Solution:**
```bash
# Install build dependencies
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# macOS (install Xcode Command Line Tools)
xcode-select --install
```

### Problem: Rust version too old

**Solution:**
```bash
rustup update stable
rustup default stable
```

### Problem: cargo install fails with permission error

**Solution:**
Don't use `sudo` with cargo commands. Cargo installs to `~/.cargo/bin/` by default.

Ensure this directory is in your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Problem: wasm-opt not found

**Solution:**
This is optional but recommended. Install binaryen:
```bash
# Linux
sudo apt install binaryen

# macOS
brew install binaryen
```

## Environment Setup

### Shell Configuration

Add these lines to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
# Rust and Cargo
export PATH="$HOME/.cargo/bin:$PATH"

# Optional: Set default network
export GLIN_NETWORK="testnet"
```

Then reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### IDE Setup

#### Visual Studio Code

Recommended extensions:
- **rust-analyzer** - Rust language support
- **Even Better TOML** - TOML syntax highlighting
- **ink! Language Support** - ink! smart contract support

```bash
# Install extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension tamasfe.even-better-toml
code --install-extension ink-analyzer.ink-analyzer
```

#### IntelliJ IDEA / CLion

- Install the **Rust** plugin from JetBrains Marketplace

## Testing Your Setup

Let's verify everything works by creating a test project:

```bash
# Create a test contract
glin-forge new test-contract --template erc20

# Navigate to project
cd test-contract

# Build the contract
glin-forge build

# Run tests
glin-forge test
```

If this completes without errors, your environment is ready!

### Expected Build Output

```
Building contract...
✓ Contract built successfully!

Output files:
  WASM: ./target/ink/test_contract.wasm
  Metadata: ./target/ink/metadata.json
  Bundle: ./target/ink/test_contract.contract
```

## Next Steps

Now that your environment is set up:

1. [Quick Start Guide](./quick-start) - Deploy your first contract in 5 minutes
2. [First Contract Tutorial](./first-contract) - Build a contract from scratch
3. [CLI Reference](../cli-reference/overview) - Learn all available commands

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust programming
- [ink! Documentation](https://use.ink/) - ink! smart contract framework
- [cargo-contract Guide](https://github.com/paritytech/cargo-contract) - Contract tooling
- [GLIN Network Docs](https://docs.glin.ai/) - Network documentation

## Getting Help

If you encounter issues:

1. Check [Troubleshooting Guide](../troubleshooting/common-errors)
2. Join our [Discord](https://discord.gg/glin)
3. Open an issue on [GitHub](https://github.com/glin-ai/glin-forge/issues)
4. Ask on our [Forum](https://forum.glin.ai/)
