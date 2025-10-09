# glin-forge

Hardhat-style development environment for GLIN Network smart contracts.

## Quick Start

No installation required! Use npx:

```bash
npx glin-forge init my-contract
cd my-contract
npm install
npx glin-forge build
```

## Installation Options

### Option 1: npx (Recommended)

Use directly without installation:

```bash
npx glin-forge init my-project
```

### Option 2: Global npm install

Install globally:

```bash
npm install -g glin-forge
glin-forge init my-project
```

### Option 3: Cargo install

If you have Rust installed:

```bash
cargo install glin-forge
glin-forge init my-project
```

## Usage

```bash
# Initialize a new project
npx glin-forge init my-contract

# Build contracts
npx glin-forge build

# Run tests
npx glin-forge test

# Deploy to network
npx glin-forge deploy --network testnet

# Start interactive console
npx glin-forge console

# Analyze contract code
npx glin-forge analyze

# And more...
npx glin-forge --help
```

## How it Works

This npm package downloads a pre-built native binary for your platform:
- **macOS**: x64 and arm64 (Apple Silicon)
- **Linux**: x64 and arm64
- **Windows**: x64

If a pre-built binary isn't available, it will attempt to compile from source using Cargo (requires Rust).

## Requirements

- Node.js 16+ (for npx/npm)
- No Rust/Cargo required (pre-built binaries provided)

## Features

- 🚀 Hardhat-style workflow
- 📦 TypeScript SDK and testing utilities
- 🎨 Interactive project initialization
- 🧪 Comprehensive testing tools
- 🔍 Built-in security analyzer
- 💻 Interactive console (REPL)
- 🌐 Multi-network support
- 📊 Full-stack dApp templates

## Documentation

Visit [docs.glin.network/glin-forge](https://docs.glin.network/glin-forge) for complete documentation.

## Examples

- [Token dApp](../../examples/token-dapp)
- [NFT Marketplace](../../examples/nft-marketplace)
- [DAO Governance](../../examples/dao-governance)

## License

Apache-2.0

## Support

- [GitHub Issues](https://github.com/glin-ai/glin-forge/issues)
- [Discord](https://discord.gg/glin)
- [Documentation](https://docs.glin.network)
