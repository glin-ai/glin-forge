---
title: glin-forge init
description: Initialize a new contract project in the current directory
---

# glin-forge init

Initialize a new smart contract project in the current or specified directory.

## Synopsis

```bash
glin-forge init [PATH]
```

## Description

The `init` command initializes a new ink! smart contract project in the current directory or a specified path. Unlike `new`, it doesn't create a new directory - it works with an existing directory (which can be empty or non-empty).

## Arguments

### `[PATH]`

**Optional.** Target directory for initialization.

- **Default**: `.` (current directory)
- Can be relative or absolute path
- Directory must exist

## Examples

### Initialize in Current Directory

```bash
mkdir my-project
cd my-project
glin-forge init
```

### Initialize in Specific Directory

```bash
glin-forge init ./contracts/my-token
```

### Initialize in Empty Directory

```bash
mkdir my-contract
glin-forge init my-contract
cd my-contract
```

## Output

### Success Output

```
Initializing contract project in current directory...
✓ Generated Cargo.toml
✓ Generated lib.rs (ERC20 template)
✓ Created .gitignore

Project initialized successfully!

Next steps:
  glin-forge build --release
  glin-forge test
  glin-forge deploy --network testnet --account alice
```

### Error Cases

**Directory doesn't exist:**
```
✗ Error: Directory './my-project' does not exist
  Create the directory first: mkdir my-project
```

**Permission denied:**
```
✗ Error: Permission denied
  Cannot write to directory: /protected/path
```

## Generated Files

### Cargo.toml

Standard ink! project manifest:

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]

[dependencies]
ink = { version = "5.0", default-features = false }
scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2", default-features = false, features = ["derive"], optional = true }

[lib]
path = "lib.rs"
crate-type = ["cdylib"]

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
```

### lib.rs

ERC20 token template by default:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod my_project {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MyProject {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    impl MyProject {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        // ... standard ERC20 methods ...
    }
}
```

### .gitignore

```gitignore
/target
/Cargo.lock
**/*.rs.bk
.DS_Store
```

## Comparison: init vs new

| Feature | `init` | `new` |
|---------|--------|-------|
| Creates directory | No | Yes |
| Requires existing directory | Yes | No |
| Works in current dir | Yes | No |
| Directory name determines project name | Yes | Argument determines name |

### When to use `init`

- **Existing directory**: Already have a directory structure
- **Monorepo**: Part of a larger workspace
- **Custom setup**: Want control over directory structure

```bash
# Monorepo structure
mkdir -p contracts/token
cd contracts/token
glin-forge init
```

### When to use `new`

- **Fresh project**: Starting from scratch
- **Quick start**: Want directory created automatically
- **Standard workflow**: Following typical project creation

```bash
glin-forge new my-token
cd my-token
```

## Advanced Usage

### Initialize with Git

```bash
mkdir my-project
cd my-project
git init
glin-forge init
git add .
git commit -m "Initialize ink! contract"
```

### Initialize in Workspace

For a Cargo workspace:

```bash
# Create workspace structure
mkdir my-workspace
cd my-workspace

# Create workspace Cargo.toml
cat > Cargo.toml << EOF
[workspace]
members = ["contracts/*"]
EOF

# Initialize contract
mkdir -p contracts/token
glin-forge init contracts/token
```

### Multiple Contracts

```bash
mkdir contracts
cd contracts

# Initialize multiple contracts
mkdir token nft dao
glin-forge init token
glin-forge init nft
glin-forge init dao
```

## After Initialization

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

## Customizing After Init

### Change Template

Edit `lib.rs` to switch from ERC20 to another pattern:

```bash
# After init, replace lib.rs with custom contract
glin-forge init
# Edit lib.rs manually or copy from another template
```

### Update Metadata

Edit `Cargo.toml`:

```toml
[package]
name = "my-awesome-token"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "My awesome token contract"
```

## Troubleshooting

### Files Already Exist

**Problem:**
```
✗ Error: lib.rs already exists
  Remove or rename existing files first
```

**Solution:**

1. Backup and remove:
   ```bash
   mv lib.rs lib.rs.bak
   glin-forge init
   ```

2. Use a different directory:
   ```bash
   mkdir new-contract
   glin-forge init new-contract
   ```

### Wrong Project Name

The project name is derived from the directory name.

**Problem:**
Directory name is `my_contract` but you want `my-contract`.

**Solution:**
Rename directory before init:
```bash
mv my_contract my-contract
cd my-contract
glin-forge init
```

Or manually edit `Cargo.toml` after init:
```toml
[package]
name = "my-contract"  # Change this
```

## Tips

### Directory Naming

Good directory names:
- `my-token` - lowercase with hyphens
- `governance-contract` - descriptive
- `token-v2` - versioned

Avoid:
- `MyToken` - use lowercase
- `my_token` - use hyphens in directory names
- `contract` - too generic

### Clean Directory

For best results, init in an empty directory:

```bash
mkdir my-contract
glin-forge init my-contract
```

If directory has files, `init` will not overwrite them unless they conflict.

### Version Control

Always use version control:

```bash
mkdir my-project
cd my-project
git init
glin-forge init
git add .
git commit -m "Initial commit"
```

## Related Commands

- [`new`](./new) - Create new project with directory
- [`build`](./build) - Build the initialized project
- [`test`](./test) - Run tests
- [`deploy`](../deployment/deploy) - Deploy contract

## See Also

- [Getting Started](../../getting-started/installation) - Installation and setup
- [First Contract](../../getting-started/first-contract) - Complete tutorial
- [Templates Overview](../../templates/overview) - Available templates
