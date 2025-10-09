# glin-forge Installation Guide

## 📦 Complete Package Structure

glin-forge now consists of **4 packages** that work together:

```
glin-forge (monorepo)
│
├── 1️⃣ glin-forge (npm) - CLI Wrapper Package
│   └── Downloads and runs the Rust binary
│
├── 2️⃣ glin-forge (cargo) - Rust Binary
│   └── The actual CLI implementation
│
├── 3️⃣ @glin-forge/sdk (npm) - TypeScript SDK
│   └── For deployment scripts and contract interaction
│
└── 4️⃣ @glin-forge/testing (npm) - Testing Utilities
    └── For writing contract tests
```

## 🚀 Installation Methods

### Method 1: npx (Recommended - No Installation!) ⭐

Just like Hardhat, use directly without installing anything:

```bash
# Initialize project
npx glin-forge init my-contract

# Use in project
cd my-contract
npx glin-forge build
npx glin-forge deploy
```

**How it works:**
1. `npx` downloads the `glin-forge` npm package
2. The package's post-install script downloads the appropriate Rust binary for your OS
3. The CLI wrapper executes the binary

**Pros:**
- ✅ No installation required
- ✅ No Rust/Cargo needed
- ✅ Familiar to JavaScript developers
- ✅ Always uses latest version

**Cons:**
- ⚠️ Slower first run (downloads binary)
- ⚠️ Requires internet connection for first use

### Method 2: Global npm Install

Install globally for repeated use:

```bash
# Install globally
npm install -g glin-forge

# Use anywhere
glin-forge init my-contract
glin-forge build
```

**Pros:**
- ✅ Fast startup after installation
- ✅ No Rust/Cargo needed
- ✅ Familiar to JavaScript developers

**Cons:**
- ⚠️ Requires installation step
- ⚠️ Larger disk usage

### Method 3: Cargo Install (For Rust Developers)

Install the native Rust binary directly:

```bash
# Install from crates.io
cargo install glin-forge

# Use anywhere
glin-forge init my-contract
glin-forge build
```

**Pros:**
- ✅ Fastest execution (native binary)
- ✅ Smallest download size
- ✅ No npm overhead

**Cons:**
- ⚠️ Requires Rust/Cargo installed
- ⚠️ Longer compile time on first install

### Method 4: Build from Source

For contributors or custom builds:

```bash
# Clone repository
git clone https://github.com/glin-ai/glin-forge
cd glin-forge

# Build
cargo build --release

# Use
./target/release/glin-forge init my-contract
```

## 🔄 Comparison with Hardhat

| Feature | Hardhat | glin-forge |
|---------|---------|------------|
| **Installation** | `npx hardhat init` | `npx glin-forge init` ✓ |
| **Language** | JavaScript/TypeScript | Rust (CLI) + TypeScript (SDK) |
| **SDK Package** | `hardhat` | `@glin-forge/sdk` |
| **Testing Tools** | `@nomicfoundation/hardhat-toolbox` | `@glin-forge/testing` |
| **Config File** | `hardhat.config.ts` | `glinforge.config.ts` |
| **Deployment Scripts** | TypeScript | TypeScript ✓ |
| **Test Files** | TypeScript | TypeScript ✓ |

## 📋 What Gets Installed

### When using npm/npx:

```
node_modules/
├── glin-forge/                    # npm wrapper package
│   ├── bin/
│   │   └── glin-forge             # Native binary (downloaded)
│   └── scripts/
│       └── download-binary.js     # Download script
│
├── @glin-forge/sdk/               # TypeScript SDK
│   └── dist/
│       ├── config.js
│       ├── deployment.js
│       └── ...
│
└── @glin-forge/testing/           # Testing utilities
    └── dist/
        ├── accounts.js
        ├── events.js
        └── ...
```

### When using cargo:

```
~/.cargo/bin/
└── glin-forge                     # Native binary only
```

## 🎯 Recommended Setup

**For JavaScript/TypeScript developers:**
```bash
npx glin-forge init my-project
cd my-project
npm install
```

**For Rust developers:**
```bash
cargo install glin-forge
glin-forge init my-project
cd my-project
npm install  # Still need npm for SDK packages
```

## 📦 Binary Distribution

Pre-built binaries are provided for:

| Platform | Architecture | Filename |
|----------|-------------|----------|
| macOS | x86_64 (Intel) | `glin-forge-{version}-macos-x86_64.tar.gz` |
| macOS | aarch64 (Apple Silicon) | `glin-forge-{version}-macos-aarch64.tar.gz` |
| Linux | x86_64 | `glin-forge-{version}-linux-x86_64.tar.gz` |
| Linux | aarch64 (ARM) | `glin-forge-{version}-linux-aarch64.tar.gz` |
| Windows | x86_64 | `glin-forge-{version}-windows-x86_64.tar.gz` |

Binaries are hosted on GitHub Releases.

## 🔧 Troubleshooting

### Binary download fails

If the binary download fails, the npm package will attempt to compile from source:

```bash
# Ensure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Retry installation
npm install glin-forge --force
```

### Unsupported platform

For unsupported platforms, compile from source:

```bash
git clone https://github.com/glin-ai/glin-forge
cd glin-forge
cargo build --release
sudo cp target/release/glin-forge /usr/local/bin/
```

### Permission errors (Linux/macOS)

If you get permission errors:

```bash
sudo npm install -g glin-forge
# or
npm install -g glin-forge --unsafe-perm
```

## 🎓 User Journey

### First-Time User (JavaScript Developer)

```bash
# Step 1: Initialize project (no installation needed!)
npx glin-forge init my-token
# ✓ Downloads npm wrapper
# ✓ Downloads Rust binary
# ✓ Creates project

# Step 2: Install dependencies
cd my-token
npm install
# ✓ Installs @glin-forge/sdk
# ✓ Installs @glin-forge/testing

# Step 3: Build contract
npx glin-forge build
# ✓ Compiles Rust contract

# Step 4: Run tests
npx glin-forge test
# ✓ Executes TypeScript tests

# Step 5: Deploy
npx glin-forge deploy
# ✓ Deploys to network
```

### First-Time User (Rust Developer)

```bash
# Step 1: Install CLI
cargo install glin-forge

# Step 2: Initialize project
glin-forge init my-token

# Step 3: Install npm dependencies
cd my-token
npm install

# Step 4: Build, test, deploy
glin-forge build
glin-forge test
glin-forge deploy
```

## 📊 Package Relationships

```
User's Project
      │
      ├─ Uses CLI ──────→ glin-forge (npm wrapper)
      │                        │
      │                        └─→ glin-forge (Rust binary)
      │
      ├─ package.json
      │   ├─ @glin-forge/sdk ───────→ For scripts/deploy.ts
      │   └─ @glin-forge/testing ───→ For test/*.test.ts
      │
      ├─ glinforge.config.ts ─────→ Uses types from @glin-forge/sdk
      ├─ scripts/deploy.ts ───────→ Uses functions from @glin-forge/sdk
      └─ test/token.test.ts ──────→ Uses utilities from @glin-forge/testing
```

## 🎉 Summary

glin-forge now offers **the same convenient installation as Hardhat**:

- ✅ `npx glin-forge init` works out of the box
- ✅ No Rust installation required for JavaScript developers
- ✅ TypeScript SDK and testing utilities via npm
- ✅ Native Rust performance where it matters
- ✅ Flexible installation options for different workflows

The hybrid architecture provides the **best of both worlds**: Rust's performance for CLI operations and TypeScript's familiarity for user-facing code!
