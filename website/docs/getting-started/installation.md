---
title: Installation
description: Install GLIN Forge on Linux, macOS, or Windows
---

# Installing GLIN Forge

GLIN Forge can be installed in multiple ways depending on your platform and preferences. Choose the method that works best for you.

## Quick Installation

### From Cargo (Recommended)

The easiest way to install GLIN Forge is via Cargo, Rust's package manager:

```bash
cargo install glin-forge
```

This will download, compile, and install the latest version of GLIN Forge globally on your system.

### Verify Installation

After installation, verify it worked:

```bash
glin-forge --version
```

You should see output like:

```
glin-forge 0.1.0
```

## Platform-Specific Installation

### Linux

#### Ubuntu/Debian

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Install GLIN Forge
cargo install glin-forge
```

#### Arch Linux

```bash
# Install Rust
sudo pacman -S rust cargo

# Install GLIN Forge
cargo install glin-forge
```

#### From Binary (x86_64)

```bash
# Download latest release
wget https://github.com/glin-ai/glin-forge/releases/latest/download/glin-forge-linux-x86_64.tar.gz

# Extract
tar -xzf glin-forge-linux-x86_64.tar.gz

# Move to PATH
sudo mv glin-forge /usr/local/bin/

# Verify
glin-forge --version
```

### macOS

#### Using Cargo

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install GLIN Forge
cargo install glin-forge
```

#### From Binary (Intel)

```bash
# Download latest release
curl -L https://github.com/glin-ai/glin-forge/releases/latest/download/glin-forge-macos-x86_64.tar.gz -o glin-forge.tar.gz

# Extract
tar -xzf glin-forge.tar.gz

# Move to PATH
sudo mv glin-forge /usr/local/bin/

# Verify
glin-forge --version
```

#### From Binary (Apple Silicon)

```bash
# Download latest release
curl -L https://github.com/glin-ai/glin-forge/releases/latest/download/glin-forge-macos-aarch64.tar.gz -o glin-forge.tar.gz

# Extract
tar -xzf glin-forge.tar.gz

# Move to PATH
sudo mv glin-forge /usr/local/bin/

# Verify
glin-forge --version
```

### Windows

#### Using Cargo (WSL2 Recommended)

Windows users should use WSL2 (Windows Subsystem for Linux) for the best experience:

```powershell
# Install WSL2 (if not already installed)
wsl --install

# Inside WSL2, install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install GLIN Forge
cargo install glin-forge
```

#### Native Windows (Advanced)

```powershell
# Install Rust from https://rustup.rs/

# Install Visual Studio Build Tools
# Download from https://visualstudio.microsoft.com/downloads/

# Install GLIN Forge
cargo install glin-forge
```

## Installing from Source

For the latest development version or to contribute:

```bash
# Clone the repository
git clone https://github.com/glin-ai/glin-forge.git
cd glin-forge

# Build and install
cargo install --path .
```

This will compile from source and install the binary to `~/.cargo/bin/glin-forge`.

## Updating GLIN Forge

### Via Cargo

```bash
cargo install glin-forge --force
```

The `--force` flag will overwrite the existing installation with the latest version.

### Check for Updates

To see if a new version is available:

```bash
# Check current version
glin-forge --version

# Check latest version on crates.io
cargo search glin-forge
```

## Uninstalling

If you need to remove GLIN Forge:

```bash
cargo uninstall glin-forge
```

## Troubleshooting Installation

### Cargo Not Found

If you get `cargo: command not found`, you need to install Rust first:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Compilation Errors

If you encounter compilation errors:

1. **Update Rust:**
   ```bash
   rustup update stable
   ```

2. **Clear Cargo cache:**
   ```bash
   cargo clean
   cargo install glin-forge
   ```

3. **Check minimum Rust version:**
   GLIN Forge requires Rust 1.70 or later.
   ```bash
   rustc --version
   ```

### Permission Denied

If you get permission errors on Linux/macOS:

```bash
# Don't use sudo with cargo install
# Instead, ensure ~/.cargo/bin is in your PATH

# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"

# Then reload your shell
source ~/.bashrc  # or ~/.zshrc
```

### Binary Not in PATH

If the `glin-forge` command is not found after installation:

```bash
# Add Cargo's bin directory to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh users
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Next Steps

Now that you have GLIN Forge installed, proceed to:

- [Prerequisites](./prerequisites) - Install required dependencies (cargo-contract, ink!)
- [Quick Start](./quick-start) - Deploy your first contract in 5 minutes
- [First Contract](./first-contract) - Build a complete contract from scratch

## System Requirements

- **Operating System:** Linux, macOS, or Windows (WSL2)
- **RAM:** 4GB minimum, 8GB recommended
- **Disk Space:** 2GB for Rust toolchain and dependencies
- **Internet:** Required for downloading dependencies and connecting to networks

## Related Links

- [Rust Installation Guide](https://rustup.rs/)
- [cargo-contract Installation](https://github.com/paritytech/cargo-contract)
- [GitHub Releases](https://github.com/glin-ai/glin-forge/releases)
