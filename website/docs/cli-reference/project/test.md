---
title: glin-forge test
description: Run smart contract tests
---

# glin-forge test

Run unit tests and end-to-end tests for your ink! smart contract.

## Synopsis

```bash
glin-forge test [OPTIONS]
```

## Description

The `test` command runs your contract's test suite. It supports both unit tests (marked with `#[ink::test]`) and end-to-end tests that interact with a real blockchain node.

## Options

### `-p, --path <PATH>`

Path to the contract project.

- **Default**: `.` (current directory)
- Can be relative or absolute path

### `--e2e`

Run end-to-end tests instead of unit tests.

- Requires running substrate-contracts-node
- Uses `e2e-tests` feature flag
- Tests contract on actual blockchain

### `--test <NAME>`

Filter tests by name pattern.

- Runs only tests matching the pattern
- Case-sensitive substring match
- Works with both unit and E2E tests

**Example:**
```bash
--test transfer        # Runs transfer_works, transfer_fails_*, etc.
--test "balance"       # Runs all tests with "balance" in name
```

### `--nocapture`

Show output from successful tests.

- By default, only failed test output is shown
- Useful for debugging
- Shows `println!` and `debug!` output

## Examples

### Run All Unit Tests

```bash
glin-forge test
```

### Run Tests in Specific Directory

```bash
glin-forge test --path ./contracts/my-token
```

### Run End-to-End Tests

```bash
# Start local node first
substrate-contracts-node --dev

# In another terminal
glin-forge test --e2e
```

### Run Specific Tests

```bash
# Run only transfer tests
glin-forge test --test transfer

# Run a single test
glin-forge test --test transfer_works
```

### Show All Output

```bash
glin-forge test --nocapture
```

### Combine Options

```bash
glin-forge test --e2e --test integration --nocapture
```

## Output

### Successful Test Run

```
Running tests for my_token...

running 8 tests
test my_token::tests::constructor_works ... ok
test my_token::tests::transfer_works ... ok
test my_token::tests::transfer_fails_insufficient_balance ... ok
test my_token::tests::approve_works ... ok
test my_token::tests::transfer_from_works ... ok
test my_token::tests::transfer_from_fails_insufficient_allowance ... ok
test my_token::tests::balance_of_works ... ok
test my_token::tests::total_supply_works ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

### Failed Test

```
Running tests for my_token...

running 8 tests
test my_token::tests::constructor_works ... ok
test my_token::tests::transfer_works ... FAILED
test my_token::tests::transfer_fails_insufficient_balance ... ok

failures:

---- my_token::tests::transfer_works stdout ----
thread 'my_token::tests::transfer_works' panicked at 'assertion failed: `(left == right)`
  left: `100`,
 right: `0`', lib.rs:156:9

failures:
    my_token::tests::transfer_works

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```

### E2E Test Output

```
Running E2E tests for my_token...

running 3 tests
test e2e::tests::deploy_and_transfer ... ok
test e2e::tests::approve_and_transfer_from ... ok
test e2e::tests::events_emitted ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.42s
```

## Unit Tests

### Writing Unit Tests

Unit tests use the `#[ink::test]` attribute:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn constructor_works() {
        let contract = MyToken::new(1000);
        assert_eq!(contract.total_supply(), 1000);
    }

    #[ink::test]
    fn transfer_works() {
        let mut contract = MyToken::new(1000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        assert!(contract.transfer(accounts.bob, 100).is_ok());
        assert_eq!(contract.balance_of(accounts.bob), 100);
        assert_eq!(contract.balance_of(accounts.alice), 900);
    }

    #[ink::test]
    fn transfer_fails_insufficient_balance() {
        let mut contract = MyToken::new(1000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        assert_eq!(
            contract.transfer(accounts.bob, 2000),
            Err(Error::InsufficientBalance)
        );
    }
}
```

### Test Accounts

ink! provides default test accounts:

```rust
let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

// Available accounts:
accounts.alice  // Default caller
accounts.bob
accounts.charlie
accounts.django
accounts.eve
accounts.frank
```

### Testing Events

```rust
#[ink::test]
fn transfer_emits_event() {
    let mut contract = MyToken::new(1000);
    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

    contract.transfer(accounts.bob, 100).unwrap();

    let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
    assert_eq!(emitted_events.len(), 2); // Constructor + transfer
}
```

## End-to-End Tests

### Writing E2E Tests

E2E tests interact with a real node:

```rust
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::build_message;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn deploy_and_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Deploy contract
        let constructor = MyTokenRef::new(1000);
        let contract_account_id = client
            .instantiate("my_token", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Call transfer
        let transfer = build_message::<MyTokenRef>(contract_account_id.clone())
            .call(|contract| contract.transfer(bob_account_id(), 100));

        client
            .call(&ink_e2e::alice(), transfer, 0, None)
            .await
            .expect("transfer failed");

        // Query balance
        let balance = build_message::<MyTokenRef>(contract_account_id.clone())
            .call(|contract| contract.balance_of(bob_account_id()));

        let balance_result = client
            .call_dry_run(&ink_e2e::alice(), &balance, 0, None)
            .await;

        assert_eq!(balance_result.return_value(), 100);

        Ok(())
    }
}
```

### Running E2E Tests

E2E tests require a running node:

```bash
# Terminal 1: Start node
substrate-contracts-node --dev

