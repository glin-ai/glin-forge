---
title: glin-forge new
description: Create a new smart contract project from a template
---

# glin-forge new

Create a new smart contract project from a pre-built template.

## Synopsis

```bash
glin-forge new <NAME> [OPTIONS]
```

## Description

The `new` command creates a new smart contract project in a directory named `<NAME>`. It uses Handlebars templates to generate boilerplate code for common contract patterns like ERC20 tokens, ERC721 NFTs, and DAOs.

## Arguments

### `<NAME>`

**Required.** The name of your project.

- Creates a directory with this name
- Used to generate the contract name (snake_case)
- Used in `Cargo.toml` package name

## Options

### `-t, --template <TEMPLATE>`

Template to use for project scaffolding.

- **Default**: `erc20`
- **Available templates**:
  - `erc20` - Fungible token (ERC20 standard)
  - `erc721` - Non-fungible token (NFT)
  - `dao` - Decentralized autonomous organization

### `-h, --help`

Print help information.

## Examples

### Create an ERC20 Token

```bash
glin-forge new my-token --template erc20
```

Creates:
```
my-token/
├── Cargo.toml
├── lib.rs
└── .gitignore
```

### Create an NFT Contract

```bash
glin-forge new my-nft --template erc721
```

### Create a DAO Contract

```bash
glin-forge new my-dao --template dao
```

### Create with Default Template

```bash
glin-forge new my-contract
```

Uses the default `erc20` template.

## Output

### Success Output

```
Creating new contract project: my-token
✓ Template: erc20
✓ Created directory: my-token
✓ Generated Cargo.toml
✓ Generated lib.rs
✓ Created .gitignore

Next steps:
  cd my-token
  glin-forge build
  glin-forge deploy --network testnet --account alice
```

### Error Cases

**Directory already exists:**
```
✗ Error: Directory 'my-token' already exists
  Use a different name or remove the existing directory
```

**Invalid template:**
```
✗ Error: Template 'invalid' not found
  Available templates: erc20, erc721, dao
```

## Generated Files

### Cargo.toml

Project manifest with dependencies:

```toml
[package]
name = "my-token"
version = "0.1.0"
edition = "2021"

[dependencies]
ink = { version = "5.0", default-features = false }
scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2", default-features = false, features = ["derive"], optional = true }

[lib]
path = "lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
```

### lib.rs

Contract source code based on the selected template. See [Templates](../../templates/overview) for details.

### .gitignore

Standard gitignore for Rust projects:

```gitignore
/target
/Cargo.lock
**/*.rs.bk
```

## Template Variables

Templates use Handlebars with these variables:

| Variable | Example | Description |
|----------|---------|-------------|
| `project_name` | `my-token` | Original project name |
| `contract_name` | `my_token` | Snake_case version |
| `contract_name_pascal` | `MyToken` | PascalCase version |
| `author` | `Your Name <you@example.com>` | Git config user |

## After Creating

### Navigate to Project

```bash
cd my-token
```

### Build the Contract

```bash
glin-forge build --release
```

### Run Tests

```bash
glin-forge test
```

### Deploy

```bash
glin-forge deploy --network testnet --account alice
```

## Comparison: new vs init

| Feature | `new` | `init` |
|---------|-------|--------|
| Creates directory | Yes | No |
| Works in empty dir | No | Yes |
| Works in current dir | No | Yes |
| Use case | New project | Existing directory |

### When to use `new`

- Starting a fresh project
- Want a new directory created
- Standard workflow

```bash
glin-forge new my-project
cd my-project
```

### When to use `init`

- Already in a directory
- Converting existing project
- Custom setup

```bash
mkdir my-project
cd my-project
glin-forge init
```

## Customizing Templates

After creation, customize the generated code:

### Edit Contract Name

In `lib.rs`:

```rust
#[ink::contract]
mod my_token {  // Change this
    // ...
}
```

### Update Package Metadata

In `Cargo.toml`:

```toml
[package]
name = "my-token"
version = "1.0.0"  # Update version
authors = ["Your Name <you@example.com>"]  # Update author
```

### Add Dependencies

```toml
[dependencies]
ink = { version = "5.0", default-features = false }
# Add more dependencies
ink_storage = { version = "5.0", default-features = false }
```

## Tips

### Project Naming

Good project names:
- `my-token` - lowercase with hyphens
- `governance-dao` - descriptive
- `nft-marketplace` - clear purpose

Avoid:
- `MyToken` - use lowercase
- `my_token` - use hyphens, not underscores
- `t` - too short

### Start Simple

Begin with a template and customize:

```bash
# Start with ERC20
glin-forge new my-project --template erc20

# Then modify lib.rs to add features
cd my-project
# Edit lib.rs
```

### Version Control

Initialize git after creation:

```bash
glin-forge new my-project
cd my-project
git init
git add .
git commit -m "Initial commit from glin-forge new"
```

## Troubleshooting

### Directory Exists Error

**Problem:**
```
✗ Error: Directory 'my-project' already exists
```

**Solutions:**

1. Use a different name:
   ```bash
   glin-forge new my-project-v2
   ```

2. Remove existing directory:
   ```bash
   rm -rf my-project
   glin-forge new my-project
   ```

3. Use `init` in existing directory:
   ```bash
   cd my-project
   glin-forge init
   ```

### Template Not Found

**Problem:**
```
✗ Error: Template 'erc2' not found
```

**Solution:**
Check spelling and use valid template:
```bash
glin-forge new my-project --template erc20
```

List available templates:
```bash
glin-forge new --help
```

## Related Commands

- [`init`](./init) - Initialize in current directory
- [`build`](./build) - Build the created project
- [`test`](./test) - Test the contract
- [`deploy`](../deployment/deploy) - Deploy to network

## See Also

- [Templates Overview](../../templates/overview) - Learn about available templates
- [ERC20 Template](../../templates/erc20) - ERC20 token template details
- [First Contract Tutorial](../../getting-started/first-contract) - Complete walkthrough
- [Quick Start](../../getting-started/quick-start) - 5-minute guide
