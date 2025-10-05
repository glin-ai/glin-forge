---
title: Your First Contract
description: Build a complete smart contract from scratch with GLIN Forge
---

# Building Your First Smart Contract

In this tutorial, you'll build a complete smart contract from scratch - a simple counter contract. You'll learn the fundamentals of ink! smart contracts and GLIN Forge's development workflow.

## What We're Building

A counter contract with three functions:
- `get()` - Read the current count
- `increment()` - Increase the count by 1
- `reset()` - Reset the count to 0

This simple contract demonstrates:
- Contract storage
- Public query methods
- State-changing transactions
- Event emission

## Step 1: Create the Project

Instead of using a template, we'll initialize an empty project:

```bash
glin-forge init counter
cd counter
```

This creates a basic project structure with a default ERC20 template. We'll replace the code with our counter.

## Step 2: Write the Contract

Open `lib.rs` and replace its contents with:

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod counter {
    use ink::storage::Mapping;

    /// Storage for our counter contract
    #[ink(storage)]
    pub struct Counter {
        /// The current count value
        value: i32,
        /// Track who created the contract
        owner: AccountId,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct Incremented {
        #[ink(topic)]
        by: AccountId,
        new_value: i32,
    }

    #[ink(event)]
    pub struct Reset {
        #[ink(topic)]
        by: AccountId,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
    }

    /// Result type for contract methods
    pub type Result<T> = core::result::Result<T, Error>;

    impl Counter {
        /// Constructor: Initialize the counter
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                owner: Self::env().caller(),
            }
        }

        /// Constructor: Initialize with default value (0)
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        /// Get the current counter value (read-only)
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// Increment the counter by 1
        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;

            // Emit event
            self.env().emit_event(Incremented {
                by: self.env().caller(),
                new_value: self.value,
            });
        }

        /// Reset counter to 0 (owner only)
        #[ink(message)]
        pub fn reset(&mut self) -> Result<()> {
            let caller = self.env().caller();

            // Only owner can reset
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.value = 0;

            // Emit event
            self.env().emit_event(Reset { by: caller });

            Ok(())
        }

        /// Get the contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let counter = Counter::default();
            assert_eq!(counter.get(), 0);
        }

        #[ink::test]
        fn new_works() {
            let counter = Counter::new(42);
            assert_eq!(counter.get(), 42);
        }

        #[ink::test]
        fn increment_works() {
            let mut counter = Counter::new(0);
            counter.increment();
            assert_eq!(counter.get(), 1);
            counter.increment();
            assert_eq!(counter.get(), 2);
        }

        #[ink::test]
        fn reset_works() {
            let mut counter = Counter::new(10);
            assert!(counter.reset().is_ok());
            assert_eq!(counter.get(), 0);
        }

        #[ink::test]
        fn reset_fails_for_non_owner() {
            let mut counter = Counter::new(10);

            // Change caller to simulate different account
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Reset should fail
            assert_eq!(counter.reset(), Err(Error::NotOwner));
        }
    }
}
```

### Understanding the Code

Let's break down the key parts:

#### 1. Storage Definition

```rust
#[ink(storage)]
pub struct Counter {
    value: i32,
    owner: AccountId,
}
```

The storage struct holds the contract's state. It persists on the blockchain.

#### 2. Events

```rust
#[ink(event)]
pub struct Incremented {
    #[ink(topic)]
    by: AccountId,
    new_value: i32,
}
```

Events notify external listeners about state changes. The `#[ink(topic)]` makes fields searchable.

#### 3. Constructors

```rust
#[ink(constructor)]
pub fn new(init_value: i32) -> Self {
    Self {
        value: init_value,
        owner: Self::env().caller(),
    }
}
```

Constructors initialize the contract. You can have multiple constructors.

#### 4. Query Methods

```rust
#[ink(message)]
pub fn get(&self) -> i32 {
    self.value
}
```

Query methods use `&self` (read-only). They don't modify state and don't cost gas when called via RPC.

#### 5. Transaction Methods

```rust
#[ink(message)]
pub fn increment(&mut self) {
    self.value += 1;
    // ...
}
```

