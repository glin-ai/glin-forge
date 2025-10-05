---
title: glin-forge query
description: Read smart contract state without gas costs
---

# glin-forge query

Query smart contract state (read-only, no gas costs).

## Synopsis

```bash
glin-forge query <ADDRESS> <METHOD> [ARGS...] [OPTIONS]
```

## Description

The `query` command reads data from a smart contract without creating a transaction. It's free (no gas costs) and doesn't modify the blockchain state. Perfect for reading balances, checking ownership, or fetching contract data.

Queries use the RPC method `ContractsApi_call` which simulates the call locally and returns the result.

## Arguments

### `<ADDRESS>`

**Required.** The contract address to query.

- SS58 format: `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`
- Must be a deployed contract

### `<METHOD>`

**Required.** The method name to call.

- Must be a read-only message in the contract
- Case-sensitive
- Examples: `balanceOf`, `totalSupply`, `owner`

### `[ARGS...]`

Optional method arguments (space-separated).

- Number and types depend on the method signature
- Use spaces to separate multiple arguments
- Quote strings: `"MyString"`

## Options

### `-n, --network <NETWORK>`

Network to query from.

- **Default**: `testnet`
- **Options**: `testnet`, `mainnet`, `local`, or custom

### `-m, --metadata <PATH>`

Path to contract metadata JSON.

- **Auto-detected** from `./target/ink/metadata.json`
- Required if querying a contract you didn't build locally
- Can fetch from contract address (future feature)

### `--json`

Output result as JSON instead of human-readable format.

- Useful for scripts and automation
- Includes type information

### `-h, --help`

Print help information.

## Examples

### Query Without Arguments

```bash
# Get total token supply
glin-forge query 5FHneW46... totalSupply

# Get contract owner
glin-forge query 5FHneW46... owner
```

### Query With Single Argument

```bash
# Get balance of an account
glin-forge query 5FHneW46... balanceOf 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### Query With Multiple Arguments

```bash
# Get allowance (owner, spender)
glin-forge query 5FHneW46... allowance \
  5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  5FrLwHLX9Tfa2e8hXr8PUYvjCPYA5fBxQ6fTVVQRzBqN8kBy
```

### Query on Mainnet

```bash
glin-forge query 5FHneW46... totalSupply --network mainnet
```

### Query with Custom Metadata

```bash
glin-forge query 5FHneW46... balanceOf 5GrwvaEF... \
  --metadata ./deployed/metadata.json
```

### JSON Output

```bash
glin-forge query 5FHneW46... totalSupply --json
```

### Use in Scripts

```bash
# Save result to variable
TOTAL_SUPPLY=$(glin-forge query 5FHneW46... totalSupply)
echo "Total supply: $TOTAL_SUPPLY"

# Check balance and transfer if needed
BALANCE=$(glin-forge query 5FHneW46... balanceOf 5GrwvaEF...)
if [ "$BALANCE" -lt 1000 ]; then
    echo "Balance too low, transferring..."
    glin-forge call 5FHneW46... transfer 5GrwvaEF... 1000 --account alice
