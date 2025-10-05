---
title: Templates Overview
description: Pre-built smart contract templates for GLIN Forge
---

# Contract Templates

GLIN Forge provides pre-built templates to help you start building smart contracts quickly. Templates include production-ready code, tests, and best practices.

## Available Templates

### ERC20 Token

Fungible token standard implementation.

**Use cases:**
- Cryptocurrency tokens
- Utility tokens
- Governance tokens
- Reward points

**Features:**
- Transfer tokens
- Approve/TransferFrom pattern
- Balance tracking
- Total supply management
- Events (Transfer, Approval)

[Learn more →](./erc20)

```bash
glin-forge new my-token --template erc20
```

### ERC721 NFT

Non-fungible token (NFT) standard implementation.

**Use cases:**
- Digital collectibles
- Art NFTs
- Gaming items
- Identity tokens

**Features:**
- Unique token minting
- Ownership tracking
- Transfer/Approval
- Token metadata URI
- Events (Transfer, Approval)

[Learn more →](./erc721)

```bash
glin-forge new my-nft --template erc721
```

### DAO Governance

Decentralized autonomous organization template.

**Use cases:**
- Governance systems
- Voting mechanisms
- Proposal management
- Treasury management

**Features:**
- Proposal creation
- Voting system
- Execution threshold
- Member management
- Time-locked proposals

[Learn more →](./dao)

```bash
glin-forge new my-dao --template dao
```

## Template Structure

Each template includes:

### Source Code

**lib.rs** - Complete contract implementation with:
- Storage definitions
- Constructor(s)
- Query methods
- Transaction methods
- Events
- Error types
- Documentation

### Configuration

**Cargo.toml** - Pre-configured dependencies:
- ink! framework
- SCALE codec
- Type information
- Test dependencies

### Tests

Built-in unit tests:
- Constructor tests
- Method tests
- Error handling tests
- Edge case coverage

### Documentation

Inline documentation:
- Function descriptions
- Parameter explanations
- Return value documentation
- Usage examples

## Using Templates

### Create New Project

```bash
glin-forge new <PROJECT_NAME> --template <TEMPLATE>
```

**Example:**
```bash
glin-forge new my-token --template erc20
```

### Template Selection

| Template | Flag | Default |
|----------|------|---------|
| ERC20 | `--template erc20` | ✓ Yes |
| ERC721 | `--template erc721` | |
| DAO | `--template dao` | |

### Default Template

If no template is specified, ERC20 is used:

```bash
# These are equivalent
glin-forge new my-project
glin-forge new my-project --template erc20
```

## Template Customization

### 1. Project Metadata

Update `Cargo.toml`:

```toml
[package]
name = "my-token"
version = "1.0.0"  # Your version
authors = ["Your Name <you@example.com>"]
edition = "2021"
```

### 2. Contract Name

In `lib.rs`, update the module and struct names:

```rust
#[ink::contract]
mod my_custom_token {  // Change module name
    #[ink(storage)]
    pub struct MyCustomToken {  // Change struct name
        // ...
    }
}
```

### 3. Add Features

Extend the template with your custom logic:

```rust
#[ink::contract]
mod my_token {
    // ... existing code ...

    // Add custom features
    #[ink(message)]
    pub fn mint(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        // Your custom minting logic
        Ok(())
    }
}
```

### 4. Modify Storage

Add or remove storage fields:

```rust
#[ink(storage)]
pub struct MyToken {
    // Existing fields
    total_supply: Balance,
    balances: Mapping<AccountId, Balance>,

    // Add custom fields
    owner: AccountId,
    paused: bool,
}
```

## Template Variables

Templates use Handlebars with these variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `project_name` | Original name | `my-token` |
| `contract_name` | Snake case | `my_token` |
| `contract_name_pascal` | Pascal case | `MyToken` |
| `author` | Git config user | `Your Name <you@example.com>` |

**Usage in templates:**

```rust
#[ink::contract]
mod {{contract_name}} {
    #[ink(storage)]
    pub struct {{contract_name_pascal}} {
        // ...
    }
}
```

## Template Comparison

| Feature | ERC20 | ERC721 | DAO |
|---------|-------|--------|-----|
| **Complexity** | Low | Medium | High |
| **Lines of Code** | ~200 | ~350 | ~500 |
| **Use Case** | Tokens | NFTs | Governance |
| **Storage** | Mappings | Mappings + Vectors | Complex |
| **Methods** | 6 | 10 | 15+ |
| **Events** | 2 | 3 | 5+ |
| **Best For** | Beginners | Intermediate | Advanced |