Transaction methods use `&mut self` (mutable). They modify state and cost gas.

## Step 3: Update Cargo.toml

Update your `Cargo.toml` to match:

```toml
[package]
name = "counter"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]

[dependencies]
ink = { version = "5.0", default-features = false }
scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2", default-features = false, features = ["derive"], optional = true }

[dev-dependencies]
ink_e2e = "5.0"

[lib]
path = "lib.rs"

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
e2e-tests = []
```

## Step 4: Build the Contract

Compile your contract:

```bash
glin-forge build --release --verify
```

### Expected Output

```
Building contract...
   Compiling counter v0.1.0
    Finished release [optimized] target(s) in 12.3s

✓ Contract built successfully!

Output files:
  WASM: ./target/ink/counter.wasm (15.2 KB)
  Metadata: ./target/ink/metadata.json
  Bundle: ./target/ink/counter.contract

Build verification:
  ✓ Code hash: 0x1234...5678
  ✓ Constructors: 2 (new, default)
  ✓ Messages: 4 (get, increment, reset, get_owner)
  ✓ Events: 2 (Incremented, Reset)
  ✓ WASM size: 15.2 KB (good)
```

## Step 5: Run Tests

Test your contract locally:

```bash
glin-forge test
```

### Expected Output

```
Running tests...

running 5 tests
test counter::tests::default_works ... ok
test counter::tests::new_works ... ok
test counter::tests::increment_works ... ok
test counter::tests::reset_works ... ok
test counter::tests::reset_fails_for_non_owner ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured
```

Great! All tests pass.

## Step 6: Deploy to Testnet

Deploy your counter contract:

```bash
glin-forge deploy \
  --network testnet \
  --account alice \
  --args "0"
```

We're using the `new` constructor with initial value `0`.

### Expected Output

```
Deploying contract...

Deployment details:
  Network: testnet
  Constructor: new
  Arguments: [0]
  Account: alice

Proceed with deployment? [y/N]: y

✓ Contract deployed successfully!

Contract info:
  Address: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Code Hash: 0x1234...5678
  Explorer: https://explorer-testnet.glin.network/contract/5FHneW46...
```

Save your contract address:

```bash
export CONTRACT_ADDR="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
```

## Step 7: Interact with the Contract

Now let's use our deployed counter!

### Get the Current Value

```bash
glin-forge query $CONTRACT_ADDR get
```

Output:
```
Result: 0
```

### Increment the Counter

```bash
glin-forge call $CONTRACT_ADDR increment \
  --account alice \
  --wait
```

Output:
```
✓ Transaction finalized!

Events:
  Incremented { by: 5GrwvaEF..., new_value: 1 }
```

### Check the New Value

```bash
glin-forge query $CONTRACT_ADDR get
```

Output:
```
Result: 1
```

Success! The counter increased.

### Increment Multiple Times

```bash
glin-forge call $CONTRACT_ADDR increment --account alice --yes
glin-forge call $CONTRACT_ADDR increment --account alice --yes
glin-forge query $CONTRACT_ADDR get
```

Output:
```
Result: 3
```

### Try to Reset (as owner)

```bash
glin-forge call $CONTRACT_ADDR reset \
  --account alice \
  --wait
```

Output:
```
✓ Transaction finalized!

Events:
  Reset { by: 5GrwvaEF... }
```

```bash
glin-forge query $CONTRACT_ADDR get
```

Output:
```
Result: 0
```

Reset worked because Alice is the owner!

### Try to Reset (as non-owner)

```bash
glin-forge call $CONTRACT_ADDR reset \
  --account bob \
  --wait
```

Output:
```
✗ Transaction failed!

Error: NotOwner
```

Perfect! The owner-only restriction works.

## Step 8: Watch Events

Monitor events in real-time:

```bash
glin-forge watch $CONTRACT_ADDR --follow
```

In another terminal, increment the counter:

```bash
glin-forge call $CONTRACT_ADDR increment --account alice --yes
```

You'll see the event appear in the watch terminal:

