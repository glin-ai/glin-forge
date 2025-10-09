# @glin-forge/testing

Testing utilities for glin-forge smart contracts. Provides a comprehensive set of helpers for writing robust tests, similar to Hardhat's testing utilities.

## Installation

```bash
npm install --save-dev @glin-forge/testing
```

## Features

- **Account Management**: Easy access to test accounts and account creation
- **Time Manipulation**: Control blockchain time and block production
- **Event Assertions**: Powerful event matching and assertion helpers
- **Balance Tracking**: Monitor and assert balance changes
- **Snapshots**: Take snapshots and revert blockchain state (dev chains only)

## Quick Start

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { setupTesting, getTestAccounts, expectEvent, balance } from '@glin-forge/testing';

// Initialize
const api = await ApiPromise.create({ provider: new WsProvider('ws://localhost:9944') });
const helper = await setupTesting(api);

// Get test accounts
const { alice, bob } = await getTestAccounts();

// Your contract instance
const contract = /* ... */;

// Execute transaction and check results
const result = await contract.transfer(bob.address, 1000);
expectEvent(result, 'contracts', 'ContractEmitted');

// Check balance changes
const bobBalance = await balance.get(bob);
console.log(`Bob's balance: ${balance.format(bobBalance)}`);
```

## Account Utilities

### Get Test Accounts

```typescript
import { getTestAccounts, getAccount } from '@glin-forge/testing';

// Get all common test accounts
const { alice, bob, charlie, dave, eve, ferdie } = await getTestAccounts();

// Get a specific account
const alice = await getAccount('alice');
```

### Create Random Accounts

```typescript
import { createRandomAccount, createRandomAccounts } from '@glin-forge/testing';

// Single random account
const randomAccount = await createRandomAccount();

// Multiple random accounts
const accounts = await createRandomAccounts(5);
```

### Create Account from Seed

```typescript
import { createAccountFromSeed, createAccountFromMnemonic } from '@glin-forge/testing';

const account = await createAccountFromSeed('//MyCustomSeed');
const account2 = await createAccountFromMnemonic('word1 word2 word3 ...');
```

## Time Manipulation

### Mine Blocks

```typescript
import { mineBlock, mineBlocks, time } from '@glin-forge/testing';

// Mine a single block
await mineBlock();

// Mine multiple blocks
await mineBlocks(10);

// Or use the time namespace
await time.mineBlocks(5);
```

### Increase Time

```typescript
import { increaseTime, fastForwardTo } from '@glin-forge/testing';

// Increase by 1 hour
await increaseTime(3600);

// Fast forward to specific timestamp
const futureTime = Date.now() + 86400000; // 1 day from now
await fastForwardTo(futureTime);
```

### Get Current Block Info

```typescript
import { getCurrentBlock, getCurrentTimestamp, getLatestBlock } from '@glin-forge/testing';

const blockNumber = await getCurrentBlock();
const timestamp = await getCurrentTimestamp();

const blockInfo = await getLatestBlock();
console.log(blockInfo); // { number, hash, timestamp }
```

## Event Assertions

### Basic Event Checks

```typescript
import { expectEvent, expectNoEvent, hasEvent } from '@glin-forge/testing';

// Assert event was emitted
expectEvent(result, 'contracts', 'ContractEmitted');

// Assert event was NOT emitted
expectNoEvent(result, 'system', 'ExtrinsicFailed');

// Check if event exists (boolean)
if (hasEvent(result, 'balances', 'Transfer')) {
  console.log('Transfer occurred');
}
```

### Multiple Event Assertions

```typescript
import { expectEvents } from '@glin-forge/testing';

// Assert multiple events in order
expectEvents(result, [
  { section: 'balances', method: 'Transfer' },
  { section: 'contracts', method: 'ContractEmitted' },
  { section: 'system', method: 'ExtrinsicSuccess' },
]);
```

### Transaction Success/Failure

```typescript
import { expectSuccess, expectFailure, expectRevert } from '@glin-forge/testing';

// Assert transaction succeeded
expectSuccess(result);

// Assert transaction failed
expectFailure(result);

// Assert transaction reverted with specific error
expectRevert(result, 'InsufficientBalance');
```

### Working with Event Data

```typescript
import { findEvent, getEventData, printEvents } from '@glin-forge/testing';

// Find specific event
const transferEvent = findEvent(result, 'balances', 'Transfer');

// Get event data
if (transferEvent) {
  const data = getEventData(transferEvent);
  console.log('Transfer data:', data);
}

// Print all events (useful for debugging)
printEvents(result);
```

## Balance Utilities

### Get Balances

```typescript
import { getBalance, getFreeBalance, getReservedBalance, getTotalBalance } from '@glin-forge/testing';

