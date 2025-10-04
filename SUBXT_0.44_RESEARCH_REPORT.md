# Subxt 0.44.0 Comprehensive Research Report

**Date:** 2025-10-04
**Target Version:** subxt 0.44.0
**Previous Version:** subxt 0.35.x
**Purpose:** Building CLI tool for pallet-contracts interaction on Substrate chains

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Version History: 0.35 → 0.44](#version-history-035--044)
3. [Core API Overview](#core-api-overview)
4. [Transaction Submission](#transaction-submission)
5. [Storage Queries](#storage-queries)
6. [Event Handling](#event-handling)
7. [RPC Methods](#rpc-methods)
8. [pallet-contracts Integration](#pallet-contracts-integration)
9. [Signing and Account Management](#signing-and-account-management)
10. [Best Practices](#best-practices)
11. [Migration Guide from 0.35](#migration-guide-from-035)

---

## Executive Summary

Subxt 0.44.0 is a mature Rust library for interacting with Substrate-based blockchains. Key capabilities include:

- **Type-safe API generation** from chain metadata
- **Multiple client types**: OnlineClient (network access) and OfflineClient (offline operations)
- **WASM compatibility** for browser-based applications
- **Comprehensive blockchain operations**: transactions, storage, events, RPC
- **Modern signing** via `subxt_signer` crate (replacing deprecated PairSigner)

### Major Changes from 0.35 to 0.44:
- **Removed `substrate-compat` feature** (v0.40.0) - use `subxt_signer` instead
- **V5 transaction support** (v0.41.0) with improved mortality handling
- **Metadata V16 support** (v0.42.0) with Pallet View Functions
- **FFI support** (v0.43.0) for Python and Node.js integration
- **Rust 2024 edition** (v0.42.0)

---

## Version History: 0.35 → 0.44

### v0.35.0 (Baseline)
- Original baseline version with PairSigner support
- substrate-compat feature available

### v0.36.0
- Type system improvements

### v0.37.0
- ✅ Added support for `CheckMetadataHash` sign extension
- ✅ Implemented `sign_prehashed` for ECDSA and Eth keypairs
- 🐛 Fixed type de-duplication regression

### v0.38.0
- ✅ **Major:** Support for decoding V5 extrinsics
- ✅ Stabilized `ChainHeadBackend`
- ✅ Introduced reconnecting RPC client
- ✅ Support for generating metadata from runtime WASM
- ✅ Improved Eth-style chain support
- ✅ Introduced `subxt-core` crate (#[no_std] compatible)
- 🔧 **Breaking:** `ExtrinsicParams` trait now receives `&ClientState<T>`

### v0.39.0
- Incremental improvements

### v0.40.0 🚨 MAJOR BREAKING CHANGES
- 🚨 **Breaking:** Removed `substrate-compat` feature flag
- **Reason:** Impossible to maintain single sp_runtime/sp_core version compatibility
- **Migration:** Use `subxt_signer` crate instead of PairSigner
- ✅ Reverted polkadot-sdk umbrella crate usage
- ✅ Bumped Polkadot SDK dependencies to latest

### v0.41.0
- ✅ Added `subxt-rpcs` crate for direct RPC interactions
- ✅ Support for creating V5 transactions
- ✅ More flexible transaction extension handling
- ✅ Simplified transaction mortality configuration
- 🔧 **Breaking:** APIs like `sign_and_submit` now take `T::AccountId` instead of `T::Address`
- 🔧 **Breaking:** Offline functions `create_partial_signed_with_nonce` renamed (lost `_with_nonce` suffix)

### v0.42.0
- ✅ Metadata V16 support
- ✅ Introduced **Pallet View Functions**
- ✅ Updated `Config` trait (removed `Hash` associated type)
- ✅ Default hasher set to `DynamicHasher256` (auto-selects Keccak256/BlakeTwo256)
- ✅ Upgraded to Rust 2024 edition
- ✅ Improved V1 archive RPC support

### v0.43.0
- ✅ Removed `codec::Encode` and `codec::Decode` derives from generated APIs by default
- ✅ Improved transaction mortality handling
- ✅ Added FFI example for Python and Node.js
- ✅ More flexible derive options

### v0.44.0 (Current)
- ✅ Introduced `subxt-historic` crate for non-head-of-chain block access
- ✅ Latest stable release with all improvements from 0.35-0.43

---

## Core API Overview

### Client Types

#### OnlineClient
```rust
use subxt::{OnlineClient, PolkadotConfig};

// Connect to node
let api = OnlineClient::<PolkadotConfig>::new().await?;

// Or with custom URL
let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944").await?;
```

**Characteristics:**
- Requires network connection
- Default transaction mortality: 32 blocks
- Auto-fetches metadata and runtime info

#### OfflineClient
```rust
// Derive from OnlineClient
let offline_api = online_api.offline();

// Or create with metadata
let offline_api = OfflineClient::<PolkadotConfig>::new(metadata);
```

**Characteristics:**
- No network required
- Default transaction mortality: Immortal
- Useful for offline signing

### Metadata Generation

```rust
// Using #[subxt] macro
#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

// Generate metadata with CLI
// cargo install subxt-cli
// subxt metadata > polkadot_metadata.scale
```

### Key Modules

```rust
use subxt::{
    backend,        // Backend trait for API interactions
    tx,            // Create and submit extrinsics
    storage,       // Access storage items
    events,        // Handle blockchain events
    runtime_api,   // Execute runtime API calls
    dynamic,       // Dynamic (untyped) interfaces
};
```

---

## Transaction Submission

### 1. Static Transaction (Recommended)

```rust
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Build transaction
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx()
        .balances()
        .transfer_allow_death(dest, 10_000);

    // Sign and submit
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("Transaction finalized: {:?}", events);
    Ok(())
}
```

### 2. Dynamic Transaction

```rust
use subxt::dynamic::{tx, Value};

let tx_payload = tx(
    "System",
    "remark",
    vec![Value::from_bytes("Hello there")]
);

let hash = api
    .tx()
    .sign_and_submit_then_watch_default(&tx_payload, &signer)
    .await?;
```

### 3. Custom Transaction Parameters

```rust
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;

// Configure parameters
let tx_params = Params::new()
    .tip(1_000)               // Add tip
    .mortal(32)               // Mortal with 32 block period
    .build();

// Submit with custom params
let hash = api
    .tx()
    .sign_and_submit(&balance_transfer_tx, &signer, tx_params)
    .await?;
```

### 4. Watching Transaction Progress

```rust
use subxt::tx::TxStatus;

let mut tx_progress = api
    .tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?;

while let Some(status) = tx_progress.next().await {
    match status? {
        TxStatus::InBestBlock(in_block) => {
            println!("In best block: {:?}", in_block.block_hash());
        }
        TxStatus::InFinalizedBlock(in_block) => {
            println!("Finalized: {:?}", in_block.block_hash());
            let events = in_block.wait_for_success().await?;
            println!("Events: {:?}", events);
            break;
        }
        TxStatus::Error { message } => {
            println!("Error: {}", message);
            break;
        }
        TxStatus::Invalid { message } => {
            println!("Invalid: {}", message);
            break;
        }
        TxStatus::Dropped { message } => {
            println!("Dropped: {}", message);
            break;
        }
        _ => {}
    }
}
```

### 5. Offline Transaction Creation

```rust
use subxt::tx::Payload;

// Create partial transaction offline
let mut partial_tx = api.tx().create_partial_signed(
    &payload,
    &signer,
    Default::default()
)?;

// Submit later when online
let hash = api.tx().submit(&partial_tx).await?;
```

---

## Storage Queries

### 1. Static Storage Query

```rust
use subxt::utils::AccountId32;
use std::str::FromStr;

#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

// Query account balance
let account = AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;
let storage_query = polkadot::storage().system().account(&account);

let account_info = api
    .storage()
    .at_latest()
    .await?
    .fetch(&storage_query)
    .await?;

if let Some(info) = account_info {
    println!("Free balance: {}", info.data.free);
}
```

### 2. Dynamic Storage Query

```rust
use subxt::dynamic::{storage, Value};

let account = dev::alice().public_key();
let storage_query = storage(
    "System",
    "Account",
    vec![Value::from_bytes(account)]
);

let result = api
    .storage()
    .at_latest()
    .await?
    .fetch(&storage_query)
    .await?;

if let Some(value) = result {
    println!("Value: {:?}", value.to_value()?);
}
```

### 3. Storage Iteration (Static)

```rust
// Iterate all accounts
let storage_query = polkadot::storage().system().account_iter();

let mut results = api
    .storage()
    .at_latest()
    .await?
    .iter(storage_query)
    .await?;

while let Some(Ok(kv)) = results.next().await {
    println!("Account: {:?}", kv.keys);
    println!("Info: {:?}", kv.value);
}
```

### 4. Storage Iteration (Dynamic)

```rust
let keys: Vec<Value> = vec![]; // Empty for full iteration
let storage_query = subxt::dynamic::storage("System", "Account", keys);

let mut results = api
    .storage()
    .at_latest()
    .await?
    .iter(storage_query)
    .await?;

while let Some(Ok(kv)) = results.next().await {
    println!("Keys: {:?}", kv.keys);
    println!("Value: {:?}", kv.to_value()?);
}
```

### 5. Partial Key Iteration

```rust
// Iterate multisig accounts for specific signer
let alice_bob_account_id = /* ... */;
let storage_query = polkadot::storage()
    .multisig()
    .multisigs_iter1(alice_bob_account_id);

let mut results = api
    .storage()
    .at_latest()
    .await?
    .iter(storage_query)
    .await?;

while let Some(Ok(kv)) = results.next().await {
    println!("Multisig: {:?}", kv.value);
}
```

### 6. Storage at Specific Block

```rust
// Get storage at block hash
let block_hash = /* ... */;
let account_info = api
    .storage()
    .at(block_hash)
    .fetch(&storage_query)
    .await?;
```

---

## Event Handling

### 1. Get Latest Block Events

```rust
let events = api.events().at_latest().await?;

// Iterate all events dynamically
for event in events.iter() {
    let event = event?;
    println!("Pallet: {}", event.pallet_name());
    println!("Event: {}", event.variant_name());
    println!("Data: {}", event.field_values()?);
}
```

### 2. Static Event Decoding

```rust
#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

let events = api.events().at_latest().await?;

for event in events.iter() {
    let event = event?;

    // Decode as root Event enum
    if let Ok(ev) = event.as_root_event::<polkadot::Event>() {
        match ev {
            polkadot::Event::Balances(balance_event) => {
                println!("Balance event: {:?}", balance_event);
            }
            polkadot::Event::System(system_event) => {
                println!("System event: {:?}", system_event);
            }
            _ => {}
        }
    }
}
```

### 3. Finding Specific Events

```rust
// Find first matching event
let transfer_event = events
    .find_first::<polkadot::balances::events::Transfer>()?;

if let Some(ev) = transfer_event {
    println!("Transfer from: {:?}", ev.from);
    println!("Transfer to: {:?}", ev.to);
    println!("Amount: {}", ev.amount);
}
```

### 4. Events from Transaction

```rust
let events = api
    .tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;

// Find specific event in transaction
let transfer = events
    .find_first::<polkadot::balances::events::Transfer>()?;
```

### 5. Subscribe to Block Events

```rust
use subxt::blocks::Block;

let mut blocks_sub = api.blocks().subscribe_finalized().await?;

while let Some(block) = blocks_sub.next().await {
    let block = block?;
    let events = block.events().await?;

    for event in events.iter() {
        let event = event?;
        println!("Block {}: {}::{}",
            block.number(),
            event.pallet_name(),
            event.variant_name()
        );
    }
}
```

### 6. Historical Events

```rust
// Get events at specific block
let block_hash = /* ... */;
let events = api.events().at(block_hash).await?;

// Or via blocks API
let block = api.blocks().at(block_hash).await?;
let events = block.events().await?;
```

---

## RPC Methods

### 1. Creating RPC Client

```rust
use subxt::backend::rpc::RpcClient;
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::PolkadotConfig;

// Create RPC client
let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;

// Create legacy RPC methods
let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

// Reuse for full client
let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client).await?;
```

### 2. System RPC Calls

```rust
// Get system information
let system_name = rpc.system_name().await?;
let system_chain = rpc.system_chain().await?;
let system_health = rpc.system_health().await?;
let system_properties = rpc.system_properties().await?;

println!("System: {}", system_name);
println!("Chain: {}", system_chain);
println!("Health: {:?}", system_health);
println!("Properties: {:?}", system_properties);
```

### 3. State Calls (Generic Helper)

```rust
use parity_scale_codec::{Encode, Decode};

async fn state_call<C, A: Encode, R: Decode>(
    rpc: &LegacyRpcMethods<C>,
    func: &str,
    args: A,
) -> Result<R>
where
    C: subxt::Config,
{
    let params = args.encode();
    let bytes = rpc.state_call(func, Some(&params), None).await?;
    Ok(R::decode(&mut bytes.as_ref())?)
}

// Usage
let result: SomeReturnType = state_call(
    &rpc,
    "ContractsApi_call",
    (origin, dest, value, gas_limit, storage_deposit_limit, input_data)
).await?;
```

### 4. Account Nonce Query

```rust
let account = dev::alice().public_key();
let nonce = rpc.system_account_next_index(&account).await?;
println!("Next nonce: {}", nonce);
```

### 5. Custom RPC Calls

```rust
use serde_json::json;

// Make raw RPC call
let result = rpc_client
    .request("custom_rpcMethod", rpc_params![param1, param2])
    .await?;
```

### 6. Block Subscription

```rust
// Subscribe to finalized blocks
let mut blocks_sub = api.blocks().subscribe_finalized().await?;

while let Some(block) = blocks_sub.next().await {
    let block = block?;
    println!("Block #{}: {:?}", block.number(), block.hash());
}

// Subscribe to best blocks (faster but may reorg)
let mut blocks_sub = api.blocks().subscribe_best().await?;
```

---

## pallet-contracts Integration

### Overview

Interacting with pallet-contracts requires:
1. **Extrinsic calls** for state changes (upload, instantiate, call)
2. **RPC calls** for read-only queries (dry-run calls)
3. **Event decoding** for contract events
4. **Weight/gas handling** (WeightsV2: refTime + proofSize)

### 1. Contract Deployment (upload_code)

```rust
use subxt::utils::H256;

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod runtime {}

// Read contract WASM
let wasm_code = std::fs::read("contract.wasm")?;

// Upload code (no gas limit needed for upload)
let upload_tx = runtime::tx()
    .contracts()
    .upload_code(
        wasm_code,
        None, // storage_deposit_limit
        runtime::runtime_types::pallet_contracts::wasm::Determinism::Enforced,
    );

let events = api
    .tx()
    .sign_and_submit_then_watch_default(&upload_tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;

// Extract code hash from events
let code_stored = events
    .find_first::<runtime::contracts::events::CodeStored>()?
    .ok_or("CodeStored event not found")?;

let code_hash = code_stored.code_hash;
println!("Code hash: {:?}", code_hash);
```

### 2. Contract Instantiation

```rust
// Prepare constructor data
let selector = [0x9b, 0xae, 0x9d, 0x5e]; // Constructor selector (4 bytes)
let mut constructor_data = selector.to_vec();
constructor_data.extend(/* encoded args */);

// Weight (V2): refTime + proofSize
let gas_limit = runtime::runtime_types::sp_weights::weight_v2::Weight {
    ref_time: 10_000_000_000, // 10B ref_time
    proof_size: 1_000_000,     // 1MB proof_size
};

let instantiate_tx = runtime::tx()
    .contracts()
    .instantiate(
        0,                          // value
        gas_limit,                  // gas_limit
        None,                       // storage_deposit_limit
        code_hash,                  // code_hash
        constructor_data,           // data
        vec![],                     // salt
    );

let events = api
    .tx()
    .sign_and_submit_then_watch_default(&instantiate_tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;

// Get contract address
let instantiated = events
    .find_first::<runtime::contracts::events::Instantiated>()?
    .ok_or("Instantiated event not found")?;

let contract_address = instantiated.contract;
println!("Contract address: {:?}", contract_address);
```

### 3. Contract Call (State Changing)

```rust
use sp_core::blake2_256;

// Generate call data
fn generate_call_data(selector_name: &str, args: &[impl Encode]) -> Vec<u8> {
    let selector = &blake2_256(selector_name.as_bytes())[0..4];
    let mut call_data = selector.to_vec();
    for arg in args {
        call_data.extend(arg.encode());
    }
    call_data
}

let call_data = generate_call_data("transfer", &[to_address, amount]);

let gas_limit = runtime::runtime_types::sp_weights::weight_v2::Weight {
    ref_time: 10_000_000_000,
    proof_size: 1_000_000,
};

let call_tx = runtime::tx()
    .contracts()
    .call(
        contract_address.into(),    // dest
        0,                          // value
        gas_limit,                  // gas_limit
        None,                       // storage_deposit_limit
        call_data,                  // data
    );

let events = api
    .tx()
    .sign_and_submit_then_watch_default(&call_tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await?;
```

### 4. Contract Query (Read-Only via RPC)

```rust
use parity_scale_codec::{Encode, Decode};
use subxt::backend::legacy::LegacyRpcMethods;

// Helper for state_call
async fn contracts_call<C: subxt::Config>(
    rpc: &LegacyRpcMethods<C>,
    origin: AccountId32,
    contract: AccountId32,
    value: u128,
    gas_limit: Weight,
    storage_deposit_limit: Option<u128>,
    input_data: Vec<u8>,
) -> Result<ContractExecResult> {
    let params = (
        origin,
        contract,
        value,
        gas_limit,
        storage_deposit_limit,
        input_data,
    );

    let encoded_params = params.encode();
    let bytes = rpc
        .state_call("ContractsApi_call", Some(&encoded_params), None)
        .await?;

    let result = ContractExecResult::decode(&mut bytes.as_ref())?;
    Ok(result)
}

// Usage
let call_data = generate_call_data("get_balance", &[account]);

let result = contracts_call(
    &rpc,
    signer.account_id().clone(),
    contract_address,
    0,
    gas_limit,
    None,
    call_data,
).await?;

// Decode result
if result.result.is_ok() {
    let data = result.result.ok().unwrap().data;
    // Decode based on contract ABI
    // Note: May need to skip first byte
    let balance = u128::decode(&mut &data[1..])?;
    println!("Balance: {}", balance);
}
```

### 5. Decoding Contract Events

```rust
// Contract events are wrapped in pallet-contracts' ContractEmitted event
let contract_events = events
    .find::<runtime::contracts::events::ContractEmitted>();

for event in contract_events {
    let event = event?;
    println!("Contract: {:?}", event.contract);
    println!("Data: {:?}", event.data);

    // Decode based on contract ABI
    // Topics help identify event type
    // event.topics[0] is event signature hash
}
```

### 6. Working with Contract Metadata (ink!)

```rust
use serde_json::Value;

// Load contract metadata (JSON)
let metadata_json = std::fs::read_to_string("contract.json")?;
let metadata: Value = serde_json::from_str(&metadata_json)?;

// Find message selector
fn find_message_selector(metadata: &Value, message_name: &str) -> Option<[u8; 4]> {
    let messages = metadata
        .get("spec")?
        .get("messages")?
        .as_array()?;

    for msg in messages {
        if msg.get("label")?.as_str()? == message_name {
            let selector = msg.get("selector")?.as_str()?;
            let bytes = hex::decode(selector.trim_start_matches("0x")).ok()?;
            return Some([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
    }
    None
}

// Generate call with metadata
let selector = find_message_selector(&metadata, "transfer")
    .ok_or("Message not found")?;

let mut call_data = selector.to_vec();
call_data.extend(to.encode());
call_data.extend(amount.encode());
```

### 7. Weight Estimation via Dry-Run

```rust
// Use dry-run to estimate weights
let dry_run_result = contracts_call(
    &rpc,
    origin,
    contract,
    value,
    Weight { ref_time: u64::MAX, proof_size: u64::MAX }, // Max for estimation
    storage_deposit_limit,
    call_data.clone(),
).await?;

if let Ok(exec_result) = dry_run_result.result {
    let required_gas = dry_run_result.gas_required;
    println!("Required gas: ref_time={}, proof_size={}",
        required_gas.ref_time,
        required_gas.proof_size
    );

    // Use with buffer for actual call
    let gas_limit = Weight {
        ref_time: required_gas.ref_time * 120 / 100, // +20%
        proof_size: required_gas.proof_size * 120 / 100,
    };
}
```

---

## Signing and Account Management

### 1. Using subxt_signer (Recommended)

```rust
use subxt_signer::sr25519;

// Development accounts
let alice = sr25519::dev::alice();
let bob = sr25519::dev::bob();

// From seed phrase
use subxt_signer::bip39::Mnemonic;

let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
let mnemonic = Mnemonic::parse(phrase)?;
let keypair = sr25519::Keypair::from_phrase(&mnemonic, None)?;

// From secret URI (SURI)
use subxt_signer::SecretUri;
use std::str::FromStr;

let uri = SecretUri::from_str("//Alice")?;
let keypair = sr25519::Keypair::from_uri(&uri)?;

// Complex URI with derivation and password
let uri = SecretUri::from_str(
    "vessel ladder alter error federal sibling chat ability sun glass valve picture/0/1///Password"
)?;
let keypair = sr25519::Keypair::from_uri(&uri)?;
```

### 2. Deriving Child Keys

```rust
use subxt_signer::DeriveJunction;

let root_keypair = sr25519::Keypair::from_phrase(&mnemonic, None)?;

// Equivalent to "//Alice/stash"
let derived = root_keypair.derive([
    DeriveJunction::hard("Alice"),
    DeriveJunction::soft("stash"),
]);

println!("Public key: {:?}", derived.public_key());
```

### 3. ECDSA and Ethereum Keys

```rust
use subxt_signer::ecdsa;

// ECDSA keypair
let ecdsa_keypair = ecdsa::Keypair::from_uri(&uri)?;

// Ethereum-compatible signing
let message = b"Hello Ethereum";
let signature = ecdsa_keypair.sign(message);
```

### 4. Account ID Conversion

```rust
use subxt::utils::AccountId32;

// From sr25519 public key
let public_key = keypair.public_key();
let account_id: AccountId32 = public_key.into();

// From SS58 address string
use std::str::FromStr;
let account_id = AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

// To bytes
let bytes: [u8; 32] = account_id.into();
```

### 5. Custom Signing (External Signer)

```rust
use subxt::tx::Signer;

// Implement custom signer
struct CustomSigner {
    account_id: AccountId32,
    // ... other fields
}

impl Signer<PolkadotConfig> for CustomSigner {
    fn account_id(&self) -> AccountId32 {
        self.account_id.clone()
    }

    fn address(&self) -> <PolkadotConfig as Config>::Address {
        self.account_id.clone().into()
    }

    fn sign(&self, payload: &[u8]) -> <PolkadotConfig as Config>::Signature {
        // Custom signing logic (e.g., hardware wallet)
        // ...
    }
}
```

### 6. Migrating from PairSigner (Pre-0.40)

```rust
// OLD (Pre-0.40 with substrate-compat)
use subxt::tx::PairSigner;
use sp_core::sr25519::Pair;

let pair = Pair::from_string("//Alice", None)?;
let signer = PairSigner::new(pair);

// NEW (0.40+)
use subxt_signer::sr25519;

let signer = sr25519::dev::alice();
// Or from URI
let signer = sr25519::Keypair::from_uri(&SecretUri::from_str("//Alice")?)?;
```

---

## Best Practices

### 1. Error Handling

```rust
use subxt::error::{Error, DispatchError};

// Handle transaction errors
match api.tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?
    .wait_for_finalized_success()
    .await
{
    Ok(events) => {
        println!("Success: {:?}", events);
    }
    Err(Error::Runtime(dispatch_error)) => {
        match dispatch_error {
            DispatchError::Module(module_err) => {
                println!("Pallet: {}", module_err.pallet);
                println!("Error: {}", module_err.error);
            }
            _ => {
                println!("Runtime error: {:?}", dispatch_error);
            }
        }
    }
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

### 2. Retry Logic with Reconnecting RPC

```rust
use subxt::backend::rpc::reconnecting_rpc_client::{
    RpcClient as ReconnectingRpcClient,
    ExponentialBackoff,
};
use std::time::Duration;

// Create reconnecting RPC client
let rpc_client = ReconnectingRpcClient::builder()
    .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
    .build("ws://127.0.0.1:9944".to_string())
    .await?;

let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client).await?;
```

### 3. Transaction Timeout

```rust
use tokio::time::{timeout, Duration};

let tx_future = api
    .tx()
    .sign_and_submit_then_watch_default(&tx, &signer);

match timeout(Duration::from_secs(30), tx_future).await {
    Ok(Ok(progress)) => {
        // Transaction submitted
        let events = progress.wait_for_finalized_success().await?;
    }
    Ok(Err(e)) => {
        println!("Transaction error: {:?}", e);
    }
    Err(_) => {
        println!("Transaction timeout");
    }
}
```

### 4. Batch Operations

```rust
// Batch multiple storage queries
let queries = vec![
    polkadot::storage().system().account(&alice),
    polkadot::storage().system().account(&bob),
    polkadot::storage().system().account(&charlie),
];

for query in queries {
    let info = api.storage().at_latest().await?.fetch(&query).await?;
    println!("Balance: {:?}", info.map(|i| i.data.free));
}
```

### 5. Version Compatibility

```toml
# Cargo.toml - Ensure compatible versions

[dependencies]
subxt = "0.44"
subxt-signer = "0.44"  # Match subxt version
sp-core = "42"         # Use compatible sp-core if needed
sp-runtime = "42"      # Match sp-core version

# Avoid version conflicts
# Use subxt::ext::sp_core instead of direct sp-core dependency
```

### 6. Metadata Validation

```rust
// Validate metadata compatibility
let metadata = api.metadata();
let version = metadata.metadata_version();

println!("Metadata version: {}", version);

// Check if specific pallet exists
if metadata.pallet_by_name("Contracts").is_some() {
    println!("Contracts pallet available");
}
```

### 7. Safe Nonce Management

```rust
// For sequential transactions, track nonce manually
let mut nonce = api
    .rpc()
    .system_account_next_index(&signer.account_id())
    .await?;

for i in 0..10 {
    let tx_params = Params::new()
        .nonce(nonce)
        .build();

    api.tx()
        .sign_and_submit(&tx, &signer, tx_params)
        .await?;

    nonce += 1; // Increment for next tx
}
```

### 8. Custom Configuration

```rust
use subxt::config::{Config, Header, Hasher, ExtrinsicParams};
use subxt::utils::{AccountId32, MultiAddress, MultiSignature};

// Define custom config
#[derive(Clone)]
pub enum MyChainConfig {}

impl Config for MyChainConfig {
    type Hash = H256;
    type AccountId = AccountId32;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
    type AssetId = u32;
}

// Use custom config
let api = OnlineClient::<MyChainConfig>::new().await?;
```

---

## Migration Guide from 0.35

### Step 1: Update Dependencies

```toml
# Before (0.35.x)
[dependencies]
subxt = "0.35"

# After (0.44.0)
[dependencies]
subxt = "0.44"
subxt-signer = "0.44"  # New for signing
```

### Step 2: Replace PairSigner

```rust
// Before (0.35 with substrate-compat)
use subxt::ext::sp_core::sr25519::Pair;
use subxt::tx::PairSigner;

let pair = Pair::from_string("//Alice", None).unwrap();
let signer = PairSigner::new(pair);

// After (0.44)
use subxt_signer::sr25519;

let signer = sr25519::dev::alice();
// Or from seed
let signer = sr25519::Keypair::from_uri(&uri)?;
```

### Step 3: Update Transaction Signatures

```rust
// Before (0.35)
api.tx()
    .sign_and_submit_then_watch(&tx, &signer, params)
    .await?;

// After (0.44) - Same API, different signer type
api.tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?;
```

### Step 4: Address vs AccountId

```rust
// Before (0.41)
// Some APIs accepted T::Address

// After (0.41+)
// APIs now use T::AccountId directly
let account_id = signer.account_id();
// No need to convert to Address
```

### Step 5: Update Offline Transaction Creation

```rust
// Before (0.40)
let partial_tx = api.tx()
    .create_signed_with_nonce(&tx, &signer, nonce, params)?;

// After (0.41+)
let params_with_nonce = params.nonce(nonce);
let partial_tx = api.tx()
    .create_partial_signed(&tx, &signer, params_with_nonce)?;
```

### Step 6: Config Trait Updates

```rust
// Before (0.41)
impl Config for MyConfig {
    type Hash = H256;
    type Hasher = BlakeTwo256;
    // ... other types
}

// After (0.42+)
impl Config for MyConfig {
    // Hash removed, Hasher changed to DynamicHasher256
    type Hasher = DynamicHasher256;
    // ... other types remain
}
```

### Step 7: Event Decoding Changes

```rust
// Before (0.42)
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod polkadot {}

// Codec traits auto-derived

// After (0.43+)
#[subxt::subxt(
    runtime_metadata_path = "./metadata.scale",
    derive_for_all_types = "Clone, Debug"  // Explicit derives
)]
pub mod polkadot {}
```

### Step 8: Update RPC Calls

```rust
// Before (legacy methods always available)
let rpc = api.rpc();

// After (0.41+) - Use dedicated crate
use subxt::backend::legacy::LegacyRpcMethods;

let rpc = LegacyRpcMethods::<Config>::new(rpc_client);
```

### Common Migration Pitfalls

1. **Version Mismatches**: Ensure all subxt-related crates are on the same version
2. **sp-core Conflicts**: Remove direct sp-core dependencies, use subxt::ext::sp_core
3. **Metadata Updates**: Regenerate metadata with `subxt metadata` CLI
4. **Transaction Mortality**: Default changed from immortal to 32 blocks
5. **Error Handling**: Runtime errors now wrapped in `Error::Runtime(DispatchError)`

---

## Appendix: Complete Example - Contract Interaction CLI

```rust
use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::backend::legacy::LegacyRpcMethods;
use subxt_signer::sr25519::{Keypair, dev};
use parity_scale_codec::{Encode, Decode};
use std::str::FromStr;

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Connect to node
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944").await?;
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(api.backend().rpc_client().clone());

    // 2. Create signer
    let signer = dev::alice();

    // 3. Upload contract
    let wasm = std::fs::read("contract.wasm")?;
    let upload_tx = runtime::tx()
        .contracts()
        .upload_code(
            wasm,
            None,
            runtime::runtime_types::pallet_contracts::wasm::Determinism::Enforced
        );

    println!("Uploading contract...");
    let events = api.tx()
        .sign_and_submit_then_watch_default(&upload_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let code_hash = events
        .find_first::<runtime::contracts::events::CodeStored>()?
        .unwrap()
        .code_hash;
    println!("Code hash: {:?}", code_hash);

    // 4. Instantiate contract
    let constructor_data = vec![0x9b, 0xae, 0x9d, 0x5e]; // new() selector
    let gas_limit = runtime::runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 10_000_000_000,
        proof_size: 1_000_000,
    };

    let instantiate_tx = runtime::tx()
        .contracts()
        .instantiate(
            0,
            gas_limit.clone(),
            None,
            code_hash,
            constructor_data,
            vec![],
        );

    println!("Instantiating contract...");
    let events = api.tx()
        .sign_and_submit_then_watch_default(&instantiate_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    let contract = events
        .find_first::<runtime::contracts::events::Instantiated>()?
        .unwrap()
        .contract;
    println!("Contract: {:?}", contract);

    // 5. Call contract (state change)
    let call_data = vec![0xdb, 0x20, 0xf9, 0xf5]; // flip() selector

    let call_tx = runtime::tx()
        .contracts()
        .call(
            contract.clone().into(),
            0,
            gas_limit.clone(),
            None,
            call_data.clone(),
        );

    println!("Calling contract...");
    api.tx()
        .sign_and_submit_then_watch_default(&call_tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    // 6. Query contract (read-only)
    let query_data = vec![0x2f, 0x86, 0x5b, 0xd9]; // get() selector

    let query_params = (
        signer.account_id().clone(),
        contract,
        0u128,
        gas_limit,
        None::<u128>,
        query_data,
    );

    let encoded = query_params.encode();
    let result = rpc.state_call("ContractsApi_call", Some(&encoded), None).await?;

    println!("Query result: {:?}", result);

    Ok(())
}
```

---

## References

- **Official Docs**: https://docs.rs/subxt/0.44.0/subxt/
- **GitHub**: https://github.com/paritytech/subxt
- **Examples**: https://github.com/paritytech/subxt/tree/master/subxt/examples
- **Book**: https://docs.rs/subxt/latest/subxt/book/
- **Polkadot Docs**: https://docs.polkadot.com/develop/toolkit/api-libraries/subxt/

---

**End of Report**
