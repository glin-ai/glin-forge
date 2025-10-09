/**
 * @glin-forge/testing
 *
 * Testing utilities for glin-forge smart contracts.
 * Provides helpers for accounts, time manipulation, events, balances, and snapshots.
 *
 * @example
 * ```typescript
 * import { getTestAccounts, expectEvent, balance } from '@glin-forge/testing';
 *
 * const { alice, bob } = await getTestAccounts();
 * const aliceBalance = await balance.get(alice);
 *
 * // Execute transaction
 * const result = await contract.transfer(bob.address, 1000);
 *
 * // Assert events and balance changes
 * expectEvent(result, 'contracts', 'ContractEmitted');
 * await balance.expect(bob, aliceBalance - 1000n);
 * ```
 */

import type { ApiPromise } from '@polkadot/api';

// Account utilities
export {
  initAccounts,
  getAccount,
  getAccounts,
  getTestAccounts,
  createRandomAccount,
  createRandomAccounts,
  createAccountFromSeed,
  createAccountFromMnemonic,
  getAddress,
  getPublicKey,
  clearAccountCache,
  AccountImpersonator,
  createImpersonator,
} from './accounts';

// Time utilities
export {
  initTime,
  getCurrentBlock,
  getCurrentTimestamp,
  mineBlock,
  mineBlocks,
  increaseTime,
  setNextBlockTimestamp,
  fastForwardTo,
  waitForBlocks,
  getLatestBlock,
  setAutoMine,
  setBlockTime,
  time,
  type TimeConfig,
} from './time';

// Event utilities
export {
  parseEvents,
  findEvent,
  findEvents,
  hasEvent,
  expectEvent,
  expectNoEvent,
  expectEvents,
  getEventData,
  expectContractEvent,
  extractError,
  expectSuccess,
  expectFailure,
  expectRevert,
  printEvents,
  events,
  type EventMatcher,
} from './events';

// Balance utilities
export {
  initBalance,
  getBalance,
  getFreeBalance,
  getReservedBalance,
  getTotalBalance,
  formatBalance,
  parseBalance,
  hasBalance,
  expectBalance,
  expectMinBalance,
  BalanceTracker,
  createBalanceTracker,
  trackBalance,
  balance,
} from './balance';

// Snapshot utilities
export {
  initSnapshot,
  takeSnapshot,
  revertToSnapshot,
  getSnapshot,
  listSnapshots,
  deleteSnapshot,
  clearSnapshots,
  getLatestSnapshot,
  fixture,
  SnapshotContext,
  createSnapshotContext,
  DatabaseSnapshot,
  snapshot,
  type SnapshotId,
} from './snapshot';

/**
 * Initialize all testing utilities at once
 */
export async function initTesting(api: ApiPromise): Promise<void> {
  const { initAccounts } = await import('./accounts');
  const { initTime } = await import('./time');
  const { initBalance } = await import('./balance');
  const { initSnapshot } = await import('./snapshot');

  await initAccounts();
  initTime(api);
  initBalance(api);
  initSnapshot(api);
}

/**
 * Test helper class that combines all utilities
 */
export class TestHelper {
  constructor(private api: ApiPromise) {}

  /**
   * Initialize all utilities
   */
  async init(): Promise<void> {
    await initTesting(this.api);
  }

  /**
   * Get the API instance
   */
  getApi(): ApiPromise {
    return this.api;
  }

  /**
   * Quick access to common test accounts
   */
  async getAccounts() {
    const { getTestAccounts } = await import('./accounts');
    return getTestAccounts();
  }

  /**
   * Quick access to time utilities
   */
  get time() {
    const { time } = require('./time');
    return time;
  }

  /**
   * Quick access to event utilities
   */
  get events() {
    const { events } = require('./events');
    return events;
  }

  /**
   * Quick access to balance utilities
   */
  get balance() {
    const { balance } = require('./balance');
    return balance;
  }

  /**
   * Quick access to snapshot utilities
   */
  get snapshot() {
    const { snapshot } = require('./snapshot');
    return snapshot;
  }
}

/**
 * Create a test helper instance
 */
export function createTestHelper(api: ApiPromise): TestHelper {
  return new TestHelper(api);
}

/**
 * Common test setup function
 */
export async function setupTesting(api: ApiPromise): Promise<TestHelper> {
  const helper = createTestHelper(api);
  await helper.init();
  return helper;
}