## Best Practices

### 1. Start Simple

Begin with a template and customize:

```bash
# Start with ERC20
glin-forge new my-project --template erc20

# Modify lib.rs to add features
# Build and test incrementally
```

### 2. Understand Before Modifying

Read through the template code before making changes:

```bash
cd my-project
# Read lib.rs carefully
# Understand storage, methods, events
# Then customize
```

### 3. Test After Changes

Always test after customization:

```bash
glin-forge test
```

### 4. Version Control

Track your customizations:

```bash
git init
git add .
git commit -m "Initial commit from template"

# Make changes
git commit -m "Add custom minting logic"
```

## Creating Custom Templates

### Local Templates

Create your own templates in `.glin/templates/`:

```bash
mkdir -p ~/.glin/templates/my-template
```

Add template files:
- `Cargo.toml.hbs`
- `lib.rs.hbs`

Use with:

```bash
glin-forge new my-project --template my-template
```

### Sharing Templates

Share your templates:

1. Create a repository
2. Add template files
3. Document usage
4. Share with community

## Template Development

### Template Structure

```
templates/
├── erc20/
│   ├── Cargo.toml.hbs
│   └── lib.rs.hbs
├── erc721/
│   ├── Cargo.toml.hbs
│   └── lib.rs.hbs
└── dao/
    ├── Cargo.toml.hbs
    └── lib.rs.hbs
```

### Handlebars Syntax

**Variables:**
```handlebars
{{project_name}}
{{contract_name}}
{{contract_name_pascal}}
```

**Conditionals:**
```handlebars
{{#if has_feature}}
// Feature code
{{/if}}
```

**Loops:**
```handlebars
{{#each items}}
// Item: {{this}}
{{/each}}
```

## Common Customizations

### Add Access Control

```rust
#[ink(storage)]
pub struct MyToken {
    // ... existing fields ...
    owner: AccountId,
}

impl MyToken {
    #[ink(message)]
    pub fn only_owner_function(&mut self) -> Result<()> {
        let caller = self.env().caller();
        if caller != self.owner {
            return Err(Error::NotOwner);
        }
        // Function logic
        Ok(())
    }
}
```

### Add Pausable

```rust
#[ink(storage)]
pub struct MyToken {
    // ... existing fields ...
    paused: bool,
}

impl MyToken {
    #[ink(message)]
    pub fn pause(&mut self) -> Result<()> {
        self.only_owner()?;
        self.paused = true;
        Ok(())
    }

    #[ink(message)]
    pub fn unpause(&mut self) -> Result<()> {
        self.only_owner()?;
        self.paused = false;
        Ok(())
    }
}
```

### Add Minting

```rust
impl MyToken {
    #[ink(message)]
    pub fn mint(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        self.only_owner()?;

        let new_balance = self.balance_of(to) + amount;
        self.balances.insert(to, &new_balance);

        self.total_supply += amount;

        self.env().emit_event(Transfer {
            from: None,
            to: Some(to),
            value: amount,
        });

        Ok(())
    }
}
```

## Getting Help

### Template Issues

If you encounter issues with a template:

1. Check [Troubleshooting Guide](../troubleshooting/common-errors)
2. Review [First Contract Tutorial](../getting-started/first-contract)
3. Ask on [Discord](https://discord.gg/glin)
4. Open [GitHub Issue](https://github.com/glin-ai/glin-forge/issues)

### Template Requests

Request new templates:

1. [GitHub Discussions](https://github.com/glin-ai/glin-forge/discussions)
2. [Discord Community](https://discord.gg/glin)
3. [Feature Requests](https://github.com/glin-ai/glin-forge/issues/new?template=feature_request.md)

## Next Steps

- [ERC20 Template Guide](./erc20) - Detailed ERC20 documentation
- [ERC721 Template Guide](./erc721) - NFT template details
- [DAO Template Guide](./dao) - Governance template
- [First Contract Tutorial](../getting-started/first-contract) - Build from scratch

## Resources

- [ink! Documentation](https://use.ink/) - ink! framework
- [OpenBrush](https://openbrush.io/) - Additional contract libraries
- [Template Repository](https://github.com/glin-ai/glin-forge/tree/main/templates) - Source code
