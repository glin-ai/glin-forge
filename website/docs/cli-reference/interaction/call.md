---
title: glin-forge call
description: Execute smart contract transactions
---

# glin-forge call

Execute a state-changing contract transaction.

## Synopsis

```bash
glin-forge call <ADDRESS> <METHOD> [ARGS...] [OPTIONS]
```

## Description

The `call` command executes a state-changing method on a smart contract. Unlike `query`, this creates a transaction on the blockchain, costs gas, and modifies the contract state.

## Arguments

### `<ADDRESS>`

**Required.** The contract address.

- SS58 format: `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`

### `<METHOD>`

**Required.** The method name to call.

- Must be a mutable message in the contract
- Case-sensitive
- Examples: `transfer`, `approve`, `mint`

### `[ARGS...]`

Method arguments (space-separated).

- Types depend on method signature
- Quote strings: `"MyString"`
- Use addresses for AccountId: `5GrwvaEF...`

## Options

### `-n, --network <NETWORK>`

Target network.

- **Default**: `testnet`

### `-a, --account <ACCOUNT>`

**Required.** Account to sign the transaction.

- Development accounts: `alice`, `bob`, etc.
- Or custom account name

### `-v, --value <VALUE>`

Value to send with the call (in GLIN).

- **Default**: `0`
- Required for payable methods

### `-m, --metadata <PATH>`

Path to contract metadata.

- **Auto-detected** from `./target/ink/metadata.json`

### `-g, --gas-limit <GAS>`

Override automatic gas estimation.

- Format: `refTime` or `refTime,proofSize`

### `-y, --yes`

Skip confirmation prompt.

### `--wait`

Wait for transaction finalization.

- Shows final status and events
- Exits after block inclusion

## Examples

### Simple Transfer

```bash
glin-forge call 5FHneW46... transfer \
  5FrLwHLX9Tfa2e8hXr8PUYvjCPYA5fBxQ6fTVVQRzBqN8kBy \
  1000 \
  --account alice
```

### With Value

Send GLIN with the call:

```bash
glin-forge call 5FHneW46... mint \
  --account alice \
  --value 10 \
  --wait
```

### Approve Tokens

```bash
glin-forge call 5FHneW46... approve \
  5FrLwHLX9Tfa2e8hXr8PUYvjCPYA5fBxQ6fTVVQRzBqN8kBy \
  5000 \
  --account alice \
  --wait
```

### Skip Confirmation

```bash
glin-forge call 5FHneW46... increment \
  --account alice \
  --yes
```

### Custom Gas

```bash
glin-forge call 5FHneW46... complexMethod \
  arg1 arg2 \
  --account alice \
  --gas-limit 10000000000,3000000
```

## Output

### Interactive Prompt

```
Calling contract method: transfer

Contract: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
Method: transfer
Arguments: [5FrLwHLX..., 1000]
Value: 0 GLIN

Account: alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)
Balance: 100.0 GLIN

Gas estimation:
  refTime: 1,500,000,000
  proofSize: 500,000

Estimated cost: ~0.002 GLIN

Proceed? [y/N]:
```

### Success (without --wait)

```
Proceed? [y/N]: y

✓ Connected to testnet
✓ Transaction submitted!

Transaction hash: 0xabcd...ef01

The transaction has been submitted to the network.
Use --wait to see the final result.
```

### Success (with --wait)

```
Proceed? [y/N]: y

✓ Connected to testnet
✓ Transaction submitted!

Hash: 0xabcd...ef01

⠋ Waiting for finalization... (block #123456)

✓ Transaction finalized!

Block: #123456
Status: Success

Events emitted:
  Transfer {
    from: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,
    to: 5FrLwHLX9Tfa2e8hXr8PUYvjCPYA5fBxQ6fTVVQRzBqN8kBy,
    value: 1000
  }

Gas used:
  refTime: 1,234,567,890
  proofSize: 456,789

Explorer: https://explorer-testnet.glin.network/extrinsic/0xabcd...
```

### Error Cases

**Insufficient balance:**
```
✗ Error: Insufficient balance
  Required: ~5.0 GLIN
  Available: 0.5 GLIN
```

**Transaction failed:**
```
✗ Transaction failed!

Block: #123457
Status: Failed
Reason: Module error: contracts::ContractTrapped

The contract execution reverted.
Check your contract logic and arguments.
```

## Gas Estimation

### Automatic

Default gas limits:
- refTime: 1,500,000,000
- proofSize: 500,000

### Manual Override

```bash
glin-forge call 5FHneW46... method \
  --account alice \
  --gas-limit 3000000000,1000000
```

### Gas Tips

```
Gas Estimation:
  → refTime: 1,500,000,000
  → proofSize: 500,000
  ℹ Tip: Gas will be refunded if unused
```

## Method Arguments

### Types

| Rust Type | CLI Format | Example |
|-----------|------------|---------|
| `Balance` | Number | `1000` |
| `String` | Quoted | `"MyToken"` |
| `bool` | true/false | `true` |
| `AccountId` | SS58 | `5GrwvaEF...` |
| `Option<T>` | Value or `null` | `100` or `null` |

