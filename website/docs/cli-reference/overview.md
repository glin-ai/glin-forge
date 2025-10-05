---
title: CLI Reference Overview
description: Complete command-line reference for GLIN Forge
---

# CLI Reference

GLIN Forge provides a comprehensive command-line interface for smart contract development. All commands follow a consistent structure with helpful options and clear output.

## Command Structure

```bash
glin-forge <COMMAND> [SUBCOMMAND] [ARGS] [OPTIONS]
```

### Common Patterns

- **Global Options**: Work with all commands (e.g., `--help`, `--version`)
- **Network Flag**: Most commands accept `-n, --network <NETWORK>`
- **Account Flag**: Transaction commands require `-a, --account <ACCOUNT>`
- **Confirmation**: Interactive commands support `-y, --yes` to skip prompts

## Command Categories

### Project Commands

Create and manage smart contract projects.

| Command | Description |
|---------|-------------|
| [`new`](./project/new) | Create new project from template |
| [`init`](./project/init) | Initialize project in current directory |
| [`build`](./project/build) | Build smart contract |
| [`test`](./project/test) | Run contract tests |

### Deployment Commands

Deploy and instantiate contracts on the blockchain.

| Command | Description |
|---------|-------------|
| [`deploy`](./deployment/deploy) | Deploy contract (upload + instantiate) |
| [`upload`](./deployment/upload) | Upload WASM code only |
| [`instantiate`](./deployment/instantiate) | Instantiate from code hash |

### Interaction Commands

Query state and execute transactions.

| Command | Description |
|---------|-------------|
| [`query`](./interaction/query) | Read contract state (no gas) |
| [`call`](./interaction/call) | Execute contract transaction |
| [`watch`](./interaction/watch) | Monitor contract events |

### Code Generation

Generate TypeScript types and React hooks.

| Command | Description |
|---------|-------------|
| [`typegen`](./codegen/typegen) | Generate TypeScript types from ABI |

### Verification

Verify contracts on block explorer.

| Command | Description |
|---------|-------------|
| [`verify`](./verification/verify) | Submit contract for verification |

### Configuration

Manage networks, accounts, and settings.

| Command | Description |
|---------|-------------|
| [`config`](config/manage-config) | Manage configuration |
| [`network`](config/network) | Manage networks |
| [`account`](config/account) | Manage accounts |
| [`balance`](config/balance) | Check account balance |

## Global Options

Available for all commands:

```bash
-h, --help       Print help information
-V, --version    Print version information
-v, --verbose    Enable verbose output
--quiet          Suppress non-essential output
```

## Network Options

Most commands accept these network-related options:

```bash
-n, --network <NETWORK>    Network to connect to [default: testnet]
                          Options: testnet, mainnet, local, or custom name
```

### Pre-configured Networks

| Network | RPC Endpoint | Explorer |
|---------|--------------|----------|
| `testnet` | wss://testnet.glin.network | https://explorer-testnet.glin.network |
| `mainnet` | wss://rpc.glin.network | https://explorer.glin.network |
| `local` | ws://localhost:9944 | None |

## Account Options

Commands that modify state require an account:

```bash
-a, --account <ACCOUNT>    Account to use for signing
                          Options: alice, bob, charlie, dave, eve, ferdie
                          Or custom account name
```

### Development Accounts

Built-in accounts for testing:

- `alice` - Default test account
- `bob` - Second test account
- `charlie` - Third test account
- `dave` - Fourth test account
- `eve` - Fifth test account
- `ferdie` - Sixth test account

## Common Workflows

### Create and Deploy

```bash
# Create project
glin-forge new my-contract --template erc20

# Build
cd my-contract
glin-forge build --release

# Deploy
glin-forge deploy --network testnet --account alice
```

### Query and Call

```bash
# Read state (no gas)
glin-forge query 5Contract... balanceOf 5Account...

# Execute transaction (costs gas)
glin-forge call 5Contract... transfer 5Recipient... 1000 \
  --account alice \
  --wait
```

### Generate Types

```bash
# Generate TypeScript types
glin-forge typegen --output ./types

# With React hooks
glin-forge typegen --output ./types --hooks
```

## Output Formats

### Standard Output

Clear, human-readable output with colors and symbols:

```
✓ Success message in green
✗ Error message in red
⠋ Loading spinner
→ Information
ℹ Tips and hints
```

### JSON Output

Some commands support `--json` for machine-readable output:

```bash
glin-forge query 5Contract... totalSupply --json
```

Output:
```json
{
  "result": "1000000",
  "type": "u128"
}
```

### Verbose Mode

Enable detailed logging:

```bash
glin-forge deploy --verbose
```

Shows:
- RPC requests/responses
- Transaction details
- Gas calculations
- Event decoding

## Exit Codes

GLIN Forge uses standard exit codes:

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `3` | Network error |
| `4` | Contract error |
| `5` | Build error |

### Usage in Scripts

```bash
#!/bin/bash
set -e  # Exit on error

glin-forge build --release
if [ $? -eq 0 ]; then
    echo "Build successful"
    glin-forge deploy --network testnet --account alice --yes
else
    echo "Build failed"
    exit 1
fi
```

## Environment Variables

Configure defaults via environment variables:

```bash
export GLIN_NETWORK="mainnet"
export GLIN_ACCOUNT="alice"
export GLIN_RPC_URL="wss://custom-node.example.com"
```

Then run commands without flags:

```bash
# Uses GLIN_NETWORK and GLIN_ACCOUNT from environment
glin-forge deploy
```

## Getting Help

### Command Help

View help for any command:

```bash
glin-forge --help
glin-forge deploy --help
glin-forge network add --help
```

### Examples in Help

Each command's help includes examples:

```bash
glin-forge deploy --help
```

Shows usage examples like:

```
Examples:
  glin-forge deploy --network testnet --account alice
  glin-forge deploy -w contract.wasm -m metadata.json --account alice
  glin-forge deploy --args "1000000,MyToken,MTK" --value 1 --account alice
```

## Quick Reference

### Most Used Commands

```bash
# Create project
glin-forge new my-project --template erc20

# Build
glin-forge build --release

# Deploy
glin-forge deploy --network testnet --account alice

# Query
glin-forge query <ADDRESS> <METHOD> [ARGS...]

# Call
glin-forge call <ADDRESS> <METHOD> [ARGS...] --account alice

# Generate types
glin-forge typegen --output ./types --hooks

# Watch events
glin-forge watch <ADDRESS> --follow

# Verify
glin-forge verify <ADDRESS> --network testnet
```

## Next Steps

Browse the detailed documentation for each command:

- **Project**: [new](./project/new), [build](./project/build), [test](./project/test)
- **Deploy**: [deploy](./deployment/deploy), [upload](./deployment/upload), [instantiate](./deployment/instantiate)
- **Interact**: [query](./interaction/query), [call](./interaction/call), [watch](./interaction/watch)
- **Generate**: [typegen](./codegen/typegen)
- **Config**: [config](./config/manage-config), [network](./config/network), [account](./config/account)

## Related Resources

- [Quick Start Guide](../getting-started/quick-start) - Get started quickly
- [Guides](../guides/building-contracts) - Step-by-step tutorials
- [Examples](../examples/erc20-token) - Real-world examples
- [Troubleshooting](../troubleshooting/common-errors) - Common issues