fi
```

## Output

### Standard Output

```
Querying contract...
✓ Connected to testnet (wss://testnet.glin.network)

Method: balanceOf
Arguments: [5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY]

Result: 1000000

Type: u128
```

### Minimal Output

For simple values, just the result:

```bash
$ glin-forge query 5FHneW46... totalSupply
Result: 1000000
```

### JSON Output

```bash
$ glin-forge query 5FHneW46... totalSupply --json
```

```json
{
  "result": "1000000",
  "type": "u128",
  "method": "totalSupply",
  "contract": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
}
```

### Boolean Results

```bash
$ glin-forge query 5FHneW46... isPaused
Result: false

Type: bool
```

### String Results

```bash
$ glin-forge query 5FHneW46... name
Result: "MyToken"

Type: String
```

### Array Results

```bash
$ glin-forge query 5FHneW46... getMembers
Result: [
  5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,
  5FrLwHLX9Tfa2e8hXr8PUYvjCPYA5fBxQ6fTVVQRzBqN8kBy,
  5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
]

Type: Vec<AccountId>
```

### Error Cases

**Contract not found:**
```
✗ Error: Contract not found at address
  Address: 5FHneW46...
  Network: testnet

  Verify the address and network are correct.
```

**Method not found:**
```
✗ Error: Method 'balanceof' not found
  Available methods: balanceOf, totalSupply, allowance, owner

  Note: Method names are case-sensitive
```

**Invalid arguments:**
```
✗ Error: Invalid arguments for method 'balanceOf'
  Expected: (AccountId)
  Provided: ()

  Usage: glin-forge query <address> balanceOf <account>
```

**Network connection failed:**
```
✗ Error: Failed to connect to network
  Network: testnet
  RPC: wss://testnet.glin.network

  Check your internet connection.
```

## Query vs Call

Understanding the difference:

| Feature | `query` | `call` |
|---------|---------|--------|
| **State changes** | No | Yes |
| **Gas cost** | Free | Costs gas |
| **Transaction** | No transaction | Creates transaction |
| **Speed** | Instant | Waits for block |
| **Account needed** | No | Yes |
| **Use case** | Read data | Modify state |

### When to Use query

```bash
# Read-only operations
glin-forge query 5FHneW46... balanceOf 5Account...
glin-forge query 5FHneW46... totalSupply
glin-forge query 5FHneW46... owner
glin-forge query 5FHneW46... isPaused
```

### When to Use call

```bash
# State-changing operations
glin-forge call 5FHneW46... transfer 5Recipient... 1000 --account alice
glin-forge call 5FHneW46... approve 5Spender... 5000 --account alice
glin-forge call 5FHneW46... mint 5Account... 100 --account alice
```

## Return Types

The query command handles various return types:

### Primitive Types

```rust
// Contract
#[ink(message)]
pub fn total_supply(&self) -> Balance { ... }
```

```bash
$ glin-forge query 5FHneW46... totalSupply
Result: 1000000
Type: u128
```

### Booleans

```rust
#[ink(message)]
pub fn is_paused(&self) -> bool { ... }
```

```bash
$ glin-forge query 5FHneW46... isPaused
Result: false
Type: bool
```

### Strings

```rust
#[ink(message)]
pub fn name(&self) -> String { ... }
```

```bash
$ glin-forge query 5FHneW46... name
Result: "MyToken"
Type: String
```

### Account IDs

```rust
#[ink(message)]
pub fn owner(&self) -> AccountId { ... }
```

```bash
$ glin-forge query 5FHneW46... owner
Result: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
Type: AccountId
```

### Options

```rust
#[ink(message)]
pub fn get_metadata(&self, id: u32) -> Option<String> { ... }
```

```bash
$ glin-forge query 5FHneW46... getMetadata 1
Result: Some("Token metadata")
Type: Option<String>

$ glin-forge query 5FHneW46... getMetadata 999
Result: None
Type: Option<String>
```

### Results

```rust
#[ink(message)]
pub fn try_transfer(&self, to: AccountId, amount: Balance) -> Result<(), Error> { ... }
```

```bash
$ glin-forge query 5FHneW46... tryTransfer 5Recipient... 100
Result: Ok(())

$ glin-forge query 5FHneW46... tryTransfer 5Recipient... 999999999
Result: Err(InsufficientBalance)
```

### Structs

```rust
#[derive(scale::Encode, scale::Decode)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimals: u8,
}

#[ink(message)]
pub fn get_info(&self) -> TokenInfo { ... }
```

```bash
$ glin-forge query 5FHneW46... getInfo
Result: TokenInfo {
  name: "MyToken",
  symbol: "MTK",
  decimals: 18
}
```

## Performance

Queries are fast because they:
- Don't create transactions
- Don't wait for block inclusion
- Run locally via RPC
- Don't require consensus

**Typical query time**: 10-100ms

## Metadata

### Auto-detection

GLIN Forge looks for metadata in:
1. `./target/ink/metadata.json`
2. `./metadata.json`
3. Current directory

### Specify Metadata

For contracts you didn't build locally:

```bash
# Download metadata from explorer or contract developer
curl https://example.com/contract/metadata.json -o metadata.json