```
Watching events for contract 5FHneW46...

[Block #123458]
Event: Incremented
  by: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  new_value: 1
```

Press `Ctrl+C` to stop watching.

## Step 9: Generate TypeScript Types

Create types for frontend integration:

```bash
glin-forge typegen --output ./types --hooks
```

### Generated Files

**types/Counter.ts:**
```typescript
export interface CounterContractQuery {
  get: () => Promise<number>
  getOwner: () => Promise<string>
}

export interface CounterContractTx {
  increment: () => Promise<TxResult>
  reset: () => Promise<TxResult>
}

export interface CounterContract {
  query: CounterContractQuery
  tx: CounterContractTx
  address: string
}
```

**types/useCounter.ts:**
```typescript
import { useContract } from '@glin-ai/sdk-react'

export function useCounter(address: string, signer?: any) {
  const { contract, loading, error } = useContract({
    address,
    abi: metadata,
    signer,
  })

  return { contract, loading, error }
}
```

## Step 10: Verify on Explorer

Submit your contract for verification:

```bash
glin-forge verify $CONTRACT_ADDR \
  --network testnet
```

Output:
```
Verifying contract on explorer...

✓ Code hash verified on-chain
✓ Submitting verification request...
✓ Contract verified!

Verification ID: 12345
Status: Verified
Explorer: https://explorer-testnet.glin.network/contract/5FHneW46...

Users can now view your contract source code on the explorer.
```

## What You've Learned

Congratulations! You've learned how to:

- ✅ Write a complete ink! smart contract from scratch
- ✅ Define storage, events, and errors
- ✅ Create constructors, queries, and transactions
- ✅ Write and run unit tests
- ✅ Build and optimize contracts
- ✅ Deploy to testnet
- ✅ Query state and execute transactions
- ✅ Monitor contract events
- ✅ Generate TypeScript types
- ✅ Verify contracts on the explorer

## Next Steps

### Enhance Your Contract

Add more features to your counter:

```rust
// Add increment by custom amount
#[ink(message)]
pub fn increment_by(&mut self, amount: i32) {
    self.value += amount;
}

// Add decrement
#[ink(message)]
pub fn decrement(&mut self) {
    self.value -= 1;
}

// Add transfer ownership
#[ink(message)]
pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    self.owner = new_owner;
    Ok(())
}
```

### Build More Complex Contracts

- [ERC20 Token](../examples/erc20-token) - Fungible tokens
- [ERC721 NFT](../examples/nft-collection) - Non-fungible tokens
- [DAO Governance](../examples/dao-governance) - Decentralized voting

### Learn Advanced Topics

- [Gas Optimization](../guides/gas-optimization) - Reduce contract costs
- [Factory Pattern](../advanced/factory-pattern) - Deploy contracts from contracts
- [Gas Estimation](../advanced/gas-estimation) - Understanding gas costs

### Integrate with Frontend

- [Frontend Integration Guide](../guides/frontend-integration) - Build a dApp
- [React Hooks](../code-generation/react-hooks) - Use generated hooks
- [Type-safe Calls](../code-generation/typescript-types) - Frontend type safety

## Troubleshooting

### Build Fails

Check that you have the correct ink! version:

```bash
cargo tree | grep ink
```

Should show `ink v5.0` or later.

### Tests Fail

Ensure your imports are correct:

```rust
use super::*;
```

### Deployment Fails

Check your account has testnet tokens:

```bash
glin-forge balance alice --network testnet
```

Request testnet tokens from the [faucet](https://faucet-testnet.glin.network).

## Resources

- [ink! Documentation](https://use.ink/) - Official ink! docs
- [Substrate Docs](https://docs.substrate.io/) - Substrate framework
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [GLIN Forge CLI](../cli-reference/overview) - All commands

## Getting Help

- [Troubleshooting Guide](../troubleshooting/common-errors)
- [Discord Community](https://discord.gg/glin)
- [GitHub Issues](https://github.com/glin-ai/glin-forge/issues)
- [Forum](https://forum.glin.ai/)

Happy coding! 🎉
