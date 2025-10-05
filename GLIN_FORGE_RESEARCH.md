# GLIN Forge - Comprehensive Research Document

**Version:** 0.1.0
**Last Updated:** 2025-10-05
**Total Lines of Code:** 3,505 (Rust)
**Repository:** https://github.com/glin-ai/glin-forge

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture & Structure](#architecture--structure)
3. [Commands & CLI](#commands--cli)
4. [Features & Capabilities](#features--capabilities)
5. [Configuration](#configuration)
6. [Technical Details](#technical-details)
7. [Code Generation](#code-generation)
8. [Templates](#templates)
9. [Dependencies](#dependencies)
10. [Development Information](#development-information)
11. [Integration with GLIN Ecosystem](#integration-with-glin-ecosystem)
12. [Common Workflows](#common-workflows)

---

## 1. Project Overview

### What is glin-forge?

`glin-forge` is the official command-line interface (CLI) tool for developing, deploying, and interacting with ink! smart contracts on the GLIN Network. It serves as the primary developer tool in the GLIN ecosystem, similar to how Hardhat serves Ethereum or `cargo-contract` serves Substrate.

### Main Purpose and Value Proposition

- **Unified Developer Experience**: Single CLI for the entire smart contract lifecycle
- **Network Abstraction**: Easy switching between testnet, mainnet, and local networks
- **Type Safety**: Auto-generate TypeScript types from contract ABIs
- **Developer Productivity**: Pre-built templates, automatic gas estimation, contract verification
- **Frontend Integration**: Generate React hooks for seamless frontend development

### Key Features

1. **Smart Contract Development**
   - Create contracts from templates (ERC20, ERC721, DAO)
   - Build and compile ink! contracts via `cargo-contract`
   - Run unit and E2E tests
   - Contract verification and validation

2. **Deployment & Interaction**
   - Deploy contracts with automatic gas estimation
   - Call contract methods (transactions)
   - Query contract state (read-only)
   - Upload code and instantiate separately
   - Watch contract events in real-time

3. **Code Generation**
   - Generate TypeScript types from ABIs
   - Generate React hooks for contract interaction
   - Frontend-ready integration code

4. **Network & Account Management**
   - Pre-configured networks (testnet, mainnet, local)
   - Account generation and import
   - Balance checking
   - Development accounts (Alice, Bob, etc.)

5. **Developer Tools**
   - Contract verification on block explorer
   - Event watching and monitoring
   - Gas optimization tips
   - Build artifact validation

---

## 2. Architecture & Structure

### Directory Structure

```
glin-forge/
├── src/
│   ├── main.rs                    # CLI entry point & command routing
│   ├── cli/                       # CLI command implementations
│   │   ├── mod.rs                 # Module exports
│   │   ├── init.rs                # Initialize project in current dir
│   │   ├── new.rs                 # Create new project from template
│   │   ├── build.rs               # Build contract with cargo-contract
│   │   ├── test.rs                # Run contract tests
│   │   ├── deploy.rs              # Deploy contract (upload + instantiate)
│   │   ├── call.rs                # Call contract method (transaction)
│   │   ├── query.rs               # Query contract state (read-only)
│   │   ├── upload.rs              # Upload code without instantiation
│   │   ├── instantiate.rs         # Instantiate from code hash
│   │   ├── typegen.rs             # Generate TypeScript types
│   │   ├── watch.rs               # Watch contract events
│   │   ├── verify.rs              # Verify contract on explorer
│   │   ├── config.rs              # Manage configuration
│   │   ├── account.rs             # Account management
│   │   ├── balance.rs             # Check account balance
│   │   └── network.rs             # Network management
│   ├── codegen/                   # Code generation modules
│   │   ├── mod.rs                 # Module exports & re-exports
│   │   ├── types.rs               # TypeScript type generation
│   │   ├── hooks.rs               # React hooks generation
│   │   └── metadata.rs            # Metadata parsing utilities
│   ├── config/                    # Configuration management
│   │   └── mod.rs                 # Network configs (testnet/mainnet/local)
│   ├── contract/                  # Contract interaction logic
│   │   └── mod.rs                 # Deploy, call, query, upload operations
│   └── templates/                 # (not src, but root level)
├── templates/                     # Contract templates
│   ├── erc20/
│   │   ├── Cargo.toml.hbs         # Handlebars template for Cargo.toml
│   │   └── lib.rs.hbs             # Handlebars template for lib.rs
│   ├── erc721/
│   │   ├── Cargo.toml.hbs
│   │   └── lib.rs.hbs
│   └── dao/
│       ├── Cargo.toml.hbs
│       └── lib.rs.hbs
├── docs/
│   └── SDK_ARCHITECTURE.md        # Multi-language SDK architecture docs
├── Cargo.toml                     # Package manifest
├── README.md                      # User-facing documentation
└── SUBXT_0.44_RESEARCH_REPORT.md  # Subxt upgrade research

```

### Main Components

#### 1. CLI Layer (`src/cli/`)
- **Purpose**: User-facing command handlers
- **Responsibilities**: Parse arguments, validate inputs, display output
- **Pattern**: Each command has its own module with `Args` struct and `execute()` function

#### 2. Contract Operations (`src/contract/mod.rs`)
- **Purpose**: Core blockchain interaction logic
- **Key Functions**:
  - `deploy_contract()` - Upload + instantiate in one step
  - `upload_code()` - Upload WASM code
  - `instantiate_contract()` - Create instance from code hash
  - `call_contract()` - Submit transaction
  - `query_contract()` - Read-only state query
- **Dependencies**: Uses `glin-client`, `glin-contracts` from glin-sdk-rust

#### 3. Code Generation (`src/codegen/`)
- **Purpose**: Generate TypeScript/React code from contract ABIs
- **Modules**:
  - `types.rs` - Generate TypeScript interfaces
  - `hooks.rs` - Generate React hooks
  - `metadata.rs` - Parse and extract ABI information

#### 4. Configuration (`src/config/`)
- **Purpose**: Network and account configuration
- **Pre-configured Networks**:
  - `testnet`: wss://testnet.glin.network
  - `mainnet`: wss://rpc.glin.network
  - `local`: ws://localhost:9944

#### 5. Templates (`templates/`)
- **Purpose**: Scaffold new contracts from pre-built templates
- **Format**: Handlebars (`.hbs`) templates
- **Available Templates**: ERC20, ERC721, DAO

### Key Files and Their Purposes

| File | Purpose | Lines | Key Responsibilities |
|------|---------|-------|---------------------|
| `src/main.rs` | Entry point | 99 | Command routing, error handling |
| `src/contract/mod.rs` | Contract ops | 530 | Deploy, call, query, upload contracts |
| `src/cli/deploy.rs` | Deployment | 224 | Deploy workflow, gas estimation |
| `src/cli/typegen.rs` | Type gen | 108 | Generate TS types from ABI |
| `src/cli/verify.rs` | Verification | 210 | Submit contract to explorer |
| `src/cli/watch.rs` | Event watching | 174 | Monitor contract events |
| `src/codegen/types.rs` | TS types | 139 | TypeScript interface generation |
| `src/codegen/hooks.rs` | React hooks | 80 | React hook generation |
| `src/config/mod.rs` | Config | 60 | Network configuration |

---

## 3. Commands & CLI

### Command Hierarchy

```
glin-forge
├── init                    # Initialize in current directory
├── new                     # Create new project
├── build                   # Build contract
├── test                    # Run tests
├── deploy                  # Deploy contract
├── call                    # Call contract method
├── query                   # Query contract state
├── upload                  # Upload code only
├── instantiate             # Instantiate from code hash
├── typegen                 # Generate TypeScript types
├── watch                   # Watch contract events
├── verify                  # Verify on explorer
├── config                  # Manage configuration
├── account                 # Manage accounts
│   ├── list
│   ├── generate
│   ├── import
│   └── show
├── balance                 # Check balance
└── network                 # Manage networks
    ├── list
    ├── add
    └── use
```

### Detailed Command Reference

#### `glin-forge init [PATH]`

Initialize a new contract project in the current or specified directory.

```bash
glin-forge init
glin-forge init my-project
```

**Options:**
- `PATH` - Target directory (default: `.`)

**Output:**
- `Cargo.toml` - Project manifest
- `lib.rs` - Contract source (ERC20 template)
- `.gitignore` - Git ignore rules

#### `glin-forge new <NAME> [OPTIONS]`

Create a new contract project from a template.

```bash
glin-forge new my-token --template erc20
glin-forge new my-nft --template erc721
glin-forge new my-dao --template dao
```

**Arguments:**
- `<NAME>` - Project name (required)

**Options:**
- `-t, --template <TEMPLATE>` - Template name (default: `erc20`)
  - Available: `erc20`, `erc721`, `dao`

**Output:**
- Creates directory `<NAME>/`
- Generates `Cargo.toml` and `lib.rs` from template
- Uses Handlebars for variable substitution

#### `glin-forge build [OPTIONS]`

Build the ink! smart contract using `cargo-contract`.

```bash
glin-forge build
glin-forge build --release
glin-forge build --verify
glin-forge build -p ./my-contract
```

**Options:**
- `-p, --path <PATH>` - Contract project path (default: `.`)
- `--release` - Build in release mode (optimized)
- `--verify` - Verify contract after building

**Build Process:**
1. Check if `cargo-contract` is installed
2. Run `cargo contract build [--release]`
3. List output files in `target/ink/`
4. If `--verify`:
   - Calculate code hash
   - Validate metadata structure
   - Check WASM size
   - List constructors and messages
   - Warn if size > 100KB

**Output Files:**
- `target/ink/*.wasm` - Contract bytecode
- `target/ink/*.json` - Contract metadata (ABI)
- `target/ink/*.contract` - Bundle file

#### `glin-forge test [OPTIONS]`

Run contract tests (unit or E2E).

```bash
glin-forge test
glin-forge test --e2e
glin-forge test --test transfer_works
glin-forge test --nocapture
```

**Options:**
- `-p, --path <PATH>` - Contract project path (default: `.`)
- `--e2e` - Run end-to-end tests
- `--test <NAME>` - Filter tests by name
- `--nocapture` - Show output of successful tests

**Test Types:**
- **Unit Tests**: `#[ink::test]` functions
- **E2E Tests**: Run with `--features e2e-tests`

#### `glin-forge deploy [OPTIONS]`

Deploy a contract to the network (upload + instantiate).

```bash
glin-forge deploy --network testnet --account alice
glin-forge deploy -w contract.wasm -m metadata.json --account alice
glin-forge deploy --args "1000000,MyToken,MTK" --value 1 --account alice --yes
```

**Options:**
- `-w, --wasm <PATH>` - WASM file (auto-detect from `target/ink/`)
- `-m, --metadata <PATH>` - Metadata JSON (auto-detect)
- `-c, --args <ARGS>` - Constructor arguments (comma-separated)
- `-v, --value <VALUE>` - Value to send in GLIN (default: `0`)
- `-n, --network <NETWORK>` - Target network (default: `testnet`)
- `-a, --account <ACCOUNT>` - Deploying account (required)
- `-g, --gas-limit <GAS>` - Gas limit override
- `--salt <SALT>` - Deterministic deployment salt
- `-y, --yes` - Skip confirmation prompt

**Workflow:**
1. Auto-detect or load WASM and metadata
2. Parse constructor arguments
3. Connect to network RPC
4. Get signer account (dev account or custom)
5. Estimate gas (display tips)
6. Prompt for confirmation (unless `--yes`)
7. Submit `instantiate_with_code` transaction
8. Wait for finalization
9. Parse events to extract contract address and code hash
10. Display contract address and explorer link

**Gas Estimation:**
- Default: 5B refTime, 2M proofSize
- Can override with `--gas-limit`
- Adds 20% buffer recommendation

**Example Output:**
```
Deploying contract...

Contract artifacts:
  WASM: ./target/ink/my_token.wasm
  Metadata: ./target/ink/metadata.json

Deployment details:
  Network: testnet
  RPC: wss://testnet.glin.network
  Account: alice
  Value: 0 GLIN

Proceed with deployment? [y/N]: y

Connecting to network...
✓ Connected to wss://testnet.glin.network
✓ Using account: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

Gas Estimation:
  → refTime: 5,000,000,000
  → proofSize: 2,000,000
  ℹ Using auto-estimated gas limit
    Tip: Add 20% buffer for safety

Deploying contract...

✓ Contract deployed successfully!

Contract info:
  Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Explorer: https://explorer-testnet.glin.network/contract/5FHneW46...
  Transaction: 0x1234...
  Code Hash: 0x5678...
```

#### `glin-forge call <ADDRESS> <METHOD> [ARGS]... [OPTIONS]`

Call a contract method (state-changing transaction).

```bash
glin-forge call 5Contract... transfer 5Recipient... 1000 --account alice
glin-forge call 5Contract... approve 5Spender... 5000 --account alice --wait
glin-forge call 5Contract... mint --value 10 --account alice --yes
```

**Arguments:**
- `<ADDRESS>` - Contract address (required)
- `<METHOD>` - Method name to call (required)
- `[ARGS]...` - Method arguments (space-separated)

**Options:**
- `-n, --network <NETWORK>` - Network (default: `testnet`)
- `-a, --account <ACCOUNT>` - Calling account (required)
- `-v, --value <VALUE>` - Value to send (default: `0`)
- `-m, --metadata <PATH>` - Contract metadata (auto-detect)
- `-g, --gas-limit <GAS>` - Gas limit override
- `-y, --yes` - Skip confirmation
- `--wait` - Wait for finalization

**Workflow:**
1. Load contract metadata
2. Connect to network
3. Get signer account
4. Encode method call (selector + arguments)
5. Estimate gas
6. Prompt for confirmation
7. Submit `Contracts::call` transaction
8. Display transaction hash
9. Optionally wait for finalization (with `--wait`)

#### `glin-forge query <ADDRESS> <METHOD> [ARGS]... [OPTIONS]`

Query contract state (read-only, no gas cost).

```bash
glin-forge query 5Contract... balanceOf 5Account...
glin-forge query 5Contract... totalSupply --json
glin-forge query 5Contract... owner --network mainnet
```

**Arguments:**
- `<ADDRESS>` - Contract address (required)
- `<METHOD>` - Method name to query (required)
- `[ARGS]...` - Method arguments (space-separated)

**Options:**
- `-n, --network <NETWORK>` - Network (default: `testnet`)
- `-m, --metadata <PATH>` - Contract metadata (auto-detect)
- `--json` - Output as JSON

**Workflow:**
1. Load contract metadata
2. Connect to network
3. Encode method call
4. Submit RPC call to `ContractsApi_call` (dry-run)
5. Decode result using return type from ABI
6. Display result (formatted or JSON)

**Technical Details:**
- Uses `state_call` RPC method (no transaction)
- Decodes `ContractExecResult` from RPC response
- Parses return type from metadata
- No gas cost, no account required

#### `glin-forge upload <OPTIONS>`

Upload contract code without instantiation.

```bash
glin-forge upload --wasm contract.wasm --account alice
```

**Options:**
- `-w, --wasm <PATH>` - WASM file (required)
- `-a, --account <ACCOUNT>` - Uploading account (required)
- `-n, --network <NETWORK>` - Network (default: `testnet`)

**Output:**
- Code hash for later instantiation

#### `glin-forge instantiate <CODE_HASH> [OPTIONS]`

Instantiate a contract from previously uploaded code.

```bash
glin-forge instantiate 0x1234... --account alice --args "1000000"
```

**Arguments:**
- `<CODE_HASH>` - Code hash from upload (required)

**Options:**
- `-a, --account <ACCOUNT>` - Account (required)
- `-c, --args <ARGS>` - Constructor arguments
- `-v, --value <VALUE>` - Value to send (default: `0`)
- `-m, --metadata <PATH>` - Contract metadata
- `-n, --network <NETWORK>` - Network (default: `testnet`)

#### `glin-forge typegen [OPTIONS]`

Generate TypeScript types and optionally React hooks from contract ABI.

```bash
glin-forge typegen --abi ./target/ink/metadata.json
glin-forge typegen --contract 5ContractAddr... --network testnet
glin-forge typegen --abi metadata.json --hooks --output ./frontend/types
```

**Options:**
- `-a, --abi <PATH>` - Path to ABI JSON
- `-c, --contract <ADDRESS>` - Contract address (fetch ABI from chain)
- `-o, --output <DIR>` - Output directory (default: `./types`)
- `-n, --network <NETWORK>` - Network (default: `testnet`)
- `--hooks` - Generate React hooks

**Output Files:**
- `<ContractName>.ts` - TypeScript interfaces
- `use<ContractName>.ts` - React hooks (if `--hooks`)

**Generated TypeScript Interface:**
```typescript
// Generated by glin-forge
import { Contract, QueryResult, TxResult } from '@glin-ai/sdk';

export interface MyTokenContractQuery {
  balanceOf: (account: string) => Promise<bigint>
  totalSupply: () => Promise<bigint>
}

export interface MyTokenContractTx {
  transfer: (to: string, amount: bigint) => Promise<TxResult>
  approve: (spender: string, amount: bigint) => Promise<TxResult>
}

export interface MyTokenContract {
  query: MyTokenContractQuery;
  tx: MyTokenContractTx;
  address: string;
}
```

**Generated React Hook:**
```typescript
// Generated by glin-forge
import { useContract, useContractQuery, useContractTx } from '@glin-ai/sdk-react';

export function useMyToken(address: string, signer?: any) {
  const { contract, loading, error } = useContract({
    address,
    abi: metadata,
    signer,
  });

  return { contract: contract as MyTokenContract, loading, error };
}
```

#### `glin-forge watch <ADDRESS> [EVENT] [OPTIONS]`

Watch contract events in real-time or historically.

```bash
glin-forge watch 5Contract... Transfer --follow
glin-forge watch 5Contract... --limit 50
glin-forge watch 5Contract... --from-block 1000
```

**Arguments:**
- `<ADDRESS>` - Contract address (required)
- `[EVENT]` - Event name filter (optional, shows all if omitted)

**Options:**
- `-n, --network <NETWORK>` - Network (default: `testnet`)
- `-f, --follow` - Follow mode (live streaming)
- `--limit <N>` - Max events to show (default: `10`)
- `--from-block <N>` - Start from block number

**Modes:**
- **Historical**: Fetch events from recent blocks (default 100 blocks)
- **Follow**: Subscribe to finalized blocks and stream events

#### `glin-forge verify <ADDRESS> [OPTIONS]`

Verify contract on block explorer.

```bash
glin-forge verify 5Contract... --network testnet
glin-forge verify 5Contract... -w contract.wasm -m metadata.json
```

**Arguments:**
- `<ADDRESS>` - Contract address (required)

**Options:**
- `-w, --wasm <PATH>` - WASM file (auto-detect)
- `-m, --metadata <PATH>` - Metadata (auto-detect)
- `-s, --source <DIR>` - Source directory
- `-n, --network <NETWORK>` - Network (default: `testnet`)
- `--compiler-version <VERSION>` - Compiler version

**Workflow:**
1. Calculate code hash from WASM
2. Verify code hash exists on-chain
3. Submit verification request to explorer API
4. Display verification status and explorer link

#### `glin-forge config <SUBCOMMAND>`

Manage network and account configuration.

```bash
glin-forge config show
glin-forge config set-network mainnet wss://rpc.glin.network
glin-forge config set-account alice
```

#### `glin-forge account <SUBCOMMAND>`

Manage accounts.

```bash
glin-forge account list
glin-forge account generate my-account
glin-forge account import my-account --seed "word1 word2 ..."
glin-forge account show alice
```

**Subcommands:**
- `list` - List all accounts (dev + custom)
- `generate <NAME>` - Generate new account with random mnemonic
- `import <NAME> --seed <SEED>` - Import from seed phrase
- `show <NAME>` - Show account details

**Development Accounts:**
- alice, bob, charlie, dave, eve, ferdie

#### `glin-forge balance <ADDRESS> [OPTIONS]`

Check account balance.

```bash
glin-forge balance 5Account... --network testnet
```

#### `glin-forge network <SUBCOMMAND>`

Manage networks.

```bash
glin-forge network list
glin-forge network add custom wss://my-node.com
glin-forge network use testnet
```

---

## 4. Features & Capabilities

### Smart Contract Development Features

#### 1. Template System
- **Pre-built Templates**: ERC20, ERC721, DAO
- **Template Engine**: Handlebars for variable substitution
- **Variables**:
  - `project_name` - Original project name
  - `contract_name` - Snake_case version
  - `contract_name_pascal` - PascalCase version
  - `author` - Author information

#### 2. Build System
- **Delegates to `cargo-contract`**: Leverages official ink! tooling
- **Automatic Detection**: Finds WASM and metadata in `target/ink/`
- **Verification**: Optional post-build validation
  - Code hash calculation
  - Metadata structure validation
  - WASM size analysis
  - Constructor/message counting
  - Size warnings (>100KB, >500KB)

#### 3. Testing Features
- **Unit Tests**: Standard `#[ink::test]` tests
- **E2E Tests**: Via `--e2e` flag and feature flag
- **Test Filtering**: `--test <name>` pattern matching
- **Output Control**: `--nocapture` for verbose output

### Deployment Capabilities

#### 1. Flexible Deployment
- **All-in-one**: `deploy` command (upload + instantiate)
- **Two-step**: `upload` then `instantiate`
- **Deterministic**: Support for salt parameter

#### 2. Gas Management
- **Auto-estimation**: Default gas limits provided
- **Manual override**: `--gas-limit` option
- **Visual feedback**: Formatted gas display with tips
- **Buffer recommendations**: Suggests 20% safety margin

#### 3. Transaction Tracking
- **Event parsing**: Extracts contract address, code hash
- **Explorer links**: Direct links to explorer
- **Wait for finalization**: Optional `--wait` flag
- **Progress indicators**: Spinner with block updates

### Code Generation Capabilities

#### 1. TypeScript Type Generation
- **Type Mapping**:
  - `u8, u16, u32, u64, u128` → `bigint`
  - `String, str` → `string`
  - `bool` → `boolean`
  - `Vec` → `any[]`
  - `Option<T>` → `T | null`
- **Interface Structure**:
  - `<Name>ContractQuery` - Read-only methods
  - `<Name>ContractTx` - State-changing methods
  - `<Name>Contract` - Combined interface

#### 2. React Hooks Generation
- **Generated Hooks**:
  - `use<Name>(address, signer?)` - Main contract hook
  - `use<Name>Query(address)` - Query helper
  - `use<Name>Tx(address, signer)` - Transaction helper
- **Integration**: Uses `@glin-ai/sdk-react`
- **Type Safety**: Fully typed with generated interfaces

### Advanced Features

#### 1. Contract Verification
- **On-chain Validation**: Checks code hash exists
- **Explorer Integration**: Submits to verification API
- **Payload**: WASM bytecode + metadata + source
- **Status Tracking**: Verification ID and status

#### 2. Event Watching
- **Real-time Streaming**: Subscribe to finalized blocks
- **Historical Queries**: Fetch past events
- **Event Filtering**: By event name
- **Block Range**: `--from-block` support
- **Output Formatting**: JSON event data display

#### 3. Multi-network Support
- **Pre-configured**: testnet, mainnet, local
- **Custom Networks**: Add via `network add`
- **Network Switching**: Via `--network` flag
- **Configuration**: RPC endpoint + explorer URL

#### 4. Account Management
- **Development Accounts**: Built-in Alice, Bob, etc.
- **Custom Accounts**: Generate or import
- **Seed Phrase**: BIP39 mnemonic generation
- **Key Derivation**: SR25519 cryptography
- **Security Warnings**: Safe storage reminders

---

## 5. Configuration

### Network Configuration

**Default Networks** (defined in `src/config/mod.rs`):

```rust
networks.insert("testnet", NetworkConfig {
    rpc: "wss://testnet.glin.network",
    explorer: Some("https://explorer-testnet.glin.network"),
});

networks.insert("mainnet", NetworkConfig {
    rpc: "wss://rpc.glin.network",
    explorer: Some("https://explorer.glin.network"),
});

networks.insert("local", NetworkConfig {
    rpc: "ws://localhost:9944",
    explorer: None,
});
```

### Configuration File

**File**: `glin-forge.toml` (in project root)

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

[networks.custom]
rpc = "wss://custom-node.example.com"
explorer = "https://explorer.example.com"

[accounts]
alice = { keystore = "~/.glin/keystore/alice.json" }
bob = { keystore = "~/.glin/keystore/bob.json" }
```

### Environment Variables

Currently not used, but could support:
- `GLIN_NETWORK` - Default network
- `GLIN_ACCOUNT` - Default account
- `GLIN_RPC_URL` - Custom RPC endpoint

---

## 6. Technical Details

### Substrate/ink! Integration

#### 1. Contract Compilation
- **Tool**: `cargo-contract` (external dependency)
- **Output**: WASM bytecode + metadata JSON
- **Location**: `target/ink/`
- **Verification**: Post-build validation with Blake2 hashing

#### 2. Contract Deployment
Uses Substrate's `Contracts` pallet:
- **Extrinsic**: `Contracts::instantiate_with_code`
- **Parameters**:
  - `value`: Balance to transfer
  - `gas_limit`: WeightV2 (refTime, proofSize)
  - `storage_deposit_limit`: Optional deposit cap
  - `code`: WASM bytecode
  - `data`: Constructor selector + encoded args
  - `salt`: For deterministic addresses

#### 3. Contract Calls
- **Read-only**: `ContractsApi_call` RPC (dry-run)
- **Transactions**: `Contracts::call` extrinsic
- **Gas**: WeightV2 structure (ref_time, proof_size)

### Network Interaction

#### 1. Subxt Integration
- **Version**: 0.44.x
- **Client**: `subxt::OnlineClient`
- **Dynamic API**: `subxt::dynamic` for runtime-agnostic calls
- **RPC Client**: `subxt::backend::rpc::RpcClient`

#### 2. Transaction Submission
```rust
let tx = subxt::dynamic::tx("Contracts", "instantiate_with_code", vec![...]);
let events = client
    .tx()
    .sign_and_submit_then_watch_default(&tx, signer)
    .await?
    .wait_for_finalized_success()
    .await?;
```

#### 3. Event Parsing
- **Event Extraction**: Parse `Instantiated`, `CodeStored`, `ContractEmitted`
- **Field Access**: `event.field_values()` with JSON conversion
- **Type Safety**: Dynamic decoding with error handling

### Account Management

#### 1. Key Derivation
- **Algorithm**: SR25519 (Schnorrkel)
- **Library**: `subxt-signer`
- **Mnemonic**: BIP39 standard (12-24 words)
- **Derivation**: `Keypair::from_phrase(mnemonic, None)`

#### 2. Development Accounts
Hardcoded seeds for testing:
- Alice: `//Alice`
- Bob: `//Bob`
- Charlie: `//Charlie`
- Dave: `//Dave`
- Eve: `//Eve`
- Ferdie: `//Ferdie`

#### 3. Address Formats
- **SS58**: Substrate address format
- **Hex**: 0x-prefixed hex (32 bytes)
- **AccountId32**: Substrate account type

### Contract Metadata Processing

#### 1. Metadata Structure
ink! metadata JSON schema (v5):
```json
{
  "contract": {
    "name": "MyContract",
    "version": "1.0.0"
  },
  "spec": {
    "constructors": [...],
    "messages": [...],
    "events": [...],
    "types": [...]
  }
}
```

#### 2. Selector Encoding
- **Selector**: First 4 bytes of Blake2-256 hash of method signature
- **Arguments**: SCALE-encoded based on type registry
- **Call Data**: `[selector (4 bytes)] + [encoded args]`

#### 3. Return Type Decoding
- **RPC Response**: `ContractExecResult` structure
- **Decoding Steps**:
  1. Skip gas consumption fields
  2. Skip storage deposit
  3. Skip debug message
  4. Decode result variant (Ok/Err)
  5. Extract and decode return data

---

## 7. Code Generation

### TypeScript Type Generation

**Source**: `src/codegen/types.rs`

#### Type Mapping Logic
```rust
fn parse_type(type_value: &JsonValue) -> String {
    match display_name {
        "u8" | "u16" | "u32" | "u64" | "u128" => "bigint",
        "String" | "str" => "string",
        "bool" => "boolean",
        "Vec" => "any[]",
        "Option" => format!("{} | null", inner_type),
        _ => "any"
    }
}
```

#### Interface Generation
```typescript
export interface MyTokenContractQuery {
  balanceOf: (account: string) => Promise<bigint>
  totalSupply: () => Promise<bigint>
  allowance: (owner: string, spender: string) => Promise<bigint>
}

export interface MyTokenContractTx {
  transfer: (to: string, amount: bigint) => Promise<TxResult>
  approve: (spender: string, amount: bigint) => Promise<TxResult>
  transferFrom: (from: string, to: string, amount: bigint) => Promise<TxResult>
}
```

### React Hooks Generation

**Source**: `src/codegen/hooks.rs`

#### Generated Hook Structure
```typescript
export function useMyToken(address: string, signer?: any) {
  const { contract, loading, error } = useContract({
    address,
    abi: metadata,
    signer,
  });

  return { contract: contract as MyTokenContract, loading, error };
}

export function useMyTokenQuery(address: string) {
  const contract = useMyToken(address);

  const query = async (method: string, ...args: any[]) => {
    if (!contract.contract) throw new Error('Contract not loaded');
    return contract.contract.query[method](...args);
  };

  return { ...contract, query };
}

export function useMyTokenTx(address: string, signer: any) {
  const contract = useMyToken(address, signer);

  const tx = async (method: string, ...args: any[]) => {
    if (!contract.contract) throw new Error('Contract not loaded');
    if (!signer) throw new Error('Signer required for transactions');
    return contract.contract.tx[method](...args);
  };

  return { ...contract, tx };
}
```

### Metadata Parsing

**Source**: `src/codegen/metadata.rs`

#### Extraction Functions
- `extract_contract_name(abi)` - Get contract name
- `extract_contract_version(abi)` - Get version
- `extract_messages(abi)` - List all messages with args/return types
- `extract_constructors(abi)` - List constructors

#### Data Structures
```rust
pub struct MessageInfo {
    pub label: String,
    pub mutates: bool,
    pub args: Vec<ArgumentInfo>,
    pub return_type: JsonValue,
}

pub struct ArgumentInfo {
    pub label: String,
    pub type_info: JsonValue,
}
```

---

## 8. Templates

### Available Templates

#### 1. ERC20 Token (`templates/erc20/`)

**Files**:
- `Cargo.toml.hbs` - Package manifest
- `lib.rs.hbs` - Contract source

**Features**:
- Total supply tracking
- Balance mapping
- Allowance system
- Transfer with events
- Approve/transferFrom
- Built-in tests

**Events**:
- `Transfer { from, to, value }`
- `Approval { owner, spender, value }`

**Methods**:
```rust
// Queries
pub fn total_supply(&self) -> Balance
pub fn balance_of(&self, owner: AccountId) -> Balance

// Transactions
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error>
pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Error>
pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Error>
```

#### 2. ERC721 NFT (`templates/erc721/`)

**Features**: (not fully detailed in research, but likely includes)
- Token minting
- Ownership tracking
- Transfer/approval
- Metadata URI

#### 3. DAO Governance (`templates/dao/`)

**Features**: (not fully detailed in research, but likely includes)
- Proposal creation
- Voting system
- Execution threshold
- Member management

### Template Rendering

**Engine**: Handlebars

**Variables**:
```json
{
  "project_name": "my-token",
  "contract_name": "my_token",
  "contract_name_pascal": "MyToken",
  "author": "Your Name <you@example.com>"
}
```

**Usage in Template**:
```rust
#[ink::contract]
mod {{contract_name}} {
    #[ink(storage)]
    pub struct {{contract_name_pascal}} {
        // ...
    }
}
```

---

## 9. Dependencies

### Core Dependencies (from `Cargo.toml`)

#### GLIN SDK
```toml
glin-client = { path = "../glin-sdk-rust/glin-client" }
glin-contracts = { path = "../glin-sdk-rust/glin-contracts" }
glin-types = { path = "../glin-sdk-rust/glin-types" }
```

**Purpose**: Core blockchain interaction, contract utilities, shared types

#### CLI Framework
```toml
clap = { version = "4.5", features = ["derive", "cargo"] }
colored = "2.1"
indicatif = "0.17"
```

**Purpose**: Argument parsing, colored output, progress indicators

#### Substrate & Contract Interaction
```toml
subxt = "0.44"
subxt-signer = "0.44"
subxt-core = "0.44"
```

**Purpose**: Substrate client library, key management

#### Contract Metadata & Compilation
```toml
ink_metadata = "5.1"
scale = { package = "parity-scale-codec", version = "3.6", features = ["derive"] }
scale-info = { version = "2.11", features = ["derive"] }
```

**Purpose**: Parse ink! metadata, SCALE encoding/decoding

#### Async Runtime
```toml
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"
```

**Purpose**: Async/await support, futures utilities

#### Serialization
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
```

**Purpose**: JSON/TOML parsing, serialization

#### Utilities
```toml
anyhow = "1.0"
thiserror = "1.0"
dirs = "5.0"
hex = "0.4"
sp-core-hashing = "15.0"
rand = "0.8"
```

**Purpose**: Error handling, filesystem, hashing, randomness

#### HTTP Client
```toml
reqwest = { version = "0.12", features = ["json"] }
```

**Purpose**: Explorer API verification requests

#### Template Rendering
```toml
handlebars = "5.1"
```

**Purpose**: Template variable substitution

### External Dependencies

**cargo-contract**: Required for building contracts
```bash
cargo install cargo-contract --force
```

### Version Alignment

**Critical**: Must match glin-sdk-rust dependency versions
- Subxt: 0.44.x
- ink!: 5.1.x
- SCALE codec: 3.6.x

---

## 10. Development Information

### Building glin-forge

```bash
# Clone repository
git clone https://github.com/glin-ai/glin-forge
cd glin-forge

# Build
cargo build

# Build release
cargo build --release

# Install locally
cargo install --path .
```

### Testing Structure

**Test Framework**: Rust's built-in `#[test]`

**Test Files**:
- `src/codegen/types.rs` - Type parsing tests
- `src/codegen/hooks.rs` - Hook generation tests
- `src/codegen/metadata.rs` - Metadata extraction tests

**Run Tests**:
```bash
cargo test
cargo test --verbose
```

**Example Test**:
```rust
#[test]
fn test_parse_type_primitives() {
    let type_u32 = serde_json::json!({
        "displayName": ["u32"]
    });
    assert_eq!(parse_type(&type_u32), "bigint");
}
```

### Code Organization

**Module Structure**:
```
glin-forge
├── main.rs               # Entry point, command routing
├── cli/                  # User-facing commands
│   ├── mod.rs            # Module exports
│   └── [command].rs      # Individual command implementations
├── codegen/              # Code generation
│   ├── mod.rs            # Re-exports
│   ├── types.rs          # TypeScript types
│   ├── hooks.rs          # React hooks
│   └── metadata.rs       # ABI parsing
├── config/               # Configuration
│   └── mod.rs            # Network configs
├── contract/             # Contract operations
│   └── mod.rs            # Deploy, call, query
└── templates/            # (not in src)
```

**Design Patterns**:
1. **Command Pattern**: Each CLI command has its own module
2. **Args Struct**: Using `clap::Parser` derive macro
3. **Execute Function**: `pub async fn execute(args: Args) -> Result<()>`
4. **Error Propagation**: Using `anyhow::Result` for error handling
5. **Separation of Concerns**: CLI layer vs. business logic layer

### Contribution Guidelines

**Code Style**:
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new features
- Update README for user-facing changes

**Commit Messages**:
- Use conventional commits format
- Examples: `feat: add watch command`, `fix: gas estimation`

---

## 11. Integration with GLIN Ecosystem

### Relationship with glin-sdk-rust

**glin-forge** depends on **glin-sdk-rust** workspace:

```
glin-sdk-rust/
├── glin-client/          # Used by: All commands
├── glin-contracts/       # Used by: Deploy, call, query, typegen
├── glin-types/           # Used by: Type conversions
└── glin-indexer/         # Not used by forge (used by glinscan)
```

**Key Integrations**:

1. **Client Creation**:
```rust
use glin_client::create_client;
let client = create_client("wss://testnet.glin.network").await?;
```

2. **Account Management**:
```rust
use glin_client::{get_dev_account, get_address, account_from_seed};
let signer = get_dev_account("alice")?;
let address = get_address(&signer);
```

3. **Metadata Operations**:
```rust
use glin_contracts::metadata;
let metadata = metadata::parse_metadata(&json_string)?;
let constructor = metadata::get_default_constructor(&metadata)?;
let message = metadata::get_message_spec(&metadata, "transfer")?;
```

4. **Encoding**:
```rust
use glin_contracts::encoding;
let encoded = encoding::encode_args(&args, param_specs, metadata)?;
let decoded = encoding::decode_result(&data, return_type, metadata)?;
```

### Frontend Integration

**Generated Code** is designed to work with:

1. **@glin-ai/sdk** (TypeScript SDK)
```typescript
import { Contract } from '@glin-ai/sdk';
import { MyTokenContract } from './types/MyToken';
```

2. **@glin-ai/sdk-react** (React SDK)
```typescript
import { useMyToken } from './types/useMyToken';

function App() {
  const { contract, loading } = useMyToken('5Contract...');
  // Use contract.query.balanceOf(...) etc.
}
```

### Block Explorer Integration

**glinscan** (block explorer) uses the same SDK:
- Fetches contract metadata
- Decodes transactions
- Displays contract code
- Verification API endpoint

**Verification Flow**:
1. glin-forge calculates code hash
2. Submits WASM + metadata to explorer API
3. Explorer verifies code hash matches on-chain
4. Stores verified contract data
5. Displays source code and ABI

---

## 12. Common Workflows

### Workflow 1: Complete Contract Development

```bash
# 1. Create project from template
glin-forge new my-token --template erc20
cd my-token

# 2. Build contract
glin-forge build --release --verify

# 3. Run tests
glin-forge test

# 4. Deploy to testnet
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "1000000,MyToken,MTK"

# Output: Contract address 5Contract...

# 5. Query state
glin-forge query 5Contract... totalSupply

# 6. Transfer tokens
glin-forge call 5Contract... transfer 5Recipient... 100 \
  --account alice \
  --wait

# 7. Generate TypeScript types for frontend
glin-forge typegen \
  --abi ./target/ink/metadata.json \
  --output ./frontend/src/contracts \
  --hooks

# 8. Verify on explorer
glin-forge verify 5Contract... --network testnet
```

### Workflow 2: Two-step Deployment (Instantiate Factory)

```bash
# 1. Upload code once
glin-forge upload --wasm contract.wasm --account alice
# Output: Code hash 0x1234...

# 2. Instantiate multiple times with different params
glin-forge instantiate 0x1234... --account alice --args "1000000"
glin-forge instantiate 0x1234... --account bob --args "2000000"
glin-forge instantiate 0x1234... --account charlie --args "3000000"
```

### Workflow 3: Contract Interaction Loop

```bash
# Query loop
while true; do
  glin-forge query 5Contract... balanceOf 5Account... --json | jq
  sleep 10
done

# Watch events
glin-forge watch 5Contract... Transfer --follow
```

### Workflow 4: Multi-network Deployment

```bash
# Deploy to testnet
glin-forge deploy --network testnet --account alice --args "1000000"

# Test on testnet
glin-forge call 5TestnetAddr... transfer 5Bob... 100 --account alice --network testnet

# Deploy to mainnet
glin-forge deploy --network mainnet --account production --args "1000000"

# Verify both
glin-forge verify 5TestnetAddr... --network testnet
glin-forge verify 5MainnetAddr... --network mainnet
```

### Workflow 5: Account Management

```bash
# List development accounts
glin-forge account list

# Generate new account
glin-forge account generate my-production
# Save seed phrase securely!

# Import existing account
glin-forge account import my-backup --seed "word1 word2 ..."

# Check balance
glin-forge balance 5MyAccount... --network mainnet

# Use in deployment
glin-forge deploy --account my-production --network mainnet
```

### Workflow 6: Frontend Integration

**Backend**:
```bash
# Build and generate types
glin-forge build --release
glin-forge typegen --abi ./target/ink/metadata.json --output ../frontend/src/contracts --hooks
```

**Frontend (React)**:
```typescript
// frontend/src/App.tsx
import { useMyToken } from './contracts/useMyToken';
import { useContractTx } from '@glin-ai/sdk-react';

function TokenTransfer() {
  const { contract } = useMyToken('5Contract...');
  const { execute, loading } = useContractTx({
    contract,
    method: 'transfer'
  });

  const handleTransfer = async () => {
    await execute('5Recipient...', 1000n);
  };

  return (
    <button onClick={handleTransfer} disabled={loading}>
      {loading ? 'Sending...' : 'Transfer'}
    </button>
  );
}
```

---

## Summary

**glin-forge** is a comprehensive CLI tool that streamlines the entire smart contract development lifecycle on GLIN Network:

**Key Strengths**:
1. **Developer Productivity**: Templates, auto-detection, gas estimation
2. **Type Safety**: TypeScript code generation from ABIs
3. **Frontend Integration**: React hooks generation
4. **Network Flexibility**: Easy switching between testnet/mainnet/local
5. **Substrate Integration**: Built on battle-tested subxt library

**Architecture Highlights**:
- 3,505 lines of well-organized Rust code
- Modular command structure with clear separation of concerns
- Re-uses glin-sdk-rust for blockchain operations
- Comprehensive CLI with 15+ commands
- Template system for rapid contract scaffolding

**Target Users**:
- Smart contract developers (primary)
- Frontend developers (via typegen)
- DevOps/deployment engineers
- Contract auditors (via verify)

**Future Enhancements** (potential):
- Contract upgradability patterns
- Multi-sig deployment
- Gas profiling and optimization tools
- Contract testing frameworks
- IDE integrations

---

**Document Version**: 1.0
**Research Completed**: 2025-10-05
**Researcher**: Claude (Anthropic)
**Source**: Direct codebase analysis of /home/eralp/Projects/glin/glin-forge
