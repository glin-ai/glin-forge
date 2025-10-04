# glin-forge

> Smart contract development CLI for GLIN Network

`glin-forge` is the official command-line tool for developing, deploying, and interacting with ink! smart contracts on the GLIN Network. Inspired by tools like GitHub CLI (`gh`) and Hardhat, it provides a seamless developer experience for blockchain development.

## Features

- 🔨 **Build & Test** - Compile and test ink! smart contracts
- 🚀 **Deploy** - Deploy contracts to any network (testnet/mainnet/local)
- 📞 **Interact** - Call contract methods and query state
- 📝 **TypeScript Generation** - Auto-generate TypeScript types from ABI
- 🔍 **Contract Verification** - Verify contracts on block explorer
- ⚡ **Gas Optimization** - Built-in gas estimation and tips
- 🌐 **Multi-network** - Easy network switching (testnet, mainnet, local)

## Installation

### From Cargo (Recommended)

```bash
cargo install glin-forge
```

### From Source

```bash
git clone https://github.com/glin-ai/glin-forge
cd glin-forge
cargo install --path .
```

### Binary Downloads

Download pre-built binaries from [GitHub Releases](https://github.com/glin-ai/glin-forge/releases):

- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

## Prerequisites

- **Rust** - Install from [rustup.rs](https://rustup.rs/)
- **cargo-contract** - Install with `cargo install cargo-contract --force`
- **ink!** - For writing smart contracts (optional)

## Quick Start

### 1. Create a New Contract

```bash
# Create from ERC20 template
glin-forge new my-token --template erc20

cd my-token
```

### 2. Build the Contract

```bash
glin-forge build
```

### 3. Deploy to Testnet

```bash
glin-forge deploy \
  --network testnet \
  --account alice
```

### 4. Interact with Contract

```bash
# Query (read-only, no gas)
glin-forge query 5ContractAddr... balanceOf 5Account...

# Call (transaction, costs gas)
glin-forge call 5ContractAddr... transfer 5Recipient... 1000 \
  --account alice
```

### 5. Generate TypeScript Types

```bash
glin-forge typegen --output ./frontend/src/types
```

## Commands

### Core Commands

#### `glin-forge build`
Compile the ink! smart contract.

```bash
glin-forge build [OPTIONS]

Options:
  -p, --path <PATH>    Path to contract project [default: .]
      --release        Build in release mode
      --verify         Verify contract after building
```

#### `glin-forge deploy`
Deploy contract to a network.

```bash
glin-forge deploy [OPTIONS]

Options:
  -w, --wasm <WASM>              Path to WASM file
  -m, --metadata <METADATA>      Path to metadata JSON
  -c, --args <ARGS>              Constructor arguments (comma-separated)
  -v, --value <VALUE>            Value to send (in GLIN) [default: 0]
  -n, --network <NETWORK>        Network [default: testnet]
  -a, --account <ACCOUNT>        Deploying account
  -g, --gas-limit <GAS_LIMIT>    Gas limit override
      --salt <SALT>              Salt for deterministic deployment
  -y, --yes                      Skip confirmation prompt
```

#### `glin-forge query`
Query contract state (read-only).

```bash
glin-forge query <ADDRESS> <METHOD> [ARGS]...

Arguments:
  <ADDRESS>  Contract address
  <METHOD>   Method name
  [ARGS]...  Method arguments

Options:
  -n, --network <NETWORK>      Network [default: testnet]
  -m, --metadata <METADATA>    Path to contract metadata
      --json                   Output as JSON
```

**Example:**
```bash
glin-forge query 5GrwvaEF... balanceOf 5Account... --json
```

#### `glin-forge call`
Execute contract transaction (state-changing).

```bash
glin-forge call <ADDRESS> <METHOD> [ARGS]...

Arguments:
  <ADDRESS>  Contract address
  <METHOD>   Method name
  [ARGS]...  Method arguments

Options:
  -n, --network <NETWORK>      Network [default: testnet]
  -a, --account <ACCOUNT>      Calling account
  -v, --value <VALUE>          Value to send [default: 0]
  -m, --metadata <METADATA>    Path to contract metadata
  -g, --gas-limit <GAS_LIMIT>  Gas limit override
  -y, --yes                    Skip confirmation
      --wait                   Wait for finalization
```

**Example:**
```bash
glin-forge call 5GrwvaEF... transfer 5Recipient... 1000 \
  --account alice \
  --network testnet \
  --wait
```

### TypeScript Generation

#### `glin-forge typegen`
Generate TypeScript types from contract ABI.

```bash
glin-forge typegen [OPTIONS]

Options:
  -a, --abi <ABI>               Path to ABI JSON
  -c, --contract <CONTRACT>     Contract address (fetch ABI from chain)
  -o, --output <OUTPUT>         Output directory [default: ./types]
  -n, --network <NETWORK>       Network [default: testnet]
      --hooks                   Generate React hooks
```

**Example:**
```bash
# Generate from local ABI
glin-forge typegen --abi ./target/ink/metadata.json --output ./frontend/src/types

# Generate with React hooks
glin-forge typegen --abi ./target/ink/metadata.json --hooks
```

**Generated Output:**
```typescript
// Generated TypeScript interface
export interface MyTokenContract {
  query: {
    balanceOf: (account: string) => Promise<bigint>
    totalSupply: () => Promise<bigint>
  }
  tx: {
    transfer: (to: string, amount: bigint) => Promise<TxResult>
    approve: (spender: string, amount: bigint) => Promise<TxResult>
  }
}
```

### Advanced Commands

#### `glin-forge upload`
Upload WASM code without instantiation.

```bash
glin-forge upload --wasm contract.wasm --account alice
```

#### `glin-forge instantiate`
Instantiate contract from uploaded code hash.

```bash
glin-forge instantiate <CODE_HASH> --account alice --args 1000000
```

#### `glin-forge watch`
Watch contract events in real-time.

```bash
glin-forge watch 5ContractAddr... Transfer --follow
```

#### `glin-forge verify`
Verify contract on block explorer.

```bash
glin-forge verify 5ContractAddr... \
  --wasm contract.wasm \
  --metadata metadata.json \
  --network testnet
```

### Configuration

#### `glin-forge config`
Manage network and account configuration.

```bash
# Set network RPC
glin-forge config set-network mainnet wss://rpc.glin.network

# Set default account
glin-forge config set-account alice

# View configuration
glin-forge config show
```

#### `glin-forge network`
Manage networks.

```bash
# List networks
glin-forge network list

# Add custom network
glin-forge network add custom wss://my-node.com

# Switch network
glin-forge network use testnet
```

#### `glin-forge account`
Manage accounts.

```bash
# Import account
glin-forge account import alice --keystore ./alice.json

# List accounts
glin-forge account list

# Export account
glin-forge account export alice
```

#### `glin-forge balance`
Check account balance.

```bash
glin-forge balance 5GrwvaEF... --network testnet
```

## Configuration File

Create `glin-forge.toml` in your project root:

```toml
[project]
name = "my-token"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[contract]
path = "./lib.rs"
output = "./target/ink"

[networks.testnet]
rpc = "wss://testnet.glin.network"
explorer = "https://explorer-testnet.glin.network"

[networks.mainnet]
rpc = "wss://rpc.glin.network"
explorer = "https://explorer.glin.network"

[accounts]
alice = { keystore = "~/.glin/keystore/alice.json" }
```

## Templates

Available contract templates:

- **erc20** - ERC20 token contract
- **erc721** - NFT contract
- **dao** - Basic DAO governance
- **multisig** - Multi-signature wallet
- **escrow** - Escrow contract

Create from template:
```bash
glin-forge new my-project --template erc20
```

## Examples

### Complete Deployment Flow

```bash
# 1. Create new contract
glin-forge new my-token --template erc20
cd my-token

# 2. Build contract
glin-forge build --release

# 3. Deploy to testnet
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK" \
  --value 1

# 4. Query total supply
glin-forge query <CONTRACT_ADDR> totalSupply

# 5. Transfer tokens
glin-forge call <CONTRACT_ADDR> transfer <RECIPIENT> 100 \
  --account alice \
  --wait

# 6. Generate TypeScript types
glin-forge typegen --output ./frontend/src/contracts
```

### Frontend Integration

After generating types:

```typescript
import { useMyToken } from './contracts/useMyToken'
import { useContractTx } from '@glin-ai/sdk-react'

function TransferButton() {
  const { contract } = useMyToken('5ContractAddress...')
  const { execute, loading } = useContractTx({
    contract,
    method: 'transfer'
  })

  const handleTransfer = async () => {
    await execute('5Recipient...', 1000n)
  }

  return (
    <button onClick={handleTransfer} disabled={loading}>
      Transfer
    </button>
  )
}
```

## Networks

Pre-configured networks:

| Network | RPC | Explorer |
|---------|-----|----------|
| **testnet** | `wss://testnet.glin.network` | https://explorer-testnet.glin.network |
| **mainnet** | `wss://rpc.glin.network` | https://explorer.glin.network |
| **local** | `ws://localhost:9944` | - |

## Troubleshooting

### cargo-contract not found

```bash
cargo install cargo-contract --force
```

### Network connection issues

Check your network configuration:
```bash
glin-forge config show
```

Test RPC connection:
```bash
glin-forge balance 5GrwvaEF... --network testnet
```

### Gas estimation errors

Manually specify gas limit:
```bash
glin-forge call ... --gas-limit 5000000000
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Development

```bash
# Clone repository
git clone https://github.com/glin-ai/glin-forge
cd glin-forge

# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## License

Apache-2.0 - see [LICENSE](LICENSE) for details.

## Support

- **Documentation:** https://docs.glin.ai/forge
- **Discord:** https://discord.gg/glin
- **GitHub Issues:** https://github.com/glin-ai/glin-forge/issues
- **Twitter:** [@glin_ai](https://twitter.com/glin_ai)

---

Built with ❤️ by the GLIN team