# Terminal 2: Run tests
glin-forge test --e2e
```

### E2E Test Configuration

In `Cargo.toml`:

```toml
[dev-dependencies]
ink_e2e = "5.0"

[features]
e2e-tests = []
```

## Test Output Control

### Quiet Mode (Default)

Only shows test results:
```bash
glin-forge test
```

### Verbose Mode

Shows all output including successful tests:
```bash
glin-forge test --nocapture
```

### Filtered Tests

Run subset of tests:
```bash
# Run only transfer tests
glin-forge test --test transfer

# Run only one test
glin-forge test --test constructor_works
```

## Best Practices

### Test Coverage

Ensure comprehensive coverage:

- ✅ Constructor initialization
- ✅ Happy path scenarios
- ✅ Error conditions
- ✅ Edge cases (zero values, max values)
- ✅ Access control
- ✅ Events emission
- ✅ State changes

### Test Organization

Group related tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod constructor {
        use super::*;

        #[ink::test]
        fn works_with_valid_supply() { }

        #[ink::test]
        fn fails_with_zero_supply() { }
    }

    mod transfer {
        use super::*;

        #[ink::test]
        fn works() { }

        #[ink::test]
        fn fails_insufficient_balance() { }
    }
}
```

### Naming Conventions

Use descriptive test names:

✅ Good:
- `transfer_works`
- `transfer_fails_insufficient_balance`
- `approve_updates_allowance`

❌ Avoid:
- `test1`
- `it_works`
- `transfer` (not descriptive enough)

## Troubleshooting

### Tests Not Found

**Problem:**
```
no tests to run
```

**Solution:**
Ensure tests are marked with `#[ink::test]`:
```rust
#[ink::test]  // Don't forget this!
fn my_test() {
    // ...
}
```

### E2E Tests Fail

**Problem:**
```
✗ Error: Failed to connect to node
```

**Solution:**
Start the substrate-contracts-node:
```bash
# Install if needed
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git

# Run node
substrate-contracts-node --dev
```

### Compilation Errors

**Problem:**
```
error: cannot find attribute `ink` in this scope
```

**Solution:**
Add ink! dependency to `Cargo.toml`:
```toml
[dependencies]
ink = { version = "5.0", default-features = false }
```

### Test Timeout

**Problem:**
E2E tests timeout.

**Solution:**
Increase timeout or check node connectivity:
```rust
#[ink_e2e::test(timeout = 60000)]  // 60 seconds
async fn long_running_test() { }
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install cargo-contract
        run: cargo install cargo-contract --force

      - name: Run tests
        run: glin-forge test
```

### Test Before Deploy

Always test before deploying:

```bash
# Build and test
glin-forge build --release
glin-forge test

# If tests pass, deploy
if [ $? -eq 0 ]; then
  glin-forge deploy --network testnet --account alice
fi
```

## Related Commands

- [`build`](./build) - Build before testing
- [`deploy`](../deployment/deploy) - Deploy after tests pass
- [`new`](./new) - Create project with test template

## See Also

- [Testing Strategies Guide](../../guides/testing-strategies) - Advanced testing techniques
- [ink! Testing Documentation](https://use.ink/basics/contract-testing)
- [E2E Testing Guide](https://use.ink/basics/contract-testing/e2e)