# Query with custom metadata
glin-forge query 5FHneW46... balanceOf 5Account... \
  --metadata ./metadata.json
```

### Metadata Contents

The metadata contains:
- Method signatures
- Argument types
- Return types
- Method selectors

## Common Query Patterns

### ERC20 Queries

```bash
# Total supply
glin-forge query $CONTRACT totalSupply

# User balance
glin-forge query $CONTRACT balanceOf $USER_ADDRESS

# Allowance
glin-forge query $CONTRACT allowance $OWNER $SPENDER

# Token name
glin-forge query $CONTRACT name

# Token symbol
glin-forge query $CONTRACT symbol

# Decimals
glin-forge query $CONTRACT decimals
```

### ERC721 Queries

```bash
# Owner of token
glin-forge query $CONTRACT ownerOf 1

# Token balance
glin-forge query $CONTRACT balanceOf $USER_ADDRESS

# Token URI
glin-forge query $CONTRACT tokenUri 1

# Total supply
glin-forge query $CONTRACT totalSupply
```

### DAO Queries

```bash
# Get proposal
glin-forge query $CONTRACT getProposal 1

# Voting power
glin-forge query $CONTRACT getVotingPower $USER_ADDRESS

# Proposal count
glin-forge query $CONTRACT proposalCount

# Has voted
glin-forge query $CONTRACT hasVoted 1 $USER_ADDRESS
```

## Troubleshooting

### Metadata Not Found

**Problem:**
```
✗ Error: Metadata file not found
```

**Solution:**
1. Build the contract to generate metadata:
   ```bash
   glin-forge build --release
   ```

2. Or specify metadata path:
   ```bash
   glin-forge query 5FHneW46... method --metadata ./path/to/metadata.json
   ```

### Method Not Found

**Problem:**
```
✗ Error: Method 'balanceof' not found
```

**Solution:**
Method names are case-sensitive. Check the correct name:
```bash
# List available methods in metadata
cat target/ink/metadata.json | jq '.spec.messages[].label'
```

### Decoding Error

**Problem:**
```
✗ Error: Failed to decode result
```

**Solution:**
Metadata might be outdated. Rebuild:
```bash
glin-forge build --release
glin-forge query 5FHneW46... method
```

### Wrong Network

**Problem:**
Contract works on testnet but not mainnet.

**Solution:**
Specify the correct network:
```bash
glin-forge query 5FHneW46... method --network testnet
```

## Automation

### Shell Scripts

```bash
#!/bin/bash
CONTRACT="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

# Monitor total supply
while true; do
    SUPPLY=$(glin-forge query $CONTRACT totalSupply)
    echo "$(date): Total supply = $SUPPLY"
    sleep 60
done
```

### CI/CD Integration

```yaml
# GitHub Actions
- name: Query Contract State
  run: |
    BALANCE=$(glin-forge query $CONTRACT_ADDR balanceOf $ACCOUNT --json | jq -r '.result')
    echo "Balance: $BALANCE"

    if [ "$BALANCE" -lt 1000 ]; then
      echo "::error::Balance too low!"
      exit 1
    fi
```

### Monitoring

```bash
# Watch balance changes
watch -n 5 'glin-forge query 5FHneW46... balanceOf 5Account...'
```

## Related Commands

- [`call`](./call) - Execute state-changing transactions
- [`watch`](./watch) - Monitor contract events
- [`deploy`](../deployment/deploy) - Deploy contracts
- [`typegen`](../codegen/typegen) - Generate types for frontend queries

## See Also

- [Contract Interaction Guide](../../guides/contract-interaction) - Detailed interaction patterns
- [Frontend Integration](../../guides/frontend-integration) - Use queries in dApps
- [TypeScript Types](../../code-generation/typescript-types) - Type-safe queries