const balance = await getBalance(alice);
const free = await getFreeBalance(alice);
const reserved = await getReservedBalance(alice);
const total = await getTotalBalance(alice);
```

### Format and Parse Balances

```typescript
import { formatBalance, parseBalance } from '@glin-forge/testing';

// Format balance with decimals
const balance = 1234567890123456789n;
const formatted = formatBalance(balance, 18); // "1.234567890123456789"

// Parse human-readable balance to bigint
const parsed = parseBalance("1.5", 18); // 1500000000000000000n
```

### Balance Assertions

```typescript
import { expectBalance, expectMinBalance } from '@glin-forge/testing';

// Assert exact balance
await expectBalance(alice, 1000000000000000000n);

// Assert minimum balance
await expectMinBalance(alice, 500000000000000000n);
```

### Track Balance Changes

```typescript
import { createBalanceTracker, trackBalance } from '@glin-forge/testing';

// Using balance tracker
const tracker = createBalanceTracker();
await tracker.before(alice);

// Execute transaction
await contract.transfer(bob.address, 1000);

// Check balance change
await tracker.expectDecrease(alice);
const change = await tracker.after(alice);
console.log(`Alice's balance changed by: ${change}`);

// Or use the shorthand
const { result, change } = await trackBalance(alice, async () => {
  return await contract.transfer(bob.address, 1000);
});
```

## Snapshot Utilities

**Note**: Snapshot/revert functionality requires a development chain with time-travel capabilities. For production chains, use fresh chain state for each test.

### Take and Revert Snapshots

```typescript
import { takeSnapshot, revertToSnapshot } from '@glin-forge/testing';

// Take a snapshot
const snapshotId = await takeSnapshot();

// Do some transactions
await contract.someMethod();

// Revert to snapshot
await revertToSnapshot(snapshotId);
```

### Using Fixtures

```typescript
import { fixture } from '@glin-forge/testing';

// Automatically snapshot and revert
await fixture(async () => {
  // Your test code here
  await contract.someMethod();
  // Will automatically revert after this function completes
});
```

### Snapshot Context

```typescript
import { createSnapshotContext } from '@glin-forge/testing';

const ctx = createSnapshotContext();

// Push snapshots onto stack
await ctx.push();
await contract.method1();

await ctx.push();
await contract.method2();

// Pop and revert
await ctx.pop(); // Reverts method2
await ctx.pop(); // Reverts method1
```

## Complete Example

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import {
  setupTesting,
  getTestAccounts,
  expectEvent,
  expectSuccess,
  createBalanceTracker,
  time,
} from '@glin-forge/testing';

describe('MyContract', () => {
  let api: ApiPromise;
  let contract: any;
  let alice: any;
  let bob: any;

  beforeAll(async () => {
    // Initialize API
    api = await ApiPromise.create({
      provider: new WsProvider('ws://localhost:9944'),
    });

    // Setup testing utilities
    await setupTesting(api);

    // Get test accounts
    ({ alice, bob } = await getTestAccounts());

    // Deploy contract
    contract = /* ... deploy your contract ... */;
  });

  it('should transfer tokens', async () => {
    const tracker = createBalanceTracker();
    await tracker.before(bob);

    const result = await contract.transfer(bob.address, 1000);

    // Assert success
    expectSuccess(result);

    // Assert events
    expectEvent(result, 'contracts', 'ContractEmitted');

    // Assert balance change
    await tracker.expectChange(bob, 1000n);
  });

  it('should respect time locks', async () => {
    // Lock tokens for 1 hour
    await contract.lockTokens(1000, 3600);

    // Try to unlock immediately (should fail)
    const result1 = await contract.unlockTokens();
    expectFailure(result1);

    // Fast forward 1 hour
    await time.increaseTime(3600);

    // Try again (should succeed)
    const result2 = await contract.unlockTokens();
    expectSuccess(result2);
  });

  afterAll(async () => {
    await api.disconnect();
  });
});
```

## API Reference

### Namespaces

All utilities are also available through namespaces:

```typescript
import { time, events, balance, snapshot } from '@glin-forge/testing';

// Time utilities
await time.mineBlocks(5);
await time.increaseTime(3600);

// Event utilities
events.expect(result, 'contracts', 'ContractEmitted');
events.print(result);

// Balance utilities
const bal = await balance.get(alice);
await balance.expect(bob, expectedBalance);

// Snapshot utilities
const id = await snapshot.take();
await snapshot.revert(id);
```

### TestHelper Class

For a unified testing interface:

```typescript
import { createTestHelper } from '@glin-forge/testing';

const helper = createTestHelper(api);
await helper.init();

// Access all utilities
const { alice, bob } = await helper.getAccounts();
await helper.time.mineBlocks(5);
helper.events.expect(result, 'system', 'ExtrinsicSuccess');
```

## TypeScript Support

This package is written in TypeScript and includes full type definitions.

## License

MIT