### Multiple Arguments

Space-separated:

```bash
glin-forge call $CONTRACT transferFrom \
  5GrwvaEF... \
  5FrLwHLX... \
  1000 \
  --account alice
```

## Return Values

Most mutations don't return values directly. Check:
1. **Events** - Emitted by the contract
2. **State** - Query after calling
3. **Status** - Success or failure

### Check Result

```bash
# Call mutation
glin-forge call $CONTRACT transfer $RECIPIENT 1000 --account alice --wait

# Query new balance
glin-forge query $CONTRACT balanceOf $RECIPIENT
```

## Events

With `--wait`, events are displayed:

```
Events emitted:
  Transfer {
    from: 5GrwvaEF...,
    to: 5FrLwHLX...,
    value: 1000
  }
  Approval {
    owner: 5GrwvaEF...,
    spender: 5FrLwHLX...,
    value: 5000
  }
```

## Transaction Finalization

### Without --wait

Transaction is submitted but doesn't wait for finalization:

```bash
glin-forge call $CONTRACT method --account alice
```

Returns immediately with transaction hash.

### With --wait

Waits for block inclusion and finalization:

```bash
glin-forge call $CONTRACT method --account alice --wait
```

Takes 6-12 seconds typically.

## Common Patterns

### ERC20 Calls

```bash
# Transfer tokens
glin-forge call $CONTRACT transfer $RECIPIENT 1000 \
  --account alice --wait

# Approve spender
glin-forge call $CONTRACT approve $SPENDER 5000 \
  --account alice --wait

# Transfer from (requires approval)
glin-forge call $CONTRACT transferFrom $FROM $TO 500 \
  --account spender --wait
```

### ERC721 Calls

```bash
# Mint NFT
glin-forge call $CONTRACT mint $RECIPIENT 1 \
  --account owner --wait

# Transfer NFT
glin-forge call $CONTRACT transferFrom $FROM $TO 1 \
  --account from --wait

# Approve NFT
glin-forge call $CONTRACT approve $SPENDER 1 \
  --account owner --wait
```

### DAO Calls

```bash
# Create proposal
glin-forge call $CONTRACT createProposal "Proposal description" \
  --account member --wait

# Vote
glin-forge call $CONTRACT vote 1 true \
  --account member --wait

# Execute proposal
glin-forge call $CONTRACT execute 1 \
  --account anyone --wait
```

## Troubleshooting

### Transaction Reverted

**Problem:**
```
✗ Transaction failed!
  Reason: contracts::ContractTrapped
```

**Causes:**
- Invalid arguments
- Business logic rejected (e.g., insufficient balance)
- Panic in contract code
- Arithmetic overflow

**Solution:**
1. Check arguments match expected types
2. Query contract state first
3. Review contract logic
4. Test locally

### Out of Gas

**Problem:**
```
✗ Transaction failed!
  Reason: contracts::OutOfGas
```

**Solution:**
Increase gas limit:
```bash
glin-forge call $CONTRACT method \
  --account alice \
  --gas-limit 5000000000,2000000
```

### Insufficient Balance

**Problem:**
```
✗ Error: Insufficient balance
```

**Solution:**
1. Check balance:
   ```bash
   glin-forge balance alice --network testnet
   ```

2. Request testnet tokens:
   https://faucet-testnet.glin.network

### Method Not Found

**Problem:**
```
✗ Error: Method 'tranfer' not found
```

**Solution:**
Check spelling (case-sensitive):
```bash
glin-forge call $CONTRACT transfer ...  # Correct
```

## Scripts and Automation

### Batch Operations

```bash
#!/bin/bash
CONTRACT="5FHneW46..."
RECIPIENTS=("5GrwvaEF..." "5FrLwHLX..." "5DAAnrj7...")

for recipient in "${RECIPIENTS[@]}"; do
    echo "Transferring to $recipient..."
    glin-forge call $CONTRACT transfer $recipient 100 \
      --account alice \
      --yes \
      --wait
    sleep 1
done
```

### Error Handling

```bash
if glin-forge call $CONTRACT method --account alice --yes --wait; then
    echo "Transaction successful"
else
    echo "Transaction failed"
    exit 1
fi
```

## CI/CD Integration

```yaml
- name: Execute Contract Call
  run: |
    glin-forge call $CONTRACT_ADDR updateConfig "new-value" \
      --account ci-deployer \
      --network testnet \
      --yes \
      --wait \
      > call_output.txt

    # Check for success
    if grep -q "Success" call_output.txt; then
      echo "Contract updated successfully"
    else
      echo "Contract call failed"
      exit 1
    fi
```

## Related Commands

- [`query`](./query) - Read contract state (no gas)
- [`deploy`](../deployment/deploy) - Deploy contracts
- [`watch`](./watch) - Monitor contract events

## See Also

- [Contract Interaction Guide](../../guides/contract-interaction)
- [Gas Optimization](../../guides/gas-optimization)
- [Event Handling](../../guides/event-handling)
